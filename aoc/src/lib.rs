use std::fs;

pub fn lines_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap_or_default()
        .lines()
        .map(String::from)
        .collect()
}

pub fn lines_str(s: &str) -> Vec<String> {
    String::from(s).lines().map(String::from).collect()
}
