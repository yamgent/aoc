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
