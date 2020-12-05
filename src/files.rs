// SPDX-License-Identifier: MIT
use std::fs;
use std::path::Path;

pub const PY_EXT: &str = ".py";
pub const INPUT_EXT: &str = ".txt";
pub const OUTPUT_EXT: &str = ".out.txt";
pub const TEST_PREFIX: &str = "t";

pub fn get_prog_filename(name: &str) -> String {
    format!("{}{}", name, PY_EXT)
}

pub fn get_input_filename(name: &str) -> String {
    format!("{}{}", name, INPUT_EXT)
}

pub fn get_output_filename(name: &str) -> String {
    format!("{}{}", name, OUTPUT_EXT)
}

pub fn get_test_prefix(name: &str) -> String {
    format!("{}{}", TEST_PREFIX, name)
}

pub fn remove_ext(filename: &str, ext: &str) -> String {
    match filename.strip_suffix(ext) {
        Some(val) => val.to_string(),
        None => filename.to_string(),
    }
}

pub fn file_exists(filename: &str) -> bool {
    Path::new(&format!("./{}", filename)).exists()
}

pub fn get_all_input_filenames_with_prefix(prefix: &str) -> Result<Vec<String>, String> {
    let mut result = vec![];

    let entries = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(err) => return Err(format!("Fail to read file listing for cwd. {}", err)),
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let filename = entry.file_name();
            let filename = filename.to_str().unwrap_or("");
            if filename.starts_with(prefix)
                && filename.ends_with(INPUT_EXT)
                && !filename.ends_with(OUTPUT_EXT)
            {
                result.push(filename.to_string());
            }
        }
    }

    // filesystem does not guarantee files are in any particular order
    result.sort();

    Ok(result)
}
