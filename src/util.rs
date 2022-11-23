use std::fs::File;
use std::io::{self, BufRead};

/**
 * Reads lines from the given input txt file. Returning a Vector of Strings
 */
pub fn read_input(filename: &str) -> Vec<String> {
    File::open(filename)
        .map(io::BufReader::new).unwrap()
        .lines()
        .filter_map(Result::ok)
        .collect()
}

/**
 * Reads lines from the given input txt file. Returning a Vector of Ints
 */
pub fn read_input_ints(filename: &str) -> Vec<usize> {
    read_input(filename)
        .iter()
        .map(|x| x.parse::<usize>())
        .filter_map(Result::ok)
        .collect()
}