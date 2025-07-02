use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, ValueEnum};
use rtlola_frontend::{Handler, ParserConfig};
use rtlola_streamir::{ir::DebugFormatter, parse, translate, ParseError};

#[derive(Parser)]
struct Args {
    spec: PathBuf,
    #[clap(short, long)]
    optimize_all: bool,
}

#[derive(ValueEnum, Clone)]
enum Optimization {}

fn print(config: &ParserConfig, optimize_all: bool) -> anyhow::Result<String> {
    let streamir = parse(config).context("parsing specification to StreamIR")?;
    let streamir = if optimize_all {
        rtlola_streamir::optimize_all(streamir).context("optimizing StreamIR")?
    } else {
        streamir
    };
    let formatter = DebugFormatter::new(&streamir);
    Ok(translate(streamir, formatter))
}

fn main() -> anyhow::Result<()> {
    let Args { spec, optimize_all } = Args::parse();

    let config = ParserConfig::from_path(spec.clone())
        .with_context(|| format!("loading specification file: {}", spec.display()))?;

    match print(&config, optimize_all) {
        Ok(res) => {
            println!("{res}")
        }
        Err(e) => {
            if let Some(ParseError::FrontendError(e)) = e.downcast_ref() {
                let handler = Handler::from(&config);
                handler.emit_error(e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
