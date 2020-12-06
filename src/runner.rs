// SPDX-License-Identifier: MIT
use crate::files;
use std::fs;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn get_python_name() -> &'static str {
    if cfg!(windows) {
        "py"
    } else {
        "python3"
    }
}

pub fn run_python_prog(prog_filename: &str, input_filename: &str) -> Result<String, String> {
    let input = match fs::read_to_string(&input_filename) {
        Ok(content) => content,
        Err(err) => return Err(format!("Cannot open {}. {}", input_filename, err)),
    };

    let process = match Command::new(get_python_name())
        .arg(prog_filename)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(res) => res,
        Err(err) => return Err(format!("Cannot execute python: {}", err)),
    };

    process
        .stdin
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap_or_else(|err| panic!("Cannot send input to python stdin. {}", err));

    let mut output = String::new();
    process
        .stdout
        .unwrap()
        .read_to_string(&mut output)
        .unwrap_or_else(|err| panic!("Cannot receive python output. {}", err));

    Ok(output)
}

pub fn write_output(output_filename: &str, output: &str) -> Result<(), String> {
    match fs::write(&output_filename, output) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Cannot write to file {}. {}", output_filename, err)),
    }
}

pub struct TestCase {
    pub input_filename: String,
    pub expected_output_filename: String,
}

pub enum TestResult {
    Success,
    Failure { actual: String, expected: String },
    NoInputFile,
    NoOutputFile,
    RunError { error: String },
}

pub fn run_test(prog_filename: &str, test_case: &TestCase) -> TestResult {
    if !files::file_exists(&test_case.input_filename) {
        return TestResult::NoInputFile;
    }

    if !files::file_exists(&test_case.expected_output_filename) {
        return TestResult::NoOutputFile;
    }

    let actual_output = match run_python_prog(&prog_filename, &test_case.input_filename) {
        Ok(output) => output,
        Err(err) => return TestResult::RunError { error: err },
    };

    let expected_output = match fs::read_to_string(&test_case.expected_output_filename) {
        Ok(output) => output,
        Err(err) => {
            return TestResult::RunError {
                error: format!(
                    "Cannot read output file {}. {}",
                    &test_case.expected_output_filename, err
                ),
            }
        }
    };

    if actual_output != expected_output {
        return TestResult::Failure {
            actual: actual_output,
            expected: expected_output,
        };
    }
    TestResult::Success
}

pub fn get_test_result_string(test_case: &TestCase, test_result: &TestResult) -> String {
    let result_string = match test_result {
        TestResult::Success => "SUCCESS",
        TestResult::Failure { .. } => "FAILURE",
        TestResult::NoInputFile => "INPUT-MISSING",
        TestResult::NoOutputFile => "OUTPUT-MISSING",
        TestResult::RunError { .. } => "RUN-ERROR",
    };
    let icon = match test_result {
        TestResult::Success => "   ",
        TestResult::Failure { .. } => "(X)",
        _ => "(!)",
    };

    format!("{} {} {}", icon, test_case.input_filename, result_string)
}

pub fn get_diff(test_case: &TestCase, test_result: &TestResult) -> String {
    match test_result {
        TestResult::Failure { actual, expected } => vec![
            format!(
                "===== DIFF ({}) =====",
                test_case.input_filename.to_string()
            ),
            "".to_string(),
            "### Actual:".to_string(),
            actual.to_string(),
            "### Expected:".to_string(),
            expected.to_string(),
            "==== END ====".to_string(),
        ]
        .join("\n"),
        _ => {
            panic!("Should not call get_diff() on non-failure cases.");
        }
    }
}
