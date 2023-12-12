//! Advent of Code repo for 2023

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::Add,
    result,
};

use crate::utils::read_file;
mod day01;
mod day02;
mod day03;
// mod day07;
mod utils;

use regex;

fn main() {
    // Nothing to do, existing code already moved into tests.
    let input = read_file("day12.txt");

    solve_day12(input);
}

fn solve_day12(input: String) {
    // First, parse the input. It looks like this:

    // ???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1

    let mut solution_count_sum = 0;

    // We parse this into Vec<HotSpringRow>:
    for line in input.lines() {
        // Split line along the single whitespace
        let mut parts = line.split(' ');
        let state_str = parts.next().unwrap();
        let contiguous_broken_str = parts.next().unwrap();

        let mut state = Vec::new();

        for char in state_str.chars() {
            match char {
                '.' => {
                    state.push(HotSpringState::Operational);
                }
                '#' => {
                    state.push(HotSpringState::Damaged);
                }
                '?' => {
                    state.push(HotSpringState::Unknown);
                }
                _ => panic!("Invalid character: {}", char),
            }
        }
        // Now we have a state. Parse the contiguous broken numbers.
        let contiguous_broken = contiguous_broken_str
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let row = HotSpringRow {
            state,
            contiguous_broken,
        };

        println!("Row: {:?}", row);
        let solution_count = count_solutions(&row, 0);
        println!("Solution count: {}", solution_count);
        solution_count_sum += solution_count;
    }
    println!("Solution count sum: {}", solution_count_sum);
}

fn count_solutions(row: &HotSpringRow, at: usize) -> usize {
    // Use backtracking to count the number of solutions.
    // Check if in the current position, we are somewhat consistent with the contiguous broken numbers.
    if at == row.state.len() {
        // We are at the end of the row. Verify the solution.
        if verify_solution(row) {
            return 1;
        } else {
            return 0;
        }
    } else if row.state[at] != HotSpringState::Unknown {
        if at + 1 < row.state.len() {
            return count_solutions(row, at + 1);
        } else {
            // Verify solution
            if verify_solution(row) {
                return 1;
            } else {
                return 0;
            }
        }
    } else {
        let mut clone = row.clone();
        clone.state[at] = HotSpringState::Operational;
        let mut count = count_solutions(&clone, at + 1);
        clone.state[at] = HotSpringState::Damaged;
        count += count_solutions(&clone, at + 1);
        count
    }
}

/// Takes a HotSpringRow without any unknowns and verifies that it is a valid solution.
fn verify_solution(row: &HotSpringRow) -> bool {
    let mut runs = Vec::new();
    let mut current_run = 0;
    let mut at = 0;
    while at < row.state.len() {
        match row.state[at] {
            HotSpringState::Damaged => {
                current_run += 1;
            }
            HotSpringState::Operational => {
                if (current_run > 0) {
                    runs.push(current_run);
                    current_run = 0;
                }
            }
            HotSpringState::Unknown => panic!("Unknown state in verify_solution"),
        }
        at += 1;
    }

    if (current_run > 0) {
        runs.push(current_run);
        current_run = 0;
    }

    runs == row.contiguous_broken
}

#[derive(Debug, Clone)]
struct HotSpringRow {
    state: Vec<HotSpringState>,
    contiguous_broken: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HotSpringState {
    Operational, // '.'
    Damaged,     // '#'
    Unknown,     // '?'
}

fn solve_day10(input: String) {
    use Direction::*;
    // Input looks like this
    // ..F7.
    // .FJ|.
    // SJ.L7
    // |F--J
    // LJ...

    // Read this into a HashMap<(i32, i32), char>
    // Also find the start position which holds the 'S' character.
    let mut max_x = 0;
    let mut max_y = 0;

    let mut map = HashMap::new();
    let mut start_position = (0, 0);
    for (y, line) in input.lines().enumerate() {
        max_y = max(max_y, y);
        for (x, char) in line.chars().enumerate() {
            max_x = max(max_x, x);
            map.insert((x as i32, y as i32), char);
            if char == 'S' {
                start_position = (x as i32, y as i32);
            }
        }
    }

    println!("Map: {:?}", map);
    println!("Start position: {:?}", start_position);

    // Walk along the pipe network, starting at 'S' until we reach 'S' again.
    let mut direction = Direction::Right;
    // Checked manually
    let mut current_position = start_position + direction;
    let mut distance_walked = 1;

    // To find the area integrate over the border of the loop using the form
    // y*dx. Since we walk right at the start, we start with y * dx = start_position.1 * 1.
    let mut gaussian_integral = start_position.1 * 1;

    while current_position != start_position {
        println!(
            "Current position: {:?}, after {}",
            current_position, distance_walked
        );
        println!("Current character: {:?}", map.get(&current_position));
        direction = next_direction(direction, current_position, &map);

        // Integrate over the border of the loop
        match direction {
            Right => gaussian_integral += current_position.1, // dx = 1
            Left => gaussian_integral -= current_position.1,  // dx = -1
            Up => {}                                          // dx = 0
            Down => {}                                        // dx = 0
        }

        current_position = current_position + direction;
        distance_walked += 1;
    }
    println!("Distance walked: {}", distance_walked);
    println!("Half of that is: {}", distance_walked / 2);
    // 6690

    let loop_length = distance_walked;

    let enclosed_squares = gaussian_integral - (distance_walked / 2) + 1;
    println!("Enclosed squares: {}", enclosed_squares);
    // 525
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, other: Direction) -> (i32, i32) {
        use Direction::*;
        match other {
            Up => (self.0, self.1 - 1),
            Down => (self.0, self.1 + 1),
            Left => (self.0 - 1, self.1),
            Right => (self.0 + 1, self.1),
        }
    }
}

fn next_direction(
    previous_direction: Direction,
    current_position: (i32, i32),
    map: &HashMap<(i32, i32), char>,
) -> Direction {
    use Direction::*;
    match map.get(&current_position).unwrap() {
        '|' | '-' => previous_direction,
        'L' => match previous_direction {
            Down => Right,
            Left => Up,
            _ => panic!("Invalid previous direction for L: {:?}", previous_direction),
        },
        'J' => match previous_direction {
            Down => Left,
            Right => Up,
            _ => panic!("Invalid previous direction for J: {:?}", previous_direction),
        },
        'F' => match previous_direction {
            Up => Right,
            Left => Down,
            _ => panic!("Invalid previous direction for F: {:?}", previous_direction),
        },
        '7' => match previous_direction {
            Up => Left,
            Right => Down,
            _ => panic!("Invalid previous direction for 7: {:?}", previous_direction),
        },
        _ => panic!("Invalid character: {}", map.get(&current_position).unwrap()),
    }
}

fn solve_day09(input: String) {
    // The input contains lines. Iterate over all lines and parse it as a list
    // of numbers separated by spaces.
    let mut sum_of_next = 0;
    let mut sum_of_previous = 0;
    let mut rows = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for number in line.split(' ') {
            row.push(number.parse::<i128>().unwrap());
        }

        // Now we have a row. Find the next number.
        let next = find_next(&row);
        println!("Next: {}", next);
        sum_of_next += next;

        // Find the previous number.
        let previous = find_previous(&row);
        println!("Previous: {}", previous);
        sum_of_previous += previous;

        rows.push(row);
    }
    println!("Rows: {:?}", rows);
    println!("Sum of next: {}", sum_of_next);
    println!("Sum of previous: {}", sum_of_previous);
}

fn find_next(row: &[i128]) -> i128 {
    // First, calculate a vector of differences between each entry and the next.
    // Track if all are zero.
    let mut all_zero = true;
    let mut differences = Vec::with_capacity(row.len() - 1);
    for i in 0..(row.len() - 1) {
        let difference = row[i + 1] - row[i];
        if difference != 0 {
            all_zero = false;
        }
        differences.push(difference);
    }
    if (all_zero) {
        // All differences are zero. The next entry is simply the last entry.
        row[row.len() - 1]
    } else {
        // Otherwise, recurse on the differences.
        let next_difference = find_next(&differences);
        // The next entry is the last entry plus the next difference.
        row[row.len() - 1] + next_difference
    }
}

fn find_previous(row: &[i128]) -> i128 {
    // First, calculate a vector of differences between each entry and the next.
    // Track if all are zero.
    let mut all_zero = true;
    let mut differences = Vec::with_capacity(row.len() - 1);
    for i in 0..(row.len() - 1) {
        let difference = row[i + 1] - row[i];
        if difference != 0 {
            all_zero = false;
        }
        differences.push(difference);
    }
    if (all_zero) {
        // All differences are zero. The previous entry is simply the first entry.
        row[0]
    } else {
        // Otherwise, recurse on the differences.
        let previous_difference = find_previous(&differences);
        // The previous entry is the first entry minus the previous difference.
        row[0] - previous_difference
    }
}

fn solve_day_05(input: String) {
    let mut lines = input.lines();
    // The first line is for seeds:
    // seeds: 79 14 55 13
    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    println!("Seeds: {:?}", seeds);

    // Discard lines up to (and including) the line "seed-to-soil map:"
    while lines.next().unwrap() != "seed-to-soil map:" {}
    let seed_to_soil = parse_map_until_blank(&mut lines);

    // Discard lines up to (and including) the line "soil-to-fertilizer map:"
    while lines.next().unwrap() != "soil-to-fertilizer map:" {}
    let soil_to_fertilizer = parse_map_until_blank(&mut lines);

    // Discard lines up to (and including) the line "fertilizer-to-water map:"
    while lines.next().unwrap() != "fertilizer-to-water map:" {}
    let fertilizer_to_water = parse_map_until_blank(&mut lines);

    // Discard lines up to (and including) the line "water-to-light map:"
    while lines.next().unwrap() != "water-to-light map:" {}
    let water_to_light = parse_map_until_blank(&mut lines);

    // Discard lines up to (and including) the line "light-to-temperature map:"
    while lines.next().unwrap() != "light-to-temperature map:" {}
    let light_to_temperature = parse_map_until_blank(&mut lines);

    // Discard lines up to (and including) the line "temperature-to-humidity map:"
    while lines.next().unwrap() != "temperature-to-humidity map:" {}
    let temperature_to_humidity = parse_map_until_blank(&mut lines);

    // Discard lines up to (and including) the line "humidity-to-location map:"
    while lines.next().unwrap() != "humidity-to-location map:" {}
    let humidity_to_location = parse_map_until_blank(&mut lines);

    // All maps parsed. Now we can start calculating.

    // First, print all the maps
    // println!("Seeds: {:?}", seeds);
    println!("Seed to soil: {:?}", seed_to_soil);
    // println!("Soil to fertilizer: {:?}", soil_to_fertilizer);
    // println!("Fertilizer to water: {:?}", fertilizer_to_water);
    // println!("Water to light: {:?}", water_to_light);
    // println!("Light to temperature: {:?}", light_to_temperature);
    // println!("Temperature to humidity: {:?}", temperature_to_humidity);
    // println!("Humidity to location: {:?}", humidity_to_location);

    // For each seed, map though all the maps, producing an untput line like:
    // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.

    // println!("{}", water_to_light.map(81));

    /*
    let mut lowest_location = u128::MAX;

    for seed in seeds {
        let soil = seed_to_soil.map(seed);
        let fertilizer = soil_to_fertilizer.map(soil);
        let water = fertilizer_to_water.map(fertilizer);
        let light = water_to_light.map(water);
        let temperature = light_to_temperature.map(light);
        let humidity = temperature_to_humidity.map(temperature);
        let location = humidity_to_location.map(humidity);
        println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}.", seed, soil, fertilizer, water, light, temperature, humidity, location);

        if location < lowest_location {
            lowest_location = location;
        }
    }

    println!("Lowest location: {}", lowest_location);
    */

    // Part 2: Same again, but use ranges this time.
    // The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range.
    // 79 14 55 13 -> (79, 14), (55, 13)
    let mut seed_ranges = Vec::new();
    for i in 0..seeds.len() / 2 {
        seed_ranges.push((seeds[i * 2], seeds[i * 2 + 1]));
    }
    println!("Seed ranges: {:?}", seed_ranges);

    // Flat Map thought all the maps, outputting along the way
    let soil_ranges = seed_ranges
        .iter()
        .flat_map(|(start, length)| seed_to_soil.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Soil ranges: {:?}", soil_ranges);

    let fertilizer_ranges = soil_ranges
        .iter()
        .flat_map(|(start, length)| soil_to_fertilizer.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Fertilizer ranges: {:?}", fertilizer_ranges);

    let water_ranges = fertilizer_ranges
        .iter()
        .flat_map(|(start, length)| fertilizer_to_water.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Water ranges: {:?}", water_ranges);

    let light_ranges = water_ranges
        .iter()
        .flat_map(|(start, length)| water_to_light.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Light ranges: {:?}", light_ranges);

    let temperature_ranges = light_ranges
        .iter()
        .flat_map(|(start, length)| light_to_temperature.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Temperature ranges: {:?}", temperature_ranges);

    let humidity_ranges = temperature_ranges
        .iter()
        .flat_map(|(start, length)| temperature_to_humidity.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Humidity ranges: {:?}", humidity_ranges);

    let location_ranges = humidity_ranges
        .iter()
        .flat_map(|(start, length)| humidity_to_location.map_range((*start, *length)))
        .collect::<Vec<(u128, u128)>>();
    println!("Location ranges: {:?}", location_ranges);

    // Find the lowest location
    let mut lowest_location = u128::MAX;
    for (start, length) in location_ranges {
        if start < lowest_location {
            lowest_location = start;
        }
    }
    println!("Lowest location: {}", lowest_location);
}

fn parse_map_until_blank(lines: &mut std::str::Lines<'_>) -> AdventMap {
    // Parse and advent mapping
    // [destination] [sourge range start] [source range length]
    // 2824905526 2969131334 898611144
    // 0 322319732 9776277
    // Do this until you reach an empty line
    let mut seed_to_soil = AdventMap::default();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let parts = line
            .split(' ')
            .map(|s| s.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        seed_to_soil.map_sections.push(MapSection {
            destination: parts[0],
            source_range_start: parts[1],
            source_range_length: parts[2],
        });
    }
    seed_to_soil
        .map_sections
        .sort_by_key(|s| s.source_range_start);

    seed_to_soil
}

/// A map section like "50 98 2" means, that 98 -> 50, 99 -> 51.
#[derive(Debug)]
struct MapSection {
    destination: u128,
    source_range_start: u128,
    source_range_length: u128,
}

#[derive(Default, Debug)]
struct AdventMap {
    map_sections: Vec<MapSection>,
}

impl AdventMap {
    fn map(&self, source: u128) -> u128 {
        // Find the map section that contains the source
        let mut index = 0;
        while index < self.map_sections.len()
            && self.map_sections[index].source_range_start
                + self.map_sections[index].source_range_length
                - 1
                < source
        {
            index += 1;
        }
        if index == self.map_sections.len() {
            return source; // No mapping defined
        }
        // println!("Index: {}, mappiing {:?}", index, self.map_sections[index]);
        if self.map_sections[index].source_range_start
            + self.map_sections[index].source_range_length
            > source
            && self.map_sections[index].source_range_start <= source
        {
            return (self.map_sections[index].destination + source)
                - self.map_sections[index].source_range_start;
        }
        return source; // No mapping defined
    }

    /// Ranges may map to multiple ranges, if they overlap with more than one map section.
    fn map_range(&self, interval: (u128, u128)) -> Vec<(u128, u128)> {
        let mut result = Vec::new();

        let mut remaining_interval = interval;
        for section in &self.map_sections {
            let (left_interval, intersecting_interval, new_remaining_interval) =
                mapOneSection(section, remaining_interval);

            result.extend(left_interval);
            result.extend(intersecting_interval);

            if let Some(new_remaining_interval) = new_remaining_interval {
                remaining_interval = new_remaining_interval;
            } else {
                return result;
            }
        }

        result.push(remaining_interval);

        result
    }
}

/// Takes a section and an interval and returns 0-2 intervals in the first two options and 0 or 1 unmapped intervals in the third option.
/// result.0 - interval fully before the section, already remapped (no-op).
/// result.1 - interval fully inside the section, already remapped by section.
/// result.2 - interval fully after the section, may be mapped by other sections.
fn mapOneSection(
    section: &MapSection,
    (start, length): (u128, u128),
) -> (
    Option<(u128, u128)>,
    Option<(u128, u128)>,
    Option<(u128, u128)>,
) {
    // Is there a first interval?
    let left_interval = if start < section.source_range_start {
        let left = start;
        let right = min(start + length - 1, section.source_range_start - 1);
        Some((left, right - left + 1))
    } else {
        None
    };

    // Is there a second interval?
    let intersecting_interval = if start + length - 1 >= section.source_range_start
        && start <= section.source_range_start + section.source_range_length - 1
    {
        // There is some overlap
        let left = max(start, section.source_range_start);
        let right = min(
            start + length - 1,
            section.source_range_start + section.source_range_length - 1,
        );
        Some((left, right - left + 1))
    } else {
        None
    };

    // Remap the intersecting interval, if there is one
    let intersecting_interval = if let Some((left, length)) = intersecting_interval {
        let left = (section.destination + left) - section.source_range_start;
        Some((left, length))
    } else {
        None
    };

    // Is there a third interval?
    let new_remaining_interval =
        if start + length - 1 > section.source_range_start + section.source_range_length - 1 {
            let left = max(
                start,
                section.source_range_start + section.source_range_length,
            );
            let right = start + length - 1;
            Some((left, right - left + 1))
        } else {
            None
        };

    (left_interval, intersecting_interval, new_remaining_interval)
}

fn day04() {
    let input = read_file("day04.txt");
    // 33950, 14814534

    // Parsing a line of the input file. Example:
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1

    let mut total_value = 0;
    let mut matches_on_card: [u32; 250] = [0; 250];
    // Too long but eh.
    let mut card_count: [u32; 250] = [0; 250];

    // Split along ":" and "|"
    for line in input.lines() {
        let parts: Vec<&str> = line.split([':', '|'].as_ref()).collect();

        println!("Parts: {:?}", parts);
        // Cut off "Card " from the first part
        let card_number = parts[0][5..].trim().parse::<i32>().unwrap();
        println!("Card number: {}", card_number);

        // Split the second part by " " and parse them as numbers
        let winning: Vec<i32> = parts[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        println!("Numbers: {:?}", winning);

        // Split the third part by " " and parse them as numbers
        let owned: Vec<i32> = parts[2]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        println!("Owned: {:?}", owned);

        // count how many of the owned numbers are in the winning numbers
        let mut count = 0;
        for number in owned {
            if winning.contains(&number) {
                count += 1;
            }
        }
        println!("Count: {}", count);
        // The value of a card is count 0 => 0, count 1 => 1, count 2 => 2, count 3 => 4, count 4 => 8, ...

        // Calculate the value of the card
        let value = if count > 0 { 2_i32.pow(count - 1) } else { 0 };
        println!("Value: {}", value);

        // Add the value to the total value
        total_value += value;
        matches_on_card[card_number as usize] = count;
        card_count[card_number as usize] = 1;
    }

    println!("Total value: {}", total_value);

    let mut recursive_card_production: [u128; 250] = [0; 250];

    // Calculate part two solution.
    // Reverse iterate over the cards
    let mut index = 249;

    while index > 0 {
        // If the card is not owned, skip it
        if card_count[index] == 0 {
            index -= 1;
            continue;
        }

        // Count the recursive card production of cards below this card.
        recursive_card_production[index] = 1; // The card itself
        for i in (index + 1)..=(index + matches_on_card[index] as usize) {
            recursive_card_production[index] += recursive_card_production[i];
        }

        index -= 1;
    }

    println!("Recursive card production: {:?}", recursive_card_production);
    println!("Sum: {}", recursive_card_production.iter().sum::<u128>());
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
