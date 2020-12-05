pub fn get_prog_filename(name: &str) -> String {
    format!("{}.py", name)
}

pub fn get_input_filename(name: &str) -> String {
    format!("{}.txt", name)
}

pub fn get_output_filename(name: &str) -> String {
    format!("{}.out.txt", name)
}
