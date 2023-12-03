//! Advent of Code repo for 2023

use std::collections::HashMap;

use crate::utils::read_file;
mod day01;
mod day02;
mod day03;
mod utils;

fn main() {
    // Nothing to do, existing code already moved into tests.
    // solve_day02();
    // let input = read_file("day03_test.txt");
}

fn solve_day02() {
    // First parse step of example using "Game ([0-9]+): .*" regex
    // let example_line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let input = read_file("day02.txt");

    let mut index_total = 0;
    let mut total_power = 0;
    for line in input.lines() {
        let (i, r, g, b) = get_color_maxes(line);
        if r <= 12 && g <= 13 && b <= 14 {
            index_total += i;
        }
        total_power += r * g * b;
    }
    println!("Total: {}", index_total);
    // 2176
    println!("Total power: {}", total_power);
    // 63700
}

/// Parses a line of the input file. Example:
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
fn get_color_maxes(example_line: &str) -> (i32, i32, i32, i32) {
    let re = regex::Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let caps = re.captures(example_line).unwrap();
    let game_number = caps.get(1).unwrap().as_str();
    let second_part = caps.get(2).unwrap().as_str();

    // Aggregate maximum number of each color
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    // Split the second part by "; " to get the rounds
    let rounds = second_part.split(';');
    // For each round, split by ", " to get the cubes
    for round in rounds {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes = round.split(", ");
        for cube in cubes {
            // Parse with regex: ([0-9]+) (red|green|blue)
            let re = regex::Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
            let caps = re.captures(cube).unwrap();
            let number = caps.get(1).unwrap().as_str();
            let color = caps.get(2).unwrap().as_str();

            match color {
                "red" => red += number.parse::<i32>().unwrap(),
                "green" => green += number.parse::<i32>().unwrap(),
                "blue" => blue += number.parse::<i32>().unwrap(),
                _ => panic!("Unknown color: {}", color),
            }
        }
        if red > max_red {
            max_red = red;
        }
        if green > max_green {
            max_green = green;
        }
        if blue > max_blue {
            max_blue = blue;
        }
    }
    (game_number.parse().unwrap(), max_red, max_green, max_blue)
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

    #[test]
    fn day03_test() {
        assert_eq!(
            day03::both_parts(&read_file("day03_test.txt")),
            (4361, 467835)
        );
        assert_eq!(
            day03::both_parts(&read_file("day03.txt")),
            (532428, 84051670)
        );
    }
}
