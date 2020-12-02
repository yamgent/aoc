// SPDX-License-Identifier: MIT
use std::env;

enum Action {
    Run { part: String },
    Test { part: String },
    TestOne { part: String, test: String },
    Invalid { message: String },
}

fn parse_args(args: &[String]) -> Action {
    if args.len() < 2 {
        return Action::Invalid {
            message: String::from("No commands specified"),
        };
    }

    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                Action::Invalid {
                    message: String::from("run: Missing <part>"),
                }
            } else if args.len() > 3 {
                Action::Invalid {
                    message: String::from("run: Too many arguments"),
                }
            } else {
                Action::Run {
                    part: args[2].clone(),
                }
            }
        }
        "test" => {
            if args.len() < 3 {
                Action::Invalid {
                    message: String::from("test: Missing <part> [test]"),
                }
            } else if args.len() == 3 {
                Action::Test {
                    part: args[2].clone(),
                }
            } else if args.len() == 4 {
                Action::TestOne {
                    part: args[2].clone(),
                    test: args[3].clone(),
                }
            } else {
                Action::Invalid {
                    message: String::from("test: Too many arguments"),
                }
            }
        }
        _ => Action::Invalid {
            message: format!("Unknown command {}", args[1]),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_args(&args) {
        Action::Invalid { message } => {
            eprintln!("Error: {}", message);
        }
        Action::Run { part } => {
            // TODO: Finish this
            println!("Running {}", part);
        }
        Action::Test { part } => {
            // TODO: Finish this
            println!("Testing {}", part);
        }
        Action::TestOne { part, test } => {
            // TODO: Finish this
            println!("Testing one {} - {}", part, test);
        }
    }
}
