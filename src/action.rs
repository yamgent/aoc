// SPDX-License-Identifier: MIT
extern crate clap;
use crate::{files, runner};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

const SUBCMD_RUN: &str = "run";
const SUBCMD_RUN_PART: &str = "PART";
const SUBCMD_RUN_INPUT: &str = "INPUT";

const SUBCMD_WRITE: &str = "write";
const SUBCMD_WRITE_PART: &str = "PART";
const SUBCMD_WRITE_INPUT: &str = "INPUT";

const SUBCMD_TEST: &str = "test";
const SUBCMD_TEST_PART: &str = "PART";
const SUBCMD_TEST_INPUT: &str = "INPUT";

pub enum TestType {
    All,
    Specific { input: String },
}

pub enum Action {
    Run { part: String, input: String },
    Write { part: String, input: String },
    Test { part: String, test_type: TestType },
    NoOp,
}

impl Action {
    pub fn parse_args() -> Action {
        let matches = App::new(crate_name!())
            .about(crate_description!())
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                SubCommand::with_name(SUBCMD_RUN)
                    .about("execute part with input")
                    .arg(
                        Arg::with_name(SUBCMD_RUN_PART)
                            .help("which part to execute")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name(SUBCMD_RUN_INPUT)
                            .help("which input to feed in")
                            .index(2),
                    ),
            )
            .subcommand(
                SubCommand::with_name(SUBCMD_WRITE)
                    .about("write output after running part with input")
                    .arg(
                        Arg::with_name(SUBCMD_WRITE_PART)
                            .help("which part to execute")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name(SUBCMD_WRITE_INPUT)
                            .help("which input to feed in")
                            .index(2),
                    ),
            )
            .subcommand(
                SubCommand::with_name(SUBCMD_TEST)
                    .about("test each input for part")
                    .arg(
                        Arg::with_name(SUBCMD_TEST_PART)
                            .help("which part to test")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name(SUBCMD_TEST_INPUT)
                            .help("which particular test input to run")
                            .index(2),
                    ),
            )
            .get_matches();

        if let Some(matches) = matches.subcommand_matches(SUBCMD_RUN) {
            let part = matches.value_of(SUBCMD_RUN_PART).unwrap().to_string();
            let input = matches
                .value_of(SUBCMD_RUN_INPUT)
                .unwrap_or(&part)
                .to_string();
            Action::Run { part, input }
        } else if let Some(matches) = matches.subcommand_matches(SUBCMD_WRITE) {
            let part = matches.value_of(SUBCMD_WRITE_PART).unwrap().to_string();
            let input = matches
                .value_of(SUBCMD_WRITE_INPUT)
                .unwrap_or(&part)
                .to_string();
            Action::Write { part, input }
        } else if let Some(matches) = matches.subcommand_matches(SUBCMD_TEST) {
            let part = matches.value_of(SUBCMD_TEST_PART).unwrap().to_string();
            let test_type = match matches.value_of(SUBCMD_TEST_INPUT) {
                Some(input) => TestType::Specific {
                    input: input.to_string(),
                },
                None => TestType::All,
            };
            Action::Test { part, test_type }
        } else {
            Action::NoOp
        }
    }

    pub fn execute(self) {
        match self {
            Action::Run { part, input } => {
                let prog_filename = files::get_prog_filename(&part);
                let input_filename = files::get_input_filename(&input);
                let output = runner::run_python_prog(&prog_filename, &input_filename)
                    .unwrap_or_else(|err| panic!("{}", err));
                print!("{}", output);
            }
            Action::Write { part, input } => {
                let prog_filename = files::get_prog_filename(&part);
                let input_filename = files::get_input_filename(&input);
                let output_filename = files::get_output_filename(&input);
                let output = runner::run_python_prog(&prog_filename, &input_filename)
                    .unwrap_or_else(|err| panic!("{}", err));
                runner::write_output(&output_filename, &output)
                    .unwrap_or_else(|err| panic!("{}", err));
            }
            Action::Test { part, test_type } => {
                let prog_filename = files::get_prog_filename(&part);

                let test_cases = match test_type {
                    TestType::Specific { input } => vec![runner::TestCase {
                        input_filename: files::get_input_filename(&input),
                        expected_output_filename: files::get_output_filename(&input),
                    }],
                    TestType::All => {
                        let test_prefix = files::get_test_prefix(&part);
                        let test_files = files::get_all_input_filenames_with_prefix(&test_prefix)
                            .unwrap_or_else(|err| panic!("{}", err));
                        let mut test_files: Vec<runner::TestCase> = test_files
                            .iter()
                            .map(|f| runner::TestCase {
                                input_filename: f.to_string(),
                                expected_output_filename: files::get_output_filename(
                                    &files::remove_ext(f, files::INPUT_EXT),
                                ),
                            })
                            .collect();

                        // if live input and output are available,
                        // then let's add them too (this is optional)
                        let live_input = files::get_input_filename(&part);
                        let live_output = files::get_output_filename(&part);
                        if files::file_exists(&live_input) && files::file_exists(&live_output) {
                            test_files.push(runner::TestCase {
                                input_filename: live_input,
                                expected_output_filename: live_output,
                            });
                        }

                        test_files
                    }
                };

                let (mut success, mut failure, mut error) = (0, 0, 0);
                let test_results: Vec<(runner::TestCase, runner::TestResult)> = test_cases
                    .into_iter()
                    .map(|tc| {
                        let result = runner::run_test(&prog_filename, &tc);
                        match result {
                            runner::TestResult::Success => success += 1,
                            runner::TestResult::Failure => failure += 1,
                            _ => error += 1,
                        }
                        (tc, result)
                    })
                    .collect();

                for (tc, tres) in test_results {
                    println!("{}", runner::get_test_result_string(&tc, &tres));
                }
                println!();
                println!("{} success, {} failure, {} error", success, failure, error);
                if failure == 0 && error == 0 {
                    println!("All test cases passed.");
                } else {
                    println!("FAILED some test cases.");
                }
            }
            Action::NoOp => (),
        }
    }
}
