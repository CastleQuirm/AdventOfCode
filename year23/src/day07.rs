// Potential improvements:
//

use std::collections::HashMap;

use itertools::Itertools;

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0]
        .iter()
        .map(|line| Hand::from(line, false))
        .sorted()
        .enumerate()
        .map(|(pos, hand)| (pos + 1) * hand.bid)
        .sum::<usize>();

    let answer2 = input_lines[0]
        .iter()
        .map(|line| Hand::from(line, true))
        .sorted()
        .enumerate()
        .map(|(pos, hand)| (pos + 1) * hand.bid)
        .sum::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

/// Hand, with fields ordered to ensure the derived Ord and PartialOrd compare the rank first.
/// The bid shouldn't technically distinguish hands but we don't expect to ever have an equal hand before that.
/// (For even cleaner code, that wouldn't even be part of the)
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    rank: HandRank,
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
        let rank = HandRank::rank(&cards);
        Self {
            cards,
            bid: bid.parse::<usize>().expect("Couldn't parse bid"),
            rank,
        }
    }
}

/// HandRanks listed as an enum. If we write the values in ascending order,
/// the derived PartialOrd gives us a ranking immediately.
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    fn rank(cards: &[u32]) -> Self {
        // IMPROVEMENT: make these into statics or OneCell'd or similar
        let five_of_a_kind = Vec::from([5]);
        let four_of_a_kind = Vec::from([1, 4]);
        let full_house = Vec::from([2, 3]);
        let three_of_a_kind = Vec::from([1, 1, 3]);
        let two_pair = Vec::from([1, 2, 2]);
        let pair = Vec::from([1, 1, 1, 2]);
        let high_card = Vec::from([1, 1, 1, 1, 1]);

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
        // the values are.
        let mut hand_collection = card_count.values().sorted().cloned().collect::<Vec<i32>>();

        if hand_collection.is_empty() {
            // Special case for five jokers
            hand_collection = five_of_a_kind.clone();
        } else {
            // Add any jokers to our single largest set, which is always the last one.
            let last_index = hand_collection.len() - 1;
            hand_collection[last_index] += joker_count;
        }

        // Get the hand rank
        if hand_collection == five_of_a_kind {
            HandRank::FiveOfAKind
        } else if hand_collection == four_of_a_kind {
            HandRank::FourOfAKind
        } else if hand_collection == full_house {
            HandRank::FullHouse
        } else if hand_collection == three_of_a_kind {
            HandRank::ThreeOfAKind
        } else if hand_collection == two_pair {
            HandRank::TwoPair
        } else if hand_collection == pair {
            HandRank::Pair
        } else if hand_collection == high_card {
            HandRank::HighCard
        } else {
            panic!()
        }
        // match foo {
        //     five_of_a_kind => HandRank::FiveOfAKind,
        //     four_of_a_kind => HandRank::FourOfAKind,
        //     full_house => HandRank::FullHouse,
        //     three_of_a_kind=> HandRank::ThreeOfAKind,
        //     two_pair => HandRank::TwoPair,
        //     pair => HandRank::Pair,
        //     high_card => HandRank::HighCard,
        //     _ => panic!()
        // }
    }
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
