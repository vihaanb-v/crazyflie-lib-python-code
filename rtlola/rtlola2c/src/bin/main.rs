use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, ValueEnum};
use itertools::Itertools;
use rtlola2c::main_function::MainFunction;
use rtlola2c::CFormatter;
use rtlola_streamir::formatter::StreamIrFormatter;
use rtlola_streamir::ir::StreamReference;
use rtlola_streamir::{optimize_all, parse, ParserConfig};

#[derive(Parser)]
struct Args {
    /// The path to the specification
    spec: PathBuf,
    #[clap(long)]
    /// Whether to overwrite existing files
    overwrite: bool,
    /// Whether to optimize the StreamIR
    #[clap(short, long)]
    optimize: bool,
    #[clap(short, long, value_enum, default_value_t=MainFunction::NoMain)]
    /// Specify which main function to generate
    main: MainFunction,
    #[clap(long, default_value = ".")]
    output_dir: PathBuf,
    #[clap(long)]
    output_streams: Vec<String>,
    #[clap(short, long, default_value_t=Verbosity::Outputs, value_enum)]
    verbosity: Verbosity,
}

#[derive(Clone, Copy, ValueEnum)]
enum Verbosity {
    Streams,
    Outputs,
    Trigger,
    Silent,
}

fn generate(
    config: &ParserConfig,
    overwrite: bool,
    optimize: bool,
    main: MainFunction,
    output_streams: Vec<String>,
    verbosity: Verbosity,
    output_dir: PathBuf,
) -> anyhow::Result<()> {
    let ir = parse(config).context("parsing specification to StreamIR")?;
    let ir = if optimize {
        optimize_all(ir).context("optimizing StreamIR")?
    } else {
        ir
    };

    let verdict_streams: Vec<StreamReference> = if !output_streams.is_empty() {
        output_streams
            .iter()
            .flat_map(|s| s.split(','))
            .map(|s| s.trim())
            .map(|stream_name| {
                ir.sr2memory
                    .iter()
                    .find_map(|(sr, m)| (m.name == stream_name).then_some(*sr))
                    .ok_or_else(|| {
                        anyhow::anyhow!("stream {stream_name} does not exist in the specification")
                    })
            })
            .collect::<anyhow::Result<_>>()
            .context("finding output streams")?
    } else {
        match verbosity {
            Verbosity::Silent => Vec::new(),
            Verbosity::Trigger => ir.triggers().sorted().map(StreamReference::Out).collect(),
            Verbosity::Outputs => ir.outputs().sorted().map(StreamReference::Out).collect(),
            Verbosity::Streams => ir.streams().sorted().collect(),
        }
    };

    let formatter = CFormatter::new(&ir, overwrite, main, verdict_streams, output_dir);
    formatter
        .format(ir)
        .context("formatting StreamIR as C code")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Args {
        spec,
        overwrite,
        optimize,
        main,
        output_dir,
        output_streams,
        verbosity,
    } = Args::parse();
    let config = ParserConfig::from_path(spec).context("loading specification file")?;
    generate(
        &config,
        overwrite,
        optimize,
        main,
        output_streams,
        verbosity,
        output_dir,
    )
    .context("generating C code")
}
