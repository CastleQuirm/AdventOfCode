// Potential improvements:
//

use std::collections::HashSet;

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let game_count = input_lines[0].len();
    let game_info = input_lines[0].iter().map(Card::from);

    let mut card_count = vec![1; game_count];
    let answer1 = game_info
        .enumerate()
        .map(|(line_number, game)| {
            // println!("{line_number}");
            // println!("{}", game.number_wins);
            assert_eq!((line_number + 1) as u32, game.id);
            for i in 1..=game.number_wins as usize {
                card_count[line_number + i] += card_count[line_number];
            }
            if game.number_wins > 0 {
                2_u32.pow(game.number_wins - 1)
            } else {
                0
            }
        })
        .sum::<u32>();

    let answer2 = card_count.iter().sum::<u32>();
    (format!("{}", answer1), format!("{}", answer2))
}

struct Card {
    id: u32,
    number_wins: u32,
}

impl From<&String> for Card {
    fn from(line: &String) -> Self {
        let mut split_by_part = line.split(": ");
        let card_info = split_by_part.next().unwrap();
        let id = card_info
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .expect("Didn't parse id as number");
        let mut numbers = split_by_part
            .next()
            .expect("No winning numbers")
            .split(" | ");
        assert!(split_by_part.next().is_none());
        let winning_numbers = numbers
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().expect("Couldn't parse number"))
            .collect::<HashSet<u32>>();
        let elf_numbers = numbers
            .next()
            .expect("No second set of numbers!")
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().expect("Couldn't parse number"))
            .collect::<HashSet<u32>>();
        let hits = elf_numbers.intersection(&winning_numbers).count() as u32;
        Self {
            id,
            number_wins: hits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", // INPUT STRING
            "13", // PART 1 RESULT
            "30", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
