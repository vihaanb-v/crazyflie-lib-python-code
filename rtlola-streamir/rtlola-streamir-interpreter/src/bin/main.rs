use std::fs::File;
use std::io::{stderr, Stderr};
use std::path::PathBuf;
use std::ptr;
use std::time::{Duration, Instant};

use anyhow::Context;
use clap::{Parser, ValueEnum};
use rtlola_streamir::ir::StreamReference;
use rtlola_streamir::{parse, Handler, ParseError, ParserConfig};
use rtlola_streamir_interpreter::csv::{CsvEventSource, CsvVerdictSink};
use rtlola_streamir_interpreter::Monitor;

#[derive(Parser, Debug, Clone)]
struct Args {
    spec: PathBuf,
    trace: PathBuf,
    #[arg(short, long, value_enum, default_value_t=Verbosity::Outputs)]
    verbosity: Verbosity,
    #[arg(long, conflicts_with = "verbosity")]
    output_streams: Vec<String>,
    #[arg(short, long)]
    optimize: bool,
    #[arg(long)]
    benchmark: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Verbosity {
    Silent,
    Trigger,
    Outputs,
    Streams,
}

fn build(
    config: &ParserConfig,
    trace: PathBuf,
    verbosity: Verbosity,
    output_streams: Vec<String>,
    optimize: bool,
) -> anyhow::Result<(Monitor, CsvEventSource<File>, CsvVerdictSink<Stderr>)> {
    let streamir = parse(config).context("parsing spec")?;
    let csv_source = CsvEventSource::new(File::open(trace)?, &streamir);

    let csv_fields: Vec<StreamReference> = if !output_streams.is_empty() {
        output_streams
            .iter()
            .flat_map(|s| s.split(','))
            .map(|s| s.trim())
            .map(|stream_name| {
                streamir
                    .sr2memory
                    .iter()
                    .find_map(|(sr, m)| (m.name == stream_name).then_some(*sr))
                    .ok_or_else(|| {
                        anyhow::anyhow!("stream {stream_name} does not exist in the specification")
                    })
            })
            .collect::<anyhow::Result<_>>()?
    } else {
        match verbosity {
            Verbosity::Silent => Vec::new(),
            Verbosity::Trigger => streamir.triggers().map(StreamReference::Out).collect(),
            Verbosity::Outputs => streamir.outputs().map(StreamReference::Out).collect(),
            Verbosity::Streams => streamir.streams().collect(),
        }
    };

    let csv_writer = CsvVerdictSink::new(stderr(), &streamir, &csv_fields)
        .context("building csv output writer")?;

    let monitor = Monitor::build(streamir, optimize);

    Ok((monitor, csv_source, csv_writer))
}

fn run(
    mut monitor: Monitor,
    mut csv_source: CsvEventSource<File>,
    mut csv_writer: CsvVerdictSink<Stderr>,
    benchmark: bool,
) -> anyhow::Result<()> {
    let start = Instant::now();
    let mut last_ts = Duration::new(0, 0);
    while let Some((inputs, ts)) = csv_source
        .next_event()
        .context("getting next event from csv source")?
    {
        let verdict = monitor.accept_event(inputs, ts);
        if !benchmark {
            for (ts, timed_verdict) in verdict.timed {
                csv_writer
                    .accept_verdict(ts, timed_verdict)
                    .context("writing timed verdicts to csv")?;
            }
            csv_writer
                .accept_verdict(verdict.ts, verdict.event)
                .context("writing event verdict to csv")?;
        } else {
            unsafe {
                std::mem::forget(ptr::read_volatile(&verdict));
            }
        }
        last_ts = ts;
    }
    let verdicts = monitor.finish(last_ts);
    if !benchmark {
        for (ts, timed_verdict) in verdicts {
            csv_writer
                .accept_verdict(ts, timed_verdict)
                .context("writing final timed verdicts to csv")?;
        }
    } else {
        unsafe {
            std::mem::forget(ptr::read_volatile(&verdicts));
        }
        println!("{}", start.elapsed().as_secs_f64());
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Args {
        spec,
        trace,
        verbosity,
        output_streams,
        optimize,
        benchmark,
    } = Args::parse();

    let config = ParserConfig::from_path(spec).context("loading specification file")?;
    match build(&config, trace, verbosity, output_streams, optimize) {
        Ok((monitor, source, sink)) => {
            run(monitor, source, sink, benchmark).context("running monitor")
        }
        Err(e) => {
            if let Some(ParseError::FrontendError(e)) = e.downcast_ref() {
                let handler = Handler::from(&config);
                handler.emit_error(e);
                std::process::exit(1);
            } else {
                Err(e)
            }
        }
    }
}
