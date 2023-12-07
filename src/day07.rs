pub fn solve_day07(input: &str, with_joker: bool) -> u128 {
    let mut all_hands = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let hand = parts[0];
        let bid = parts[1].parse::<u128>().unwrap();
        all_hands.push(Hand::new(hand.to_string(), bid, with_joker));
    }
    all_hands.sort_by_key(|h| h.integer_proxy);
    // println!("All hands: {:?}", all_hands);
    for hand in &all_hands {
        println!("Hand: {:?}", hand);
    }

    let mut total_winning = 0;
    for (i, hand) in all_hands.iter().enumerate() {
        total_winning += hand.bid * (i + 1) as u128;
    }

    println!("Total winning: {}", total_winning);
    total_winning
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u128,
    hand_type: HandType,
    integer_proxy: u64,
}

impl Hand {
    fn new(cards: String, bid: u128, with_joker: bool) -> Self {
        let hand_type = classify_hand_type(cards.clone(), with_joker);
        let integer_proxy = Self::hand_to_integer_proxy(cards.clone(), hand_type, with_joker);
        Self {
            cards,
            bid,
            hand_type,
            integer_proxy,
        }
    }

    fn hand_to_integer_proxy(cards: String, hand_type: HandType, with_joker: bool) -> u64 {
        // Builds an integer that, looking at the bits is:
        // 3 bits for hand type, 4 bits for each cards from left to right in cards.
        let chars = cards.chars().collect::<Vec<char>>();
        hand_type as u64 * 2_u64.pow(20)
            + card_to_ordinal(chars[0], with_joker) as u64 * 2_u64.pow(16)
            + card_to_ordinal(chars[1], with_joker) as u64 * 2_u64.pow(12)
            + card_to_ordinal(chars[2], with_joker) as u64 * 2_u64.pow(8)
            + card_to_ordinal(chars[3], with_joker) as u64 * 2_u64.pow(4)
            + card_to_ordinal(chars[4], with_joker) as u64
    }
}

#[derive(Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn card_to_ordinal(card: char, with_joker: bool) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => {
            if with_joker {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Card {} not recognized", card),
    }
}

/// Every hand is a string of 5 cards, like "32T3K".
/// There are no colors in "Camel Cards".
fn classify_hand_type(cards: String, with_joker: bool) -> HandType {
    let mut count_by_ordinal: [u8; 15] = [0; 15];
    for card in cards.chars() {
        count_by_ordinal[card_to_ordinal(card, with_joker) as usize] += 1;
    }

    let joker_count = count_by_ordinal[1];

    // Find the type (joker excluded) with the highest count
    // Then add the joker count to it.
    let mut max_count = 0;
    let mut max_count_i = 0;
    for i in 2..=14 {
        if count_by_ordinal[i] > max_count {
            max_count = count_by_ordinal[i];
            max_count_i = i;
        }
    }

    if max_count_i == 0 {
        // All jokers.
        return HandType::FiveOfAKind;
    }
    count_by_ordinal[max_count_i] += joker_count;
    count_by_ordinal[1] = 0;

    let max_count = *count_by_ordinal.iter().max().unwrap();

    if max_count == 5 {
        return HandType::FiveOfAKind;
    }
    if max_count == 4 {
        return HandType::FourOfAKind;
    }
    if max_count == 3 {
        if count_by_ordinal.iter().filter(|&&count| count == 2).count() == 1 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if max_count == 2 {
        if count_by_ordinal.iter().filter(|&&count| count == 2).count() == 2 {
            return HandType::TwoPair;
        }
        return HandType::OnePair;
    }
    HandType::HighCard
}

// test module
#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_day07_part1() {
        let input = utils::read_file("day07.txt");
        let result = solve_day07(&input, false);
        assert_eq!(result, 249638405);
    }

    #[test]
    fn test_day07_part2() {
        let input = utils::read_file("day07.txt");
        let result = solve_day07(&input, true);
        assert_eq!(result, 249776650);
    }
}
