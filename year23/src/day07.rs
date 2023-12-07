// Potential improvements:
//

use std::collections::HashMap;

use itertools::Itertools;

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    assert!(HandRank::FiveOfAKind > HandRank::FourOfAKind);
    assert!(HandRank::ThreeOfAKind > HandRank::TwoPair);

    let hand1=  Hand::from(&input_lines[0][0]);
    let hand2 = Hand::from(&input_lines[0][1]);
    let hand3 = Hand::from(&input_lines[0][2]);
    let hand4 = Hand::from(&input_lines[0][3]);
    let hand5 = Hand::from(&input_lines[0][4]);
    println!("{:?}", hand1);
    println!("{:?}", hand2);
    println!("{:?}", hand3);
    println!("{:?}", hand4);
    println!("{:?}", hand5);

    assert!(hand1 < hand4);
    assert!(hand4 < hand3);
    assert!(hand3 < hand2);
    assert!(hand2 < hand5);

    let answer1 = input_lines[0].iter().map(Hand::from).sorted().enumerate().map(|(pos, hand)| {
        println!("{:?}", hand);
        (pos + 1) * hand.bid}).sum::<usize>();

    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: usize,
    rank: HandRank
}

impl From<&String> for Hand {
    fn from(line: &String) -> Self {
        let (hand, bid) = line.split_once(" ").expect("Bad entry line");
        assert_eq!(hand.len(), 5);
        let cards = hand.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).expect("not a digit"),
            }
        }).collect::<Vec<u32>>();
        let rank = HandRank::rank(&cards);
        Self {
            cards,
            bid: bid.parse::<usize>().expect("Couldn't parse bid"),
            rank,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank_comp = self.rank.cmp(&other.rank);
        if rank_comp.is_eq() {
            for i in 0..5 {
                if self.cards[i].cmp(&other.cards[i]).is_ne() {
                    return self.cards[i].cmp(&other.cards[i])
                }
            }
            std::cmp::Ordering::Equal
        } else {
            println!("{:?} is {:?} than {:?}", self.rank, rank_comp, other.rank);
            rank_comp
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
        println!("Cards: {:?}", cards);
        let mut card_count = HashMap::new();
        for card in cards {
            println!("current card_count {:?}", card_count);
            println!("current card {card}");
            card_count.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }
        println!("{:?}", card_count.values());
        assert_eq!(card_count.values().sum::<i32>(), 5);
        let five_of_a_kind = Vec::from([&5]);
        let four_of_a_kind = Vec::from([&1, &4]);
        let full_house = Vec::from([&2, &3]);
        let three_of_a_kind = Vec::from([&1, &1, &3]);
        let two_pair = Vec::from([&1, &2, &2]);
        let pair = Vec::from([&1, &1, &1, &2]);
        let high_card = Vec::from([&1, &1, &1, &1, &1]);
        let foo = card_count.values().sorted().collect::<Vec<&i32>>();
        println!("{:?}", foo);

        if foo == five_of_a_kind {
            HandRank::FiveOfAKind
        } else if foo == four_of_a_kind {
            HandRank::FourOfAKind
        } else if foo == full_house {
            HandRank::FullHouse
        } else if foo == three_of_a_kind {
            HandRank::ThreeOfAKind
        } else if foo == two_pair {
            HandRank::TwoPair
        } else if foo == pair {
            HandRank::Pair
        } else if foo == high_card {
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
QQQJA 483",  // INPUT STRING
            "6440", // PART 1 RESULT
            "0", // PART 2 RESULT
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
