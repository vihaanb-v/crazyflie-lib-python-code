use std::{
    fs::{self, File},
    io::Write,
    marker::PhantomData,
    path::{Path, PathBuf},
    process::Command,
};

use tempdir::TempDir;

use crate::error::Error;

use super::{RunOutput, Runner};

pub(crate) trait Compiler {
    /// Compile lola spec to target language
    fn lola_compiler(
        lola_path: &Path,
        compiler_binary: &Path,
        to: &Path,
        only_output: Option<&[String]>,
    ) -> Command;
    /// Compile target language to binary
    fn compile_generated(generated_code_path: &Path, binary_path: &Path) -> Command;
    /// Run binary on given trace
    fn binary_runner(binary_path: &Path, trace: &Path, only_output: Option<&[String]>) -> Command;
}

pub(crate) struct CompilerRunner<C> {
    binary: PathBuf,
    temp_dir: PathBuf,
    compiled_path: PathBuf,
    binary_path: PathBuf,
    logs_path: PathBuf,
    compiler: PhantomData<C>,
}

impl<C: Compiler> CompilerRunner<C> {
    pub(crate) fn new(
        binary: &Path,
        spec: &Path,
        only_output: Option<&[String]>,
    ) -> Result<Self, Error> {
        let temp_dir = TempDir::new("lola-test").expect("error creating temp dir");
        let temp_dir = temp_dir.into_path(); // this prevents the temp dir from getting deleted after getting out of scope
        let compiled_path = temp_dir.join("compiled_spec");
        let binary_path = temp_dir.join("monitor");
        let logs_path = temp_dir.join("logs.txt");

        let runner = Self {
            binary: binary.into(),
            temp_dir,
            compiled_path,
            binary_path,
            logs_path,
            compiler: PhantomData,
        };

        runner.compile_lola_to_target_language(spec, only_output)?;
        runner.compile_target_language_to_binary()?;
        Ok(runner)
    }
}

impl<C: Compiler> Runner for CompilerRunner<C> {
    fn run(&self, trace: &Path, only_output: Option<&[String]>) -> Result<RunOutput, Error> {
        let res = self.run_monitor(trace, only_output)?;
        RunOutput::from_reader(&res[..])
    }
}

impl<C: Compiler> CompilerRunner<C> {
    /// use the rtlola-compiler to compile the specification to target language
    fn compile_lola_to_target_language(
        &self,
        lola_path: &Path,
        only_output: Option<&[String]>,
    ) -> Result<(), Error> {
        fs::create_dir(&self.compiled_path).map_err(|_| {
            format!(
                "Error creating target directory: {}",
                self.compiled_path.display()
            )
        })?;
        let output = C::lola_compiler(lola_path, &self.binary, &self.compiled_path, only_output)
            .output()
            .map_err(|_| format!("compiler binary {} does not exist.", &self.binary.display()))?;

        if !output.status.success() {
            return Err(
                Error::from("non zero exit code when compiling lola to target language")
                    .with_debug(String::from_utf8_lossy(&output.stderr)),
            );
        }
        Ok(())
    }

    /// compile the (previously generated target code) to an executable
    fn compile_target_language_to_binary(&self) -> Result<(), Error> {
        let result = C::compile_generated(&self.compiled_path, &self.binary_path)
            .output()
            .map_err(|_| "error compiling target language to binary")?;

        if !result.status.success() {
            return Err(
                Error::from("non zero exit code when compiling target language to binary")
                    .with_debug(format!("Target Code: {}", self.compiled_path.display()))
                    .with_debug(String::from_utf8_lossy(&result.stderr)),
            );
        }

        Ok(())
    }

    /// run the compiled monitor on a given trace
    fn run_monitor(&self, trace: &Path, only_output: Option<&[String]>) -> Result<Vec<u8>, Error> {
        let output = C::binary_runner(&self.binary_path, trace, only_output)
            .output()
            .map_err(|_| {
                format!(
                    "error running monitor binary: {}",
                    self.binary_path.display()
                )
            })?;

        if !output.status.success() {
            return Err(Error::from("non zero exit code when running monitor")
                .with_debug(format!("compilation files: {}", self.temp_dir.display())));
        }

        // write logs to `self.logs_path` (only for debugging)
        let mut f = File::create(&self.logs_path).expect("error creating log file");
        f.write_all(&output.stdout)
            .expect("error writing log output");

        Ok(output.stdout)
    }
}
