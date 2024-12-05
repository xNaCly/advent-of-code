use std::fs;

/// 2d grid interaction abstraction, which are common in aoc problems.
pub mod grid;
/// (x,y) abstraction to interact with crate::grid
pub mod point;

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
