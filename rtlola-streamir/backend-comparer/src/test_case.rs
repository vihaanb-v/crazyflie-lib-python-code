use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{
    error::Error,
    runner::RunOutput,
    test_runner::{DEBUG_STYLE, FAILED_STYLE, FAILED_TEXT, PASSED_TEXT, TERMINAL_WIDTH},
    RunnerArg,
};

#[derive(Default)]
pub(crate) struct NoResult;

/// Holds the information about all the tests of one specification (which can
/// consist of different traces)
/// Before the test was run, it has type TestCase<NoResult>, and running
/// it turns the test to TestCase<TesultResult>, containg the results of the test.
#[derive(Deserialize)]
pub(crate) struct TestCase<R> {
    /// The specification to test.
    pub spec: PathBuf,
    /// All the traces to test the specification with.
    pub traces: Vec<PathBuf>,
    /// If given, only compare the specified streams
    pub streams: Option<Vec<String>>,
    /// The result of the test case
    #[serde(skip)]
    pub result: R,
}

/// The result of a test case (consisting of different traces).
pub(crate) enum TestResult {
    /// There was an error during building the Runner.
    InitError(Error),
    /// The runner was sucessfully build.
    Results(
        /// A vector of results (one for each trace).
        Vec<Result<(), Error>>,
    ),
}

impl From<Error> for TestResult {
    fn from(value: Error) -> Self {
        Self::InitError(value)
    }
}

impl TestCase<NoResult> {
    /// Test all the traces in the TestCase and attach the result to the TestCase.
    pub(crate) fn run(self, runner: &[(RunnerArg, PathBuf)]) -> TestCase<TestResult> {
        let result = self.get_result(runner);

        let Self {
            spec,
            traces,
            streams,
            result: _,
        } = self;

        TestCase::<TestResult> {
            spec,
            traces,
            streams,
            result,
        }
    }

    /// Test all the traces in the TestCase and return the results.
    fn get_result(&self, runner: &[(RunnerArg, PathBuf)]) -> TestResult {
        // we need at least one trace
        if self.traces.is_empty() {
            return TestResult::InitError("test case has to have at least one trace.".into());
        }

        let runner = runner
            .iter()
            .map(|(runner, binary)| runner.into_runner(binary, &self.spec, self.streams.as_deref()))
            .collect::<Result<Vec<_>, _>>();

        let runner = match runner {
            Ok(runner) => runner,
            Err(e) => return TestResult::InitError(e),
        };

        // for each trace: run trace with each runner and compare results
        let results = self
            .traces
            .iter()
            .map(|trace| {
                let outputs = runner
                    .iter()
                    .map(|runner| runner.run(trace, self.streams.as_deref()))
                    .collect::<Result<_, _>>()?;
                compare_outputs(outputs)
            })
            .collect();

        TestResult::Results(results)
    }
}

impl<R> TestCase<R> {
    /// Returns the total number of tests (=traces) in the test case.
    pub(crate) fn total_num_tests(&self) -> usize {
        self.traces.len()
    }
}

impl TestCase<TestResult> {
    /// Returns the number of tests (=traces) in the test case that were passed.
    pub(crate) fn num_passed_tests(&self) -> usize {
        match &self.result {
            // tests were actually run
            TestResult::Results(results) => results.iter().filter(|r| r.is_ok()).count(),
            // init error
            TestResult::InitError(_) => 0,
        }
    }

    /// Returns whether all tests (=traces) in the test case were passed.
    pub(crate) fn passed(&self) -> bool {
        self.num_passed_tests() == self.total_num_tests()
            && matches!(self.result, TestResult::Results { .. })
    }

    /// Print the header of the test result to stdout.
    fn print_header_result(&self) {
        let passed_mark = if self.passed() {
            &*PASSED_TEXT
        } else {
            &*FAILED_TEXT
        };

        let spec_name = self.spec.file_name().unwrap().to_string_lossy();

        let result_line = format!("{passed_mark} {spec_name}");
        let passed_tests_str = format!("[{}/{}]", self.num_passed_tests(), self.total_num_tests());

        // pad the result line with dots and print the number of passed tests (=traces) on the right.
        let pad_width = *TERMINAL_WIDTH + 4; // the 4 is to compensate escape characters that are not printed
        let result_line = format!("{result_line:.<pad_width$}{passed_tests_str}");

        println!("{result_line}");

        // if there was an error while initializing the runner, print the error message in separate line.
        if let TestResult::InitError(e) = &self.result {
            println!(
                "{}",
                &format!("  {} {e}", FAILED_STYLE.apply_to("[INIT ERROR]"))
            );
            self.print_debug_info(e);
        }
    }

    /// Print the test result to stdout that shows the information of one trace of the test case.
    fn print_trace_result(&self, trace: &Path, result: &Result<(), Error>) {
        let trace_name = trace.file_name().unwrap().to_string_lossy();

        match &result {
            Ok(_) => println!("  {} {trace_name}", &*PASSED_TEXT),
            Err(e) => {
                println!("  {} {trace_name}: {e}", &*FAILED_TEXT);
                self.print_debug_info(e);
            }
        };
    }

    //  Prints the debug information if there is some attached to the error
    fn print_debug_info(&self, error: &Error) {
        if let Some(debug) = error.debug_info() {
            let debug_msg = format!("  [DEBUG INFO] {debug}");
            println!("{}", DEBUG_STYLE.apply_to(debug_msg))
        }
    }

    pub(crate) fn print_result(&self) {
        // print the header
        self.print_header_result();

        // only print the individual traces if the initialisation was successful
        if let TestResult::Results(results) = &self.result {
            // there has to be a result for every trace
            assert!(results.len() == self.traces.len());

            // print the test result for each trace in the test case
            for (trace, result) in self.traces.iter().zip(results.iter()) {
                self.print_trace_result(trace, result);
            }
        }
    }
}

/// Compares the output of two runner outputs.
/// Returns Ok(()) if they are equal, otherwise returns an error message.
pub(crate) fn compare_outputs(outputs: Vec<RunOutput>) -> Result<(), Error> {
    let mut outputs = outputs
        .into_iter()
        .map(|lines| lines.0.into_iter())
        .collect::<Vec<_>>();

    loop {
        let current_lines = outputs
            .iter_mut()
            .map(|lines| lines.next())
            .collect::<Vec<_>>();

        // if all runner have no lines left, we are done
        if current_lines.iter().all(|line| line.is_none()) {
            return Ok(());
        }

        // if the first runner has no line left, but others do, its an error
        if current_lines[0].is_none() {
            let (with_some, the_line) = current_lines[1..]
                .iter()
                .enumerate()
                .find_map(|(with_some, line)| line.as_ref().map(|l| (with_some, l)))
                .unwrap();

            return Err(format!(
                "runner 0 has no lines left, but runner {with_some} has additionally entry at time {}.",
                the_line.time
            )
            .into());
        }

        // we now know that the first line is some, and use it as a reference of the others
        let reference_line = current_lines[0].as_ref().unwrap();

        // if any other runner does not have a line left to compare to, its an error.
        if let Some(with_none) = current_lines
            .iter()
            .enumerate()
            .find_map(|(i, line)| line.is_none().then_some(i))
        {
            return Err(format!(
                "runner 0 has entry at time {}, but runner {with_none} is missing that line.",
                reference_line.time
            )
            .into());
        }

        // if both lines have different timestamps there is an error
        if let Some((wrong_runner, wrong_ts)) = current_lines
            .iter()
            .map(|s| s.as_ref().unwrap())
            .enumerate()
            .find(|(_, line)| line.time != reference_line.time)
        {
            return Err(format!(
                "runner 0 had next cycle at time {}, but runner {wrong_runner} had next at time {}.",
                reference_line.time, wrong_ts.time
            )
            .into());
        };

        // if there is any other line, that is different to the reference, its an error.
        // here, we know that all lines exist, so we can unwrap all lines.
        if let Some((wrong_runner, wrong_line)) = current_lines
            .iter()
            .map(|s| s.as_ref().unwrap())
            .enumerate()
            .find(|(_, line)| line.outputs != reference_line.outputs)
        {
            return Err(format!(
                "runner 0 expected {:?}, but runner {wrong_runner} had {:?} at time {}.",
                reference_line.outputs, wrong_line.outputs, reference_line.time
            )
            .into());
        }
    }
}
