// Potential improvements:
//

use std::collections::HashMap;

use itertools::Itertools;

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = calculate_answers(input_lines, false);
    let answer2 = calculate_answers(input_lines, true);
    (format!("{}", answer1), format!("{}", answer2))
}

fn calculate_answers(input_lines: &[Vec<String>], with_jokers: bool) -> usize {
    input_lines[0]
        .iter()
        .map(|line| Hand::from(line, with_jokers))
        .sorted()
        .enumerate()
        .map(|(pos, hand)| (pos + 1) * hand.bid)
        .sum::<usize>()
}

/// Hand, with fields ordered to ensure the derived Ord and PartialOrd compare the rank first.
/// The bid shouldn't technically distinguish hands but we don't expect to ever have an equal hand before that.
/// (For even cleaner code, that wouldn't even be part of the struct!)
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    rank: Vec<i32>,
    cards: Vec<u32>,
    bid: usize,
}

impl Hand {
    fn from(line: &str, with_jokers: bool) -> Self {
        let (hand, bid) = line.split_once(' ').expect("Bad entry line");
        assert_eq!(hand.len(), 5);
        let cards = hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if with_jokers {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => c.to_digit(10).expect("not a digit"),
            })
            .collect::<Vec<u32>>();
        let rank = rank(&cards);
        Self {
            rank,
            bid: bid.parse::<usize>().expect("Couldn't parse bid"),
            cards,
        }
    }
}

fn rank(cards: &[u32]) -> Vec<i32> {
    let mut card_count = HashMap::new();
    for card in cards {
        card_count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    assert_eq!(card_count.values().sum::<i32>(), 5);

    // Get the number of jokers and remove its entry from the list.
    let joker_count = card_count.remove(&1).unwrap_or(0);

    // Collect the hand rank by ordering how many of different values we have; we no longer care what
    // the values are. Use `.rev()` to put the largest values at the start.
    let mut hand_collection = card_count
        .values()
        .sorted()
        .rev()
        .cloned()
        .collect::<Vec<i32>>();

    if hand_collection.is_empty() {
        // Special case for five jokers
        hand_collection = Vec::from([5]);
    } else {
        // Add any jokers to our single largest set, which is the first one.
        hand_collection[0] += joker_count;
    }

    hand_collection
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483", // INPUT STRING
            "6440", // PART 1 RESULT
            "5905", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
