use anyhow::Context;
use rtlola2solidity::{
    interface::InterfaceConfig, SolidityFormatter, TriggerAction, TriggerFunctionMode,
};
use rtlola_streamir::{
    optimize, parse,
    rewrite_rules::{
        CombineIf, CombineIterate, CombineNestedIf, CombineSeq, ImpliedGuards, IterateAssign,
        MemoryOptimizations, MoveCommonGuardsOutside, MoveIfOutside, RemoveClose, RemoveIfs,
        RemoveShift, RemoveSpawn, SimplifyGuard,
    },
    translate, Handler, ParseError, ParserConfig,
};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A tool to compile RTLola specifications to Solidity
struct Args {
    /// Path to the specification
    spec_path: PathBuf,
    /// Configuration that maps function arguments/return values to input/output streams
    config_file: PathBuf,
    #[clap(long, short = 'n', default_value = "Contract")]
    /// The name of the resulting contract
    contract_name: String,
    #[clap(long, short, value_enum, default_value_t = OptimizationLevel::PartialEval)]
    /// Whether to optimize the IR
    optimize: OptimizationLevel,
    /// Whether a trigger throws revert or emits an event
    #[clap(long, value_enum, default_value_t=TriggerAction::EmitMultiple)]
    trigger_action: TriggerAction,
    /// Whether a function is generated for each trigger or only a single one
    #[clap(long, value_enum, default_value_t=TriggerFunctionMode::Multiple)]
    trigger_function_mode: TriggerFunctionMode,
    #[clap(long)]
    /// The path where the output contract is written to
    output_file: PathBuf,
    /// Whether to overwrite already existing output files
    #[clap(long)]
    overwrite: bool,
    /// Emit the given output streams at the end of each cycle evaluation (for testing purposes)
    #[clap(long)]
    output_streams: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy, Eq, PartialEq)]
enum OptimizationLevel {
    PartialEval,
    Rewriting,
    Memory,
    All,
}

fn run(config: &ParserConfig, args: Args) -> anyhow::Result<()> {
    let Args {
        spec_path: _,
        config_file,
        contract_name,
        optimize: optimize_level,
        trigger_action,
        trigger_function_mode,
        output_file,
        overwrite,
        output_streams,
    } = args;

    let ir = parse(config).context("parsing specification to StreamIR")?;

    let optimized_ir = match optimize_level {
        OptimizationLevel::PartialEval => Ok(ir),
        OptimizationLevel::Rewriting => optimize(
            ir,
            vec![
                Box::new(CombineIf),
                Box::new(SimplifyGuard),
                Box::new(MoveCommonGuardsOutside),
                Box::new(ImpliedGuards),
                Box::new(SimplifyGuard),
                Box::new(RemoveIfs),
                Box::new(CombineSeq),
                Box::new(MoveIfOutside),
                Box::new(IterateAssign),
                Box::new(CombineNestedIf),
                Box::new(CombineIterate),
                Box::new(RemoveIfs),
            ],
        ),
        OptimizationLevel::Memory => optimize(
            ir,
            vec![
                Box::new(RemoveShift),
                Box::new(MemoryOptimizations),
                Box::new(RemoveSpawn),
                Box::new(RemoveClose),
            ],
        ),
        OptimizationLevel::All => optimize(
            ir,
            vec![
                Box::new(CombineIf),
                Box::new(SimplifyGuard),
                Box::new(MoveCommonGuardsOutside),
                Box::new(ImpliedGuards),
                Box::new(SimplifyGuard),
                Box::new(RemoveIfs),
                Box::new(CombineSeq),
                Box::new(MoveIfOutside),
                Box::new(IterateAssign),
                Box::new(CombineNestedIf),
                Box::new(CombineIterate),
                Box::new(RemoveIfs),
                Box::new(RemoveShift),
                Box::new(MemoryOptimizations),
                Box::new(RemoveSpawn),
                Box::new(RemoveClose),
            ],
        ),
    }
    .context("optimizing specification")?;

    let config = std::fs::read_to_string(config_file.clone())
        .with_context(|| format!("reading config file: {}", config_file.display()))?;
    let config = InterfaceConfig::from_toml(&config)
        .map_err(anyhow::Error::msg)
        .context("loading function interface")?;

    let output_streams = output_streams
        .iter()
        .flat_map(|s| s.split(','))
        .map(|s| s.trim())
        .map(|stream_name| {
            optimized_ir
                .sr2memory
                .iter()
                .find_map(|(sr, m)| (m.name == stream_name).then_some(*sr))
                .ok_or_else(|| {
                    anyhow::anyhow!("stream {stream_name} does not exist in the specification")
                })
        })
        .collect::<anyhow::Result<_>>()?;

    let formatter = SolidityFormatter::new(
        &optimized_ir,
        config,
        contract_name,
        trigger_action,
        trigger_function_mode,
        output_file,
        overwrite,
        output_streams,
    );

    Ok(translate(optimized_ir, formatter)?)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.trigger_action == TriggerAction::EmitMultiple
        && args.trigger_function_mode == TriggerFunctionMode::Single
    {
        anyhow::bail!("Conflicting command line arguments: --trigger-action emit-multiple with --trigger-function-mode single")
    }

    let parser_config = ParserConfig::from_path(args.spec_path.clone())
        .with_context(|| format!("loading specification file: {}", args.spec_path.display()))?;

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
