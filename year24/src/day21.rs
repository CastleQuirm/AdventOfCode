// Potential improvements:
//

use std::collections::HashMap;

use lazy_static::lazy_static;

pub fn day21(input_lines: &[Vec<String>]) -> (String, String) {
    // Set up the number of button presses the human needs to make to get the first
    // robot to move its arm from the first character in the pair to the second and
    // then press it.
    let mut fewest_presses: HashMap<String, usize> = HashMap::from([
        ("AA".to_string(), 1),
        ("^^".to_string(), 1),
        ("vv".to_string(), 1),
        ("<<".to_string(), 1),
        (">>".to_string(), 1),
        ("A^".to_string(), 2),
        ("Av".to_string(), 3),
        ("A<".to_string(), 4),
        ("A>".to_string(), 2),
        ("^A".to_string(), 2),
        ("^<".to_string(), 3),
        ("^>".to_string(), 3),
        ("^v".to_string(), 2),
        ("<A".to_string(), 4),
        ("<^".to_string(), 3),
        ("<>".to_string(), 3),
        ("<v".to_string(), 2),
        (">A".to_string(), 2),
        (">^".to_string(), 3),
        ("><".to_string(), 3),
        (">v".to_string(), 2),
        ("vA".to_string(), 3),
        ("v^".to_string(), 2),
        ("v>".to_string(), 2),
        ("v<".to_string(), 2),
    ]);

    // Determine the following robot's resulting human presses from the previous one.
    // (So second robot using a directional pad).
    fewest_presses = determine_next_robots_human_presses(&fewest_presses);

    // For Part 1, the third robot then uses the numpad. We go straight into solving.
    let answer1 = solve(&input_lines[0], &fewest_presses);

    // In part 2 we have 25 robots using the direction pads before the 26th robot on the
    // numpad, so iteratively update the fewest_presses hashmap before finally going to
    // solve with the 26th robot using the numpad.
    for _ in 2..25 {
        fewest_presses = determine_next_robots_human_presses(&fewest_presses)
    }
    let answer2 = solve(&input_lines[0], &fewest_presses);

    (format!("{}", answer1), format!("{}", answer2))
}

fn determine_next_robots_human_presses(
    prev_robot: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    prev_robot
        .keys()
        .map(|move_and_press| {
            (
                move_and_press.clone(),
                DIRECTIONAL_BUTTON_SEQUENCES
                    .get(move_and_press)
                    .expect("Pair unknown?")
                    .iter()
                    .map(|sequence| {
                        (0..sequence.len())
                            .map(|i| {
                                prev_robot
                                    .get(&get_button_pair(sequence, i))
                                    .expect("oh no")
                            })
                            .sum::<usize>()
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect::<HashMap<String, usize>>()
}

fn solve(codes: &[String], optimal_counts: &HashMap<String, usize>) -> usize {
    codes
        .iter()
        .map(|line| {
            (0..line.len())
                .map(|i| {
                    NUMPAD_BUTTON_SEQUENCES
                        .get(&get_button_pair(line, i))
                        .unwrap()
                        .iter()
                        .map(|option| {
                            (0..option.len())
                                .map(|j| {
                                    optimal_counts
                                        .get(&get_button_pair(option, j))
                                        .expect("woooah")
                                })
                                .sum::<usize>()
                        })
                        .min()
                        .unwrap()
                })
                .sum::<usize>()
                * line[0..3].parse::<usize>().expect("bad parse")
        })
        .sum::<usize>()
}

fn get_button_pair(sequence: &str, target_index: usize) -> String {
    if target_index == 0 {
        "A".to_string() + &sequence[0..1]
    } else {
        sequence[target_index - 1..=target_index].to_string()
    }
}

lazy_static! {
    // Wrote this out originally to have all possible button pairs. When it had to be updated from just giving a sequence to
    // needing to list every possible sequence, I commented out any that aren't used by at least one of the test lines and/or
    // my input. Obviously this means it might not work on others' input but the stuff to add is pretty minimal and easy, just
    // tedious.
    pub static ref NUMPAD_BUTTON_SEQUENCES: HashMap<String, Vec<String>> =
        HashMap::from([
            // ("AA".to_string(), vec!["A".to_string()]),
            ("A0".to_string(), vec!["<A".to_string()]),
            ("A1".to_string(), vec!["^<<A".to_string(), "<^<A".to_string()]),
            // ("A2".to_string(), vec!["<^A".to_string(), "^<A".to_string()]),
            ("A3".to_string(), vec!["^A".to_string()]),
            ("A4".to_string(), vec!["^^<<A".to_string(), "^<^<A".to_string(), "<^^<A".to_string(), "^<<^A".to_string(), "<^<^A".to_string()]),
            ("A5".to_string(), vec!["<^^A".to_string(), "^^<A".to_string(), "^<^A".to_string()]),
            // ("A6".to_string(), vec!["^^A".to_string()]),
            // ("A7".to_string(), vec!["^^^<<A".to_string(), "^^<^<A".to_string(), "^<^^<A".to_string(), "<^^^<A".to_string(), "^^<<^A".to_string(), "^<^<^A".to_string(), "<^^<^A".to_string(), "^<<^^A".to_string(), "<^<^^A".to_string()]),
            ("A8".to_string(), vec!["<^^^A".to_string(), "^<^^A".to_string(), "^^<^A".to_string(), "^^^<A".to_string()]),
            ("A9".to_string(), vec!["^^^A".to_string()]),

            ("0A".to_string(), vec![">A".to_string()]),
            // ("00".to_string(), "A".to_string()),
            // ("01".to_string(), "^<A".to_string()),
            ("02".to_string(), vec!["^A".to_string()]),
            ("03".to_string(), vec![">^A".to_string(), "^>A".to_string()]),
            // ("04".to_string(), "^^<A".to_string()),
            // ("05".to_string(), "^^A".to_string()),
            // ("06".to_string(), ">^^A".to_string()),
            // ("07".to_string(), "^^^<A".to_string()),
            // ("08".to_string(), "^^^A".to_string()),
            // ("09".to_string(), ">^^^A".to_string()),

            ("1A".to_string(), vec![">>vA".to_string(), ">v>A".to_string()]),
            // ("10".to_string(), ">vA".to_string()),
            // ("11".to_string(), "A".to_string()),
            // ("12".to_string(), ">A".to_string()),
            // ("13".to_string(), ">>A".to_string()),
            // ("14".to_string(), "^A".to_string()),
            // ("15".to_string(), ">^A".to_string()),
            // ("16".to_string(), ">>^A".to_string()),
            ("17".to_string(), vec!["^^A".to_string()]),
            // ("18".to_string(), ">^^A".to_string()),
            ("19".to_string(), vec![">>^^A".to_string(), ">^>^A".to_string(), "^>>^A".to_string(), ">^^>A".to_string(), "^>^>A".to_string(), "^^>>A".to_string()]),

            // ("2A".to_string(), ">vA".to_string()),
            // ("20".to_string(), "vA".to_string()),
            // ("21".to_string(), "<A".to_string()),
            // ("22".to_string(), "A".to_string()),
            // ("23".to_string(), ">A".to_string()),
            // ("24".to_string(), "<^A".to_string()),
            // ("25".to_string(), "^A".to_string()),
            // ("26".to_string(), ">^A".to_string()),
            // ("27".to_string(), "<^^A".to_string()),
            ("28".to_string(), vec!["^^A".to_string()]),
            ("29".to_string(), vec![">^^A".to_string(), "^>^A".to_string(), "^^>A".to_string()]),

            ("3A".to_string(), vec!["vA".to_string()]),
            // ("30".to_string(), "<vA".to_string()),
            ("31".to_string(), vec!["<<A".to_string()]),
            // ("32".to_string(), "<A".to_string()),
            // ("33".to_string(), "A".to_string()),
            ("34".to_string(), vec!["<<^A".to_string(), "<^<A".to_string(), "^<<A".to_string()]),
            // ("35".to_string(), "<^A".to_string()),
            // ("36".to_string(), "^A".to_string()),
            ("37".to_string(), vec!["<<^^A".to_string(), "<^<^A".to_string(), "^<<^A".to_string(), "<^^<A".to_string(), "^<^<A".to_string(), "^^<<A".to_string()]),
            // ("38".to_string(), "<^^A".to_string()),
            // ("39".to_string(), "^^A".to_string()),

            // ("4A".to_string(), ">>vvA".to_string()),
            // ("40".to_string(), ">vvA".to_string()),
            ("41".to_string(), vec!["vA".to_string()]),
            // ("42".to_string(), ">vA".to_string()),
            // ("43".to_string(), ">>vA".to_string()),
            // ("44".to_string(), "A".to_string()),
            ("45".to_string(), vec![">A".to_string()]),
            // ("46".to_string(), ">>A".to_string()),
            // ("47".to_string(), "^A".to_string()),
            // ("48".to_string(), "^>A".to_string()),
            // ("49".to_string(), "^>>A".to_string()),

            // ("5A".to_string(), ">vvA".to_string()),
            // ("50".to_string(), "vvA".to_string()),
            // ("51".to_string(), "<vA".to_string()),
            ("52".to_string(), vec!["vA".to_string()]),
            // ("53".to_string(), ">vA".to_string()),
            // ("54".to_string(), "<A".to_string()),
            // ("55".to_string(), "A".to_string()),
            ("56".to_string(), vec![">A".to_string()]),
            // ("57".to_string(), "<^A".to_string()),
            ("58".to_string(), vec!["^A".to_string()]),
            // ("59".to_string(), ">^A".to_string()),

            ("6A".to_string(), vec!["vvA".to_string()]),
            // ("60".to_string(), "<vvA".to_string()),
            // ("61".to_string(), "<<vA".to_string()),
            // ("62".to_string(), "<vA".to_string()),
            // ("63".to_string(), "vA".to_string()),
            // ("64".to_string(), "<<A".to_string()),
            // ("65".to_string(), "<A".to_string()),
            // ("66".to_string(), "A".to_string()),
            // ("67".to_string(), "<<^A".to_string()),
            // ("68".to_string(), "<^A".to_string()),
            // ("69".to_string(), "^A".to_string()),

            // ("7A".to_string(), ">>vvvA".to_string()),
            // ("70".to_string(), ">vvvA".to_string()),
            // ("71".to_string(), "vvA".to_string()),
            // ("72".to_string(), ">vvA".to_string()),
            // ("73".to_string(), ">>vvA".to_string()),
            // ("74".to_string(), "vA".to_string()),
            // ("75".to_string(), ">vA".to_string()),
            // ("76".to_string(), ">>vA".to_string()),
            // ("77".to_string(), "A".to_string()),
            // ("78".to_string(), ">A".to_string()),
            ("79".to_string(), vec![">>A".to_string()]),

            ("8A".to_string(), vec![">vvvA".to_string(), "v>vvA".to_string(), "vv>vA".to_string(), "vvv>A".to_string()]),
            ("80".to_string(), vec!["vvvA".to_string()]),
            // ("81".to_string(), "<vvA".to_string()),
            // ("82".to_string(), "vvA".to_string()),
            // ("83".to_string(), ">vvA".to_string()),
            // ("84".to_string(), "<vA".to_string()),
            // ("85".to_string(), "vA".to_string()),
            ("86".to_string(), vec![">vA".to_string(), "v>A".to_string()]),
            // ("87".to_string(), "<A".to_string()),
            // ("88".to_string(), "A".to_string()),
            // ("89".to_string(), ">A".to_string()),

            ("9A".to_string(), vec!["vvvA".to_string()]),
            // ("90".to_string(), "<vvvA".to_string()),
            // ("91".to_string(), "<<vvA".to_string()),
            // ("92".to_string(), "<vvA".to_string()),
            // ("93".to_string(), "vvA".to_string()),
            // ("94".to_string(), "<<vA".to_string()),
            // ("95".to_string(), "<vA".to_string()),
            // ("96".to_string(), "vA".to_string()),
            // ("97".to_string(), "<<A".to_string()),
            ("98".to_string(), vec!["<A".to_string()]),
            // ("99".to_string(), "A".to_string()),

        ]);

        pub static ref DIRECTIONAL_BUTTON_SEQUENCES: HashMap<String, Vec<String>> = HashMap::from([
            ("AA".to_string(), vec!["A".to_string()]),
            ("^^".to_string(), vec!["A".to_string()]),
            ("vv".to_string(), vec!["A".to_string()]),
            ("<<".to_string(), vec!["A".to_string()]),
            (">>".to_string(), vec!["A".to_string()]),
            ("A^".to_string(), vec!["<A".to_string()]),
            ("Av".to_string(), vec!["<vA".to_string(), "v<A".to_string()]),
            ("A<".to_string(), vec!["v<<A".to_string(), "<v<A".to_string()]),
            ("A>".to_string(), vec!["vA".to_string()]),
            ("^A".to_string(), vec![">A".to_string()]),
            ("^<".to_string(), vec!["v<A".to_string()]),
            ("^>".to_string(), vec![">vA".to_string(), "v>A".to_string()]),
            ("^v".to_string(), vec!["vA".to_string()]),
            ("<A".to_string(), vec![">>^A".to_string(), ">^>A".to_string()]),
            ("<^".to_string(), vec![">^A".to_string()]),
            ("<>".to_string(), vec![">>A".to_string()]),
            ("<v".to_string(), vec![">A".to_string()]),
            (">A".to_string(), vec!["^A".to_string()]),
            (">^".to_string(), vec!["<^A".to_string(), "^<A".to_string()]),
            ("><".to_string(), vec!["<<A".to_string(), "<<A".to_string()]),
            (">v".to_string(), vec!["<A".to_string()]),
            ("vA".to_string(), vec![">^A".to_string(), "^>A".to_string()]),
            ("v^".to_string(), vec!["^A".to_string()]),
            ("v>".to_string(), vec![">A".to_string()]),
            ("v<".to_string(), vec!["<A".to_string()]),

        ]);
}

#[cfg(test)]
mod tests {
    use super::day21;
    use crate::utils::load_input;

    #[test]
    fn check_day21_case01() {
        full_test(
            "379A",           // INPUT STRING
            "24256",          // PART 1 RESULT
            "29556553253044", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day21_case02() {
        full_test(
            "029A
980A
179A
456A
379A", // INPUT STRING
            "126384",          // PART 1 RESULT
            "154115708116294", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day21(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
