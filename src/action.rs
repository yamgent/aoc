// SPDX-License-Identifier: MIT
extern crate clap;
use crate::runner;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

const SUBCMD_RUN: &str = "run";
const SUBCMD_RUN_PART: &str = "PART";

pub enum Action {
    Run { part: String },
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
                            .required(true),
                    ),
            )
            .get_matches();

        if let Some(matches) = matches.subcommand_matches(SUBCMD_RUN) {
            Action::Run {
                part: matches.value_of(SUBCMD_RUN_PART).unwrap().to_string(),
            }
        } else {
            Action::NoOp
        }
    }

    pub fn execute(self) {
        match self {
            Action::Run { part } => {
                let input_filename = format!("{}.txt", part);
                let prog_filename = format!("{}.py", part);
                let output = runner::run_python_prog(&prog_filename, &input_filename);
                print!("{}", output);
            }
            Action::NoOp => (),
        }
    }
}
