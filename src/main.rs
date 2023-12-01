//! Advent of Code repo for 2023

use crate::utils::read_file;
mod utils;
use regex;

fn main() {
    let input = read_file("day01.txt");
    // let input = read_file("day01_test.txt");

    let mut total = 0;
    for line in input.lines() {
        total += find_callibration_value(line);
    }
    println!("Total: {}", total);

    // Part 2
    // It is possible, that letters may overlap!
    // sxoneightoneckk9ldctxxnffqnzmjqvj
    let mut total = 0;
    for line in input.lines() {
        total += first_last(parse_all_digits(line));
    }
    println!("Total: {}", total);
}

fn first_last(digits: Vec<u32>) -> u32 {
    let first = digits[0];
    let last = digits[digits.len() - 1];
    10 * first + last
}

/// Use the following regex to find all digits in a line:
/// [1-9]|zero|one|two|three|four|five|six|seven|eight|nine
/// And the spelled out digits get transformed to numbers.
fn parse_all_digits(mut line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    'outer: while !line.is_empty() {
        // print!("{} ", line);
        let re =
            regex::Regex::new(r"[1-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let mut captures_iter = re.captures_iter(line);
        if let Some(cap) = captures_iter.next() {
            let mut next_start_index = 0;
            let digit = match cap.get(0) {
                Some(m) => {
                    next_start_index = m.start() + 1;
                    m.as_str()
                }
                None => panic!("No match"),
            };
            match digit {
                "zero" => digits.push(0),
                "one" => digits.push(1),
                "two" => digits.push(2),
                "three" => digits.push(3),
                "four" => digits.push(4),
                "five" => digits.push(5),
                "six" => digits.push(6),
                "seven" => digits.push(7),
                "eight" => digits.push(8),
                "nine" => digits.push(9),
                _ => digits.push(digit.parse::<u32>().unwrap()),
            }
            if next_start_index == 0 {
                return digits;
            }
            let (_, rest) = line.split_at(next_start_index);
            line = rest;
        } else {
            break 'outer;
        }
    }
    digits
}

/// On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
fn find_callibration_value(line: &str) -> i32 {
    // "1abc2" -> 12
    // "pqr3stu8vwx" -> 38

    let chars: Vec<_> = line.chars().flat_map(|c| c.to_digit(10)).collect();

    // println!("{:?}", chars);

    let first = chars[0];
    let last = chars[chars.len() - 1];
    10 * first as i32 + last as i32
}
