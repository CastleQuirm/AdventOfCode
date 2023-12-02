// Potential improvements:
//

use std::collections::HashMap;

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let game_info = input_lines[0].iter().map(Game::from);

    let answer1 = game_info
        .clone()
        .filter_map(|game| if game.possible() { Some(game.id) } else { None })
        .sum::<u32>();
    let answer2 = game_info.map(|game| game.power_min_set()).sum::<u32>();
    (format!("{}", answer1), format!("{}", answer2))
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl From<&String> for Game {
    fn from(line: &String) -> Self {
        let mut split_on_colon = line.split(": ");
        let game_info = split_on_colon.next().unwrap();
        let id = game_info
            .strip_prefix("Game ")
            .expect("Didn't start with 'Game '")
            .parse::<u32>()
            .expect("Didn't parse id as number");
        let hands_text = split_on_colon
            .next()
            .expect("Nothing after the game heading");
        let hands = hands_text.split("; ");
        Self {
            id,
            hands: hands.map(Hand::from).collect::<Vec<_>>(),
        }
    }
}

impl Game {
    fn possible(&self) -> bool {
        self.hands.iter().all(|hand| hand.possible())
    }

    fn power_min_set(&self) -> u32 {
        let min_red = self
            .hands
            .iter()
            .map(|hand| hand.num_red)
            .max()
            .expect("Look Ma, no hands?");
        let min_green = self
            .hands
            .iter()
            .map(|hand| hand.num_green)
            .max()
            .expect("Look Ma, no hands?");
        let min_blue = self
            .hands
            .iter()
            .map(|hand| hand.num_blue)
            .max()
            .expect("Look Ma, no hands?");
        min_red * min_green * min_blue
    }
}

struct Hand {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        let colour_info = hand
            .split(", ")
            .map(|one_colour| {
                let mut info = one_colour.split_whitespace();
                let count = info
                    .next()
                    .expect("String was wholly whitespace")
                    .parse::<u32>()
                    .expect("Couldn't parse the count as a number");
                let colour = info.next().expect("No second text for colour");
                assert_eq!(info.next(), None);
                (colour, count)
            })
            .collect::<HashMap<_, _>>();
        Self {
            num_red: *colour_info.get("red").unwrap_or(&0),
            num_green: *colour_info.get("green").unwrap_or(&0),
            num_blue: *colour_info.get("blue").unwrap_or(&0),
        }
    }
}

impl Hand {
    fn possible(&self) -> bool {
        self.num_red <= 12 && self.num_green <= 13 && self.num_blue <= 14
    }
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", // INPUT STRING
            "8",    // PART 1 RESULT
            "2286", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
