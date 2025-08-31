use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::error::Error;

use super::{RunOutput, Runner};

pub(crate) struct NewInterpreter {
    binary: PathBuf,
    spec: PathBuf,
    optimized: bool,
}
impl NewInterpreter {
    pub(crate) fn new(binary: &Path, spec: &Path, optimized: bool) -> Result<Self, Error> {
        Ok(Self {
            binary: binary.into(),
            spec: spec.into(),
            optimized,
        })
    }
}

/// Runs the rtlola-interpreter on the given specification and trace
/// and returns the log output.
impl Runner for NewInterpreter {
    fn run(&self, trace: &Path, streams: Option<&[String]>) -> Result<RunOutput, Error> {
        let mut cmd = Command::new(&self.binary);
        cmd.arg(&self.spec).arg(trace);
        if self.optimized {
            cmd.arg("--optimize");
        }
        if let Some(streams) = streams {
            cmd.arg("--output-streams").arg(streams.join(","));
        }

        // run interpeter and capture output
        let output = cmd
            .output()
            .map_err(|e| Error::from(format!("error running rtlola-interpreter: {e}")))?;

        if !output.status.success() {
            return Err(
                Error::from("rtlola-interpreter existed with non-zero exit code.")
                    .with_debug(String::from_utf8_lossy(&output.stderr).trim()),
            );
        }

        // temporary fix until we have decided how to output parameterized streams with Compiler
        let stderr = &output.stderr[..];
        RunOutput::from_reader(stderr)
    }
}
