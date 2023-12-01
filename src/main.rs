//! Advent of Code repo for 2023

use crate::utils::read_file;
mod utils;
mod day01;


fn main() {
    // Nothing to do, existing code already moved into tests.
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day01_test() {
        let input = read_file("day01.txt");
        assert_eq!(day01::part01(input.as_str()), 54388);
        assert_eq!(day01::part02(input.as_str()), 53515);
    }
}