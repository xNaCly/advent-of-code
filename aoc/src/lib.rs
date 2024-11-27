use std::fs;

pub fn lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap_or_default()
        .lines()
        .map(String::from)
        .collect()
}
