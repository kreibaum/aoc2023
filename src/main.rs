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
    // let input = read_file("day05_test.txt");
    let input = read_file("day05.txt");

    let mut lines = input.lines();
    // The first line is for seeds:
    // seeds: 79 14 55 13
    let seeds = lines.next().unwrap().split(' ').skip(1).map(|s| s.parse::<u128>().unwrap()).collect::<Vec<u128>>();
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
    println!("Seeds: {:?}", seeds);
    println!("Seed to soil: {:?}", seed_to_soil);
    println!("Soil to fertilizer: {:?}", soil_to_fertilizer);
    println!("Fertilizer to water: {:?}", fertilizer_to_water);
    println!("Water to light: {:?}", water_to_light);
    println!("Light to temperature: {:?}", light_to_temperature);
    println!("Temperature to humidity: {:?}", temperature_to_humidity);
    println!("Humidity to location: {:?}", humidity_to_location);        

    // For each seed, map though all the maps, producing an untput line like:
    // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.

    // println!("{}", water_to_light.map(81));

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
        let parts = line.split(' ').map(|s| s.parse::<u128>().unwrap()).collect::<Vec<u128>>();
        seed_to_soil.map_sections.push(MapSection {
            destination: parts[0],
            source_range_start: parts[1],
            source_range_length: parts[2],
        });
    }
    seed_to_soil.map_sections.sort_by_key(|s| s.source_range_start);

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
        while index < self.map_sections.len() && self.map_sections[index].source_range_start+ self.map_sections[index].source_range_length -1 < source {
            index += 1;
        }
        if index == self.map_sections.len() {
            return source; // No mapping defined
        }
        // println!("Index: {}, mappiing {:?}", index, self.map_sections[index]);
        if self.map_sections[index].source_range_start + self.map_sections[index].source_range_length > source && self.map_sections[index].source_range_start <= source {
            return (self.map_sections[index].destination + source) - self.map_sections[index].source_range_start;
        }
        return source; // No mapping defined
    }
}

fn day04() {
    let input = read_file("day04.txt");
    // 33950, 14814534

    // Parsing a line of the input file. Example:
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1

    let mut total_value = 0;
    let mut matches_on_card : [u32; 250] = [0; 250];
    // Too long but eh.
    let mut card_count : [u32; 250] = [0; 250];

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

    let mut recursive_card_production : [u128; 250] = [0; 250];

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
