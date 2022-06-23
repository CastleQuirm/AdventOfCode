// Potential improvements:
//

use regex::Regex;

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    // Prepare the player count
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let (player_count, last_marble) = re
        .captures(&input_lines[0][0])
        .map(|cap| {
            let players = cap[1].parse::<usize>().expect("Didn't parse player count");
            let points = cap[2].parse::<usize>().expect("Didn't parse max points");
            (players, points)
        })
        .expect("Regex didn't match");

    let answer1 = winning_score(player_count, last_marble);
    let answer2 = winning_score(player_count, last_marble * 100);
    (format!("{}", answer1), format!("{}", answer2))
}

fn winning_score(player_count: usize, last_marble: usize) -> usize {
    let mut player_scores = vec![0; player_count];

    // // The naive approach: let's just have a mutable vec and a pointer.
    // // THIS DOESN'T WORK FOR PART 2: insert and remove become horribly slow with extremely large vectors.
    // let mut circle = vec![0];
    // let mut current_marble_ix = 0;

    // for marble_num in 1..=last_marble {
    //     if marble_num % 23 != 0 {
    //         // Insert the marble
    //         let insert_ix = (current_marble_ix + 2) % circle.len();
    //         circle.insert(insert_ix, marble_num);
    //         current_marble_ix = insert_ix;
    //     } else {
    //         // Determine the active player
    //         let active_player = marble_num as usize % player_count;

    //         // Move back 7.
    //         current_marble_ix = if current_marble_ix < 7 {
    //             circle.len() + current_marble_ix - 7
    //         } else {
    //             current_marble_ix - 7
    //         };

    //         // Score both marbles.
    //         let scored_marble = circle.remove(current_marble_ix);
    //         println!("Player {} scores marbles {} and {} taking their score from {} to {}", active_player, scored_marble, marble_num, player_scores[active_player], player_scores[active_player] + scored_marble + marble_num);
    //         player_scores[active_player] += scored_marble + marble_num;

    //         if current_marble_ix == circle.len() {
    //             current_marble_ix = 0
    //         };
    //     }
    // }

    // The "I did this in a different year" approach: we'll just have an array of "next" values.
    let mut circle = vec![0; last_marble + 1];
    let mut current_marble = 0;
    for marble_number in 1..=last_marble {
        if marble_number % 23 != 0 {
            current_marble = circle[current_marble];
            circle[marble_number] = circle[current_marble];
            circle[current_marble] = marble_number;
            current_marble = marble_number;
        } else {
            // Remove the marble after the one 5 values ago (number is non-obvious, but works out consistently)
            let removed_marble = circle[marble_number - 5];
            circle[marble_number - 5] = circle[removed_marble];

            // Add the score
            let active_player = marble_number % player_count;
            player_scores[active_player] += removed_marble + marble_number;

            // Move the pointer
            current_marble = circle[marble_number - 5];
        }
    }

    *player_scores.iter().max().expect("No highest score?!")
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case01() {
        part1_test(
            "9 players; last marble is worth 25 points", // INPUT STRING
            "32",                                        // PART 1 RESULT
        );
        part1_test(
            "10 players; last marble is worth 1618 points", // INPUT STRING
            "8317",                                         // PART 1 RESULT
        );
        part1_test(
            "13 players; last marble is worth 7999 points", // INPUT STRING
            "146373",                                       // PART 1 RESULT
        );
    }

    fn part1_test(input_text: &str, part1_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day09(&input_lines).0, part1_result.to_string());
    }
}
