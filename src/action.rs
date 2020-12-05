// SPDX-License-Identifier: MIT
extern crate clap;
use crate::runner;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

const SUBCMD_RUN: &str = "run";
const SUBCMD_RUN_PART: &str = "PART";
const SUBCMD_RUN_INPUT: &str = "INPUT";

const SUBCMD_WRITE: &str = "write";
const SUBCMD_WRITE_PART: &str = "PART";
const SUBCMD_WRITE_INPUT: &str = "INPUT";

pub enum Action {
    Run { part: String, input: String },
    Write { part: String, input: String },
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
        } else {
            Action::NoOp
        }
    }

    pub fn execute(self) {
        match self {
            Action::Run { part, input } => {
                let prog_filename = format!("{}.py", part);
                let input_filename = format!("{}.txt", input);
                let output = runner::run_python_prog(&prog_filename, &input_filename);
                print!("{}", output);
            }
            Action::Write { part, input } => {
                let prog_filename = format!("{}.py", part);
                let input_filename = format!("{}.txt", input);
                let output_filename = format!("{}.out.txt", input);
                let output = runner::run_python_prog(&prog_filename, &input_filename);
                runner::write_output_safe(&output_filename, &output);
            }
            Action::NoOp => (),
        }
    }
}
