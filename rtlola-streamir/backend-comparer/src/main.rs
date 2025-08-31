//! Tests RtLola monitoring implementations by monitoring specifications on different
//! traces and compare the results between different implementations.

#![forbid(unused_must_use)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::{fs::read_to_string, iter::zip, path::PathBuf};

use clap::{ArgAction, Parser, ValueEnum};
use test_case::{NoResult, TestCase};
use test_runner::run_tests;

mod error;
mod runner;
mod test_case;
mod test_runner;

/// Runs two different RtLola monitoring implementations on different
/// test cases and compares their output.
#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// a list of runners that run the tests
    #[arg(short, long)]
    runner: Vec<RunnerArg>,
    #[arg(short, long)]
    binary: Vec<PathBuf>,
    /// one or more csv-files containing all the test cases.
    #[arg(value_name = "TEST_FILE", required = true)]
    test_files: Vec<PathBuf>,
    /// Filter which specifications are tested.
    #[arg(short, long="filter", value_name="SPEC", action=ArgAction::Append)]
    filters: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub(crate) enum RunnerArg {
    /// run the specification and trace on the rtlola-interpeter.
    Interpreter,
    /// run the trace on a monitor that the rtlola-compiler compiled in Rust.
    RustCompiler,
    /// run the trace on a monitor that the rtlola-compiler compiled in optimized Rust.
    OptimizedRustCompiler,
    NewInterpreter,
    OptimizedNewInterpreter,
}

impl std::fmt::Display for RunnerArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self.to_possible_value().unwrap().get_name(), f)
    }
}

fn main() -> std::process::ExitCode {
    let Args {
        runner,
        binary,
        test_files,
        filters,
    } = Args::parse();

    if runner.len() <= 1 {
        eprintln!("Give at least two runner as an input.");
        std::process::exit(1);
    }

    if runner.len() != binary.len() {
        eprintln!("Give a path to the binary for each runner.");
        eprintln!("Runner: {runner:?}, Paths: {binary:?}");
        std::process::exit(1);
    }

    let runner = zip(runner, binary).collect();

    let test_cases = test_files
        .into_iter()
        // read files to string
        .map(|test_file| read_to_string(test_file).expect("error reading test file"))
        // parse json content to test cases
        .flat_map(|test_json| {
            serde_json::from_str::<Vec<TestCase<NoResult>>>(&test_json)
                .expect("error parsing test json")
        })
        // filter test cases based on --filter argument
        .filter(|test_case| {
            if !filters.is_empty() {
                filters
                    .iter()
                    .any(|filter| test_case.spec.to_string_lossy().contains(filter))
            } else {
                true
            }
        })
        .collect();

    let success = run_tests(test_cases, runner);

    if success {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}
