use std::path::PathBuf;

use console::Style;
use lazy_static::lazy_static;
use terminal_size::terminal_size;

use crate::{
    test_case::{NoResult, TestCase},
    RunnerArg,
};

lazy_static! {
    pub(crate) static ref PASSED_STYLE: Style = Style::new().green();
    pub(crate) static ref PASSED_TEXT: String = PASSED_STYLE.apply_to("[PASSED]").to_string();
    pub(crate) static ref FAILED_STYLE: Style = Style::new().red();
    pub(crate) static ref FAILED_TEXT: String = FAILED_STYLE.apply_to("[FAILED]").to_string();
    pub(crate) static ref DEBUG_STYLE: Style = Style::new().dim();
    pub(crate) static ref TERMINAL_WIDTH: usize =
        terminal_size().map(|w| w.0 .0 as usize).unwrap_or(60);
}

pub(crate) fn run_tests(tests: Vec<TestCase<NoResult>>, runner: Vec<(RunnerArg, PathBuf)>) -> bool {
    let separator = "-".repeat(*TERMINAL_WIDTH);

    // print general information about all the tests that are going to run.
    let number_of_specs = tests.len();
    let total_number_of_tests: usize = tests.iter().map(|test| test.total_num_tests()).sum();
    println!("Running {number_of_specs} tests ({total_number_of_tests} traces).");
    println!("{}", separator);
    println!("Comparing results of:");
    for (i, (runner, runner_binary)) in runner.iter().enumerate() {
        println!("   {}. {runner}: {}", i, runner_binary.display());
    }
    println!("{}", separator);

    let results = tests
        .into_iter()
        // run each test case
        .map(|test| test.run(&runner))
        // and print the result
        .inspect(|result| result.print_result())
        .collect::<Vec<_>>();

    let number_specs_passed = results.iter().filter(|result| result.passed()).count();
    let number_total_passed: usize = results.iter().map(|result| result.num_passed_tests()).sum();

    let all_passed = results.iter().all(|result| result.passed());

    let result_style = if all_passed {
        &*PASSED_STYLE
    } else {
        &*FAILED_STYLE
    };

    let result_msg = format!("Passed {number_specs_passed}/{number_of_specs} tests ({number_total_passed}/{total_number_of_tests} traces).");
    let result_msg = result_style.apply_to(result_msg);

    println!("{}", separator);
    println!("{}", result_msg);

    all_passed
}
