use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use clap::{Args, Parser, ValueEnum};
use itertools::Itertools;
use rtlola2rust::{MainFunction, NoStdInfo, RustFormatter};
use rtlola_streamir::{
    ir::StreamReference, optimize_all, parse, translate, Handler, ParseError, ParserConfig,
};

#[derive(Parser)]
struct CliArgs {
    /// The path to the RTLola specification
    spec: PathBuf,
    #[clap(short, long)]
    /// Whether to optimize the StreamIR
    optimize: bool,
    #[clap(short = 'd', long, default_value = ".")]
    /// Specify the output directory for the generated files
    output_dir: PathBuf,
    #[clap(long)]
    /// Overwrite existing files
    overwrite: bool,
    #[clap(short, long, value_enum, default_value_t=MainFunction::NoMain)]
    /// Specify which main function to generate
    main: MainFunction,
    #[clap(short, long, default_value_t=Verbosity::Outputs, value_enum)]
    /// Specify which streams occur in verdict
    verbosity: Verbosity,
    #[clap(long)]
    /// Only output the following streams
    output_streams: Vec<String>,
    #[command(flatten)]
    no_std: NoStdCli,
}

#[derive(Args)]
#[command(next_help_heading = "NoStd Configuration")]
struct NoStdCli {
    #[clap(long)]
    /// Generate code without using the std library
    no_std: bool,
    #[clap(long, requires = "no_std")]
    /// Specify the maximal number of instances per parameterized stream
    num_instances: Vec<String>,
    /// Specify the maximal number of streams spawned per cycle
    #[clap(long, requires = "no_std")]
    max_spawned: Option<usize>,
    /// Specify the maximal number of streams closed per cycle
    #[clap(long, default_value_t = 10, requires = "no_std")]
    max_closed: usize,
    /// Specify the maximal number of periodic verdict per event
    #[clap(long, default_value_t = 100, requires = "no_std")]
    max_verdict_periodic: usize,
    /// Specify the maximal number of streams per dynamic deadline
    #[clap(long, default_value_t = 5, requires = "no_std")]
    max_dynamic_deadlines: usize,
    /// Specify the maximal number of instances per dynamically scheduled stream
    #[clap(long, default_value_t = 5, requires = "no_std")]
    max_dynamic_instances: usize,
    /// Specify the maximal size of the queue
    #[clap(long, default_value_t = 50, requires = "no_std")]
    max_queue_size: usize,
}

#[derive(Clone, Copy, ValueEnum)]
enum Verbosity {
    Streams,
    Outputs,
    Trigger,
    Silent,
}

fn run(config: &ParserConfig, args: CliArgs) -> anyhow::Result<()> {
    let CliArgs {
        spec: _,
        optimize,
        output_dir,
        overwrite,
        main,
        verbosity,
        output_streams,
        no_std,
    } = args;

    let mut ir = parse(config).context("parsing specification")?;
    if optimize {
        ir = optimize_all(ir).context("optimizing StreamIR")?;
    }

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

    let no_std_info = if no_std.no_std {
        let NoStdCli {
            no_std: _,
            num_instances,
            max_spawned,
            max_closed,
            max_verdict_periodic,
            max_dynamic_deadlines,
            max_dynamic_instances,
            max_queue_size,
        } = no_std;

        let max_instances = num_instances
            .iter()
            .flat_map(|s| s.split(","))
            .map(|s| {
                s.split_once("=").ok_or_else(|| {
                    anyhow::anyhow!(
                        "could not parse \"{s}\" as stream-number assignment. Equals required."
                    )
                })
            })
            .map_ok(|(stream, num)| {
                let sr = ir.stream_by_name(stream).ok_or_else(|| {
                    anyhow::anyhow!("stream {stream} not found in specification.")
                })?;
                if ir.stream_memory(sr).num_parameters() == 0 {
                    anyhow::bail!("stream {stream} is not an parameterized output.")
                }
                let num_instances: usize = num
                    .parse()
                    .with_context(|| format!("parsing number of instances for stream {stream}"))?;
                Ok((sr.out_idx(), num_instances))
            })
            .collect::<anyhow::Result<anyhow::Result<HashMap<_, _>>>>()??;

        let max_spawned = max_spawned.unwrap_or(max_instances.len());

        let missing_outputs = ir
            .parameterized_outputs()
            .filter(|sr| !max_instances.contains_key(sr))
            .map(|sr| ir.name(sr.sr()))
            .collect::<Vec<_>>();
        if !missing_outputs.is_empty() {
            anyhow::bail!("the no_std option requires number of instances for the following parameterized output streams: {missing_outputs:?}")
        }

        Some(NoStdInfo {
            max_instances,
            max_spawned,
            max_closed,
            max_verdict_periodic,
            max_dynamic_deadlines,
            max_dynamic_instances,
            max_queue_size,
        })
    } else {
        None
    };

    let formatter = RustFormatter::new(
        &ir,
        output_dir,
        overwrite,
        main,
        verdict_streams,
        no_std_info,
    );
    translate(ir, formatter).context("generating rust code")
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let parser_config = ParserConfig::from_path(args.spec.clone())
        .with_context(|| format!("loading specification file: {}", args.spec.display()))?;

    if let Err(e) = run(&parser_config, args) {
        if let Some(ParseError::FrontendError(e)) = e.downcast_ref() {
            let handler = Handler::from(&parser_config);
            handler.emit_error(e);
            std::process::exit(1);
        } else {
            Err(e)
        }
    } else {
        Ok(())
    }
}
