use std::{collections::HashMap, io::Read, path::Path};

use crate::{error::Error, RunnerArg};

mod compiler_runner;
mod interpreter;
mod new_interpreter;
mod rust_compiler;

pub(crate) use interpreter::Interpreter;
use itertools::Itertools;
use new_interpreter::NewInterpreter;
pub(crate) use rust_compiler::RustCompiler;
use serde::Deserialize;

use self::{compiler_runner::CompilerRunner, rust_compiler::OptimizedRustCompiler};

#[derive(Debug, Clone)]
pub(crate) struct RunOutput(pub Vec<Verdict>);

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Verdict {
    pub time: f64,
    pub outputs: HashMap<String, String>,
}

impl RunOutput {
    fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let inner = csv::Reader::from_reader(reader)
            .deserialize()
            .map(|r| r.map_err(|e| Error::from(format!("Error parsing monitor output: {e:?}"))))
            .map_ok(|mut map: HashMap<String, String>| {
                map.values_mut().for_each(Self::round_floats);
                Verdict {
                    time: map
                        .remove("time")
                        .expect("time column missing")
                        .parse()
                        .expect("error parsing time column"),
                    outputs: map
                        .into_iter()
                        .filter(|(_, v)| v != "#")
                        .map(|(k, v)| (k, v.trim_matches('"').to_string()))
                        .collect(),
                }
            })
            // the compiler outputs all hashtags if the current cycle did not produce any new outputs, while the interpreter does not.
            .filter_ok(|v| !v.outputs.is_empty())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RunOutput(inner))
    }

    fn round_floats(potential_float: &mut String) {
        if let Ok(num) = potential_float.parse::<f64>() {
            *potential_float = format!("{:.4}", num)
        }
    }
}

/// A runner is able to take an rtlola specification and a trace and produces log output.
/// The log output is used to compare the results of different runners with each other.
pub(crate) trait Runner {
    /// Run the specification on the given trace and returns the output.
    fn run(&self, trace: &Path, streams: Option<&[String]>) -> Result<RunOutput, Error>;
}

impl RunnerArg {
    pub(crate) fn into_runner(
        self,
        binary: &Path,
        spec: &Path,
        streams: Option<&[String]>,
    ) -> Result<Box<dyn Runner>, Error> {
        Ok(match self {
            RunnerArg::Interpreter => {
                let runner: Box<dyn Runner> = Box::new(Interpreter::new(binary, spec)?);
                runner
            }
            RunnerArg::RustCompiler => {
                let runner: Box<dyn Runner> =
                    Box::new(CompilerRunner::<RustCompiler>::new(binary, spec, streams)?);
                runner
            }
            RunnerArg::OptimizedRustCompiler => {
                let runner: Box<dyn Runner> = Box::new(
                    CompilerRunner::<OptimizedRustCompiler>::new(binary, spec, streams)?,
                );
                runner
            }
            RunnerArg::NewInterpreter => {
                let runner: Box<dyn Runner> = Box::new(NewInterpreter::new(binary, spec, false)?);
                runner
            }
            RunnerArg::OptimizedNewInterpreter => {
                let runner: Box<dyn Runner> = Box::new(NewInterpreter::new(binary, spec, true)?);
                runner
            }
        })
    }
}
