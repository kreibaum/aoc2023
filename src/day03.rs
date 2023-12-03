use std::collections::HashMap;

pub fn both_parts(input: &str) -> (u64, u64) {
    let symbol_map = scan_symbols(input);

    // Each entry is (product, count) with a default of (1, 0) for empty spaces
    let mut gear_ratio_map: HashMap<(i32, i32), (u64, u64)> = HashMap::new();

    let mut accumulator = 0;

    for (y, line) in input.lines().enumerate() {
        let mut number = ScannedNumber::new();
        for (x, c) in line.chars().enumerate() {
            // Do we see a digit?
            if let Some(digit) = c.to_digit(10) {
                number.extend(digit, x, y, &symbol_map);
            } else {
                if number.is_part {
                    // Track number in accumulator
                    accumulator += number.number;
                    // Track number in Gear Ratio Map
                    number.update_gear_ratio_map(&mut gear_ratio_map);
                }
                // Reset number
                number = ScannedNumber::new();
            }
        }
        if number.is_part {
            // Track number in accumulator
            accumulator += number.number;
            // Track number in Gear Ratio Map
            number.update_gear_ratio_map(&mut gear_ratio_map);
        }
    }

    // Part 1: Accumulator
    let part1 = accumulator;

    // Part 2: Gear Ratio Map
    let part2: u64 = gear_ratio_map
        .iter()
        .filter(|(_, (_, count))| *count == 2)
        .filter(|(coord, _)| symbol_map.get(coord) == Some(&'*'))
        .map(|(_, (product, _))| product)
        .sum();

    (part1, part2)
}

struct ScannedNumber {
    number: u64,
    is_part: bool,
    left_x: i32,
    right_x: i32,
    y: i32,
}

impl ScannedNumber {
    fn new() -> Self {
        Self {
            number: 0,
            is_part: false,
            left_x: 0,
            right_x: 0,
            y: 0,
        }
    }

    fn is_some(&self) -> bool {
        self.number > 0
    }

    fn extend(&mut self, digit: u32, x: usize, y: usize, symbol_map: &HashMap<(i32, i32), char>) {
        // Is this the first digit?
        if self.number == 0 {
            self.left_x = x as i32;
        }
        self.number = self.number * 10 + digit as u64;
        self.right_x = x as i32;
        self.y = y as i32;
        self.is_part = self.is_part || symbol_in_nbhd(symbol_map, x as i32, y as i32);
    }

    fn update_gear_ratio_map(&self, gear_ratio_map: &mut HashMap<(i32, i32), (u64, u64)>) {
        if !self.is_some() {
            return;
        }
        for x in (self.left_x - 1)..=(self.right_x + 1) {
            for y in (self.y - 1)..=(self.y + 1) {
                let entry = gear_ratio_map.entry((x, y)).or_insert((1, 0));
                entry.0 *= self.number;
                entry.1 += 1;
            }
        }
    }
}

fn scan_symbols(input: &str) -> HashMap<(i32, i32), char> {
    let mut symbol_map: HashMap<(i32, i32), char> = HashMap::new();

    // Scan input to put all symbols into a map
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !"0123456789.".contains(c) {
                symbol_map.insert((x as i32, y as i32), c);
            }
        }
    }

    symbol_map
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
