use std::{
    path::{Path, PathBuf},
    process::Command,
};

use rtlola_io_plugins::outputs::json_plugin::JsonVerdict;

use crate::{error::Error, runner::Verdict};

use super::{RunOutput, Runner};

pub(crate) struct Interpreter {
    binary: PathBuf,
    spec: PathBuf,
}
impl Interpreter {
    pub(crate) fn new(binary: &Path, spec: &Path) -> Result<Self, Error> {
        Ok(Self {
            binary: binary.into(),
            spec: spec.into(),
        })
    }
}

/// Runs the rtlola-interpreter on the given specification and trace
/// and returns the log output.
impl Runner for Interpreter {
    fn run(&self, trace: &Path, only_output: Option<&[String]>) -> Result<RunOutput, Error> {
        assert!(only_output.is_none());
        let output = Command::new(&self.binary)
            .arg("monitor")
            // absolute timestamp in trace
            .args(["--offline", "absolute"])
            // run the interpreter on the given trace
            .arg("--csv-in")
            .arg(trace)
            // print the same absolute time as in the input
            .args(["--output-time-format", "absolute"])
            // print logs about new stream values and triggers
            .args(["--verbosity", "outputs"])
            // print output in csv format
            .args(["--output-format", "json"])
            .arg(&self.spec)
            // run interpeter and capture output
            .output()
            .map_err(|e| Error::from(format!("error running rtlola-interpreter: {e}")))?;

        if !output.status.success() {
            return Err(
                Error::from("rtlola-interpreter existed with non-zero exit code.")
                    .with_debug(String::from_utf8_lossy(&output.stderr).trim()),
            );
        }

        // temporary fix until we have decided how to output parameterized streams with Compiler
        let mut stdout = &output.stdout[..];
        let verdicts = std::iter::from_fn(|| jsonl::read::<_, JsonVerdict>(&mut stdout).ok())
            .map(|mut verdict| {
                RunOutput::round_floats(&mut verdict.time);
                verdict
            })
            .map(|verdict| Verdict {
                time: verdict.time.parse().unwrap(),
                outputs: verdict
                    .updates
                    .into_iter()
                    .flat_map(|(stream, updates)| {
                        updates
                            .into_iter()
                            .find_map(|update| {
                                update.eval.map(|v| {
                                    if update.instance.is_empty() {
                                        let mut v = v.to_string();
                                        v = v.trim_matches('"').into();
                                        RunOutput::round_floats(&mut v);
                                        v
                                    } else {
                                        "!!".to_string()
                                    }
                                })
                            })
                            .map(|u| (stream, u))
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        Ok(RunOutput(verdicts))
    }
}
