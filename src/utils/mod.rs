use std::fs;

pub fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|val| val.into())
        .collect()
}
