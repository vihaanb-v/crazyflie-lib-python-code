use std::{path::Path, process::Command};

use super::compiler_runner::Compiler;

pub(crate) struct RustCompiler;

pub(crate) struct OptimizedRustCompiler;

impl Compiler for OptimizedRustCompiler {
    fn lola_compiler(
        lola_path: &Path,
        compiler_binary: &Path,
        compiled_path: &Path,
        only_output: Option<&[String]>,
    ) -> Command {
        let mut cmd =
            RustCompiler::lola_compiler(lola_path, compiler_binary, compiled_path, only_output);
        cmd.arg("--optimize");
        cmd
    }

    fn compile_generated(generated_code_path: &Path, binary_path: &Path) -> Command {
        RustCompiler::compile_generated(generated_code_path, binary_path)
    }

    fn binary_runner(binary_path: &Path, trace: &Path, only_output: Option<&[String]>) -> Command {
        RustCompiler::binary_runner(binary_path, trace, only_output)
    }
}

impl Compiler for RustCompiler {
    fn lola_compiler(
        lola_path: &Path,
        compiler_binary: &Path,
        compiled_path: &Path,
        only_output: Option<&[String]>,
    ) -> Command {
        let mut cmd = Command::new(compiler_binary);
        cmd.args(["-m", "csv-offline", "--output-dir"])
            .arg(compiled_path)
            .arg(lola_path);
        if let Some(streams) = only_output {
            cmd.arg("--output-streams").arg(streams.join(","));
        }
        cmd
    }

    fn compile_generated(generated_code_path: &Path, binary_path: &Path) -> Command {
        let mut cmd = Command::new("rustc");
        cmd.args(["--edition", "2021"])
            .arg("-o")
            .arg(binary_path)
            .arg(generated_code_path.join("main.rs"));
        cmd
    }

    fn binary_runner(binary_path: &Path, trace: &Path, _only_output: Option<&[String]>) -> Command {
        let mut cmd = Command::new(binary_path);
        cmd.arg(trace);
        cmd
    }
}
