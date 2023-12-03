//! Advent of Code repo for 2023

use std::collections::HashMap;

use crate::utils::read_file;
mod day01;
mod day02;
mod utils;

fn main() {
    // Nothing to do, existing code already moved into tests.
    // solve_day02();
    // let input = read_file("day03_test.txt"); // 4361 is right
    let input = read_file("day03.txt");
    // I first made a mistake where I didn't catch numbers that would go against
    // the right edge of the grid. I fixed it by adding a check for the last number
    // in each line. (After inner loop over x, before incrementing y.)

    let mut symbol_map: HashMap<(i32, i32), char> = HashMap::new();
    let mut gear_ratio_map: HashMap<(i32, i32), (u64, u64)> = HashMap::new();

    // Scan everything once to put all symbols into a map
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            print!("{} ", c);
            if !"0123456789.".contains(c) {
                symbol_map.insert((x as i32, y as i32), c);
            }
        }
        println!();
    }

    println!("Symbol map: {:?}", symbol_map);

    let mut accumulator: i64 = 0;
    // Scan again to find all numbers. A number accumulates digits until a non-digit is found.
    for (y, line) in input.lines().enumerate() {
        let mut number = 0;
        let mut is_part = false;
        let mut last_x_seen = 0;
        let mut number_left_index = 0;
        let mut number_right_index = 0;
        for (x, c) in line.chars().enumerate() {
            last_x_seen = x;
            if let Some(digit) = c.to_digit(10) {
                if number == 0 {
                    number_left_index = x;
                }
                number = number * 10 + digit as i64;
                is_part = is_part || symbol_in_nbhd(&symbol_map, x as i32, y as i32);
            } else if number > 0 {
                number_right_index = x - 1;

                println!(
                    "Found number {} at (({}-{}), {})",
                    number, number_left_index, number_right_index, y
                );
                if is_part {
                    accumulator += number;
                    println!("Accumulator now at: {}", accumulator);
                }

                update_gear_ratio_map(
                    &mut gear_ratio_map,
                    number,
                    number_left_index as i32,
                    number_right_index as i32,
                    y as i32,
                );

                number = 0;
                is_part = false;
            }
        }
        if number > 0 {
            number_right_index = last_x_seen;
            println!(
                "Found number {} at (({}-{}), {})",
                number, number_left_index, number_right_index, y
            );
            if is_part {
                accumulator += number;
                println!("Accumulator now at: {}", accumulator);
            }

            update_gear_ratio_map(
                &mut gear_ratio_map,
                number,
                number_left_index as i32,
                number_right_index as i32,
                y as i32,
            );

            number = 0;
            is_part = false;
        }
    }

    // Part 1
    println!("Accumulator: {}", accumulator);

    // Part 2
    let mut gear_ratio_sum = 0;
    for (key, value) in gear_ratio_map.iter() {
        if symbol_map.contains_key(key) {
            if value.1 == 2 {
                gear_ratio_sum += value.0;
            }
        }
    }
    println!("Gear ratio sum: {}", gear_ratio_sum);
}

fn update_gear_ratio_map(
    gear_ratio_map: &mut HashMap<(i32, i32), (u64, u64)>,
    number: i64,
    number_left_index: i32,
    number_right_index: i32,
    y: i32,
) {
    for x in (number_left_index - 1)..=(number_right_index + 1) {
        for dy in -1..=1 {
            // use entry api to update the value
            // Each entry is (product, count)
            let entry = gear_ratio_map.entry((x, y + dy)).or_insert((1, 0));
            entry.0 *= number as u64;
            entry.1 += 1;
        }
    }
}

fn symbol_in_nbhd(symbol_map: &HashMap<(i32, i32), char>, x: i32, y: i32) -> bool {
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if symbol_map.contains_key(&(x + dx, y + dy)) {
                return true;
            }
        }
    }
    false
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
}
