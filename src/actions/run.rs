use std::fs;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn get_python_name() -> &'static str {
    if cfg!(windows) {
        "py"
    } else {
        "python3"
    }
}

pub fn execute(part: String) {
    let input_file_name = format!("{}.txt", part);
    let prog_file_name = format!("{}.py", part);

    let input = fs::read_to_string(&input_file_name)
        .unwrap_or_else(|err| panic!("Cannot open {}. {}", input_file_name, err));

    let process = Command::new(get_python_name())
        .arg(prog_file_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("Cannot execute python: {}", err));

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

    print!("{}", output);
}
