
pub fn part01(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        total += decode_into_number(line, starting_digit) as u32;
    }
    total
}

pub fn part02(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        total += decode_into_number(line, starting_digit_allow_text) as u32;
    }
    total
}

/// Decodes a line into a number using the given decoder
fn decode_into_number(line: &str, decoder: impl Fn(&str) -> Option<u8>) -> u8 {
    let mut first_digit = None;
    let mut last_digit = None;
    for i in 0..line.len() {
        let slice = &line[i..];
        if let Some(digit) = decoder(slice) {
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }
    }
    // Since this is AOC, we are allowed to just unwrap and can assume this to be valid.
    first_digit.unwrap() * 10 + last_digit.unwrap()
}

/// Checks if the slice starts with a digit and returns it.
fn starting_digit(slice: &str) -> Option<u8> {
    let mut chars = slice.chars();
    let digit = chars.next()?;
    digit.to_digit(10).map(|d| d as u8)
}

/// Checks if the slice starts with a digit and returns it.
/// If not, checks if the slice starts with a text representation of a digit and returns it.
fn starting_digit_allow_text(slice: &str) -> Option<u8> {
    if let Some(digit) = starting_digit(slice) {
        Some(digit)
    } else if slice.starts_with("zero") {
        Some(0)
    } else if slice.starts_with("one") {
        Some(1)
    } else if slice.starts_with("two") {
        Some(2)
    } else if slice.starts_with("three") {
        Some(3)
    } else if slice.starts_with("four") {
        Some(4)
    } else if slice.starts_with("five") {
        Some(5)
    } else if slice.starts_with("six") {
        Some(6)
    } else if slice.starts_with("seven") {
        Some(7)
    } else if slice.starts_with("eight") {
        Some(8)
    } else if slice.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}
