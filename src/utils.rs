//! Shared utility functions

use std::{fs::File, io::Read};

/// Read a file to a string.
pub fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(&format!("input/{}", filename)).unwrap();
    file.read_to_string(&mut input).unwrap();
    input
}