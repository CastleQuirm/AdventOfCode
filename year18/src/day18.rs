// Potential improvements:
//

use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

pub fn day18(input_lines: &[Vec<String>]) -> (String, String) {
    // read input into vec of vecs.
    let mut map = read_map(&input_lines[0]);
    // println!("{}", map);

    // Game of life it, recording our results in a Hash.
    let mut seen_state_scores: HashMap<u32, HashMap<Map, i32>> = HashMap::new();
    let mut answer1 = 0;
    let mut answer2 = 0;
    for minute in 1..=1000000000 {
        let new_score = map.next().unwrap();
        if minute == 10 {
            answer1 = new_score;
            // Hack to stop the UT from running indefinitely
            if map.map.len() < 20 {
                break;
            }
        }

        let saved_map_1 = map.clone();
        let saved_map_2 = map.clone();
        let entry = seen_state_scores
            .entry(new_score)
            .or_insert_with(|| HashMap::from([(saved_map_1, minute)]));
        let entry = entry.entry(saved_map_2).or_insert(minute);

        if *entry < minute {
            // found a loop.
            let needed_hops = (1000000000 - minute) % (minute - *entry) - 1;
            answer2 = map.nth(needed_hops as usize).unwrap();
            break;
        }
    }

    (format!("{}", answer1), format!("{}", answer2))
}

fn read_map(input_lines: &[String]) -> Map {
    let map = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => AcreState::Open,
                    '|' => AcreState::Trees,
                    '#' => AcreState::Lumber,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Map { map }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<AcreState>>,
}

impl Map {
    fn adjacent(&self, x: usize, y: usize, acre: AcreState) -> usize {
        let dy = if y > 0 { -1 } else { 0 }..=i32::from(y < self.map.len() - 1);

        dy.map(|dy| {
            let dx = if x > 0 { -1 } else { 0 }..=i32::from(x < self.map[0].len() - 1);
            dx.filter(|&dx| {
                let x: usize = (x as i32 + dx).try_into().expect("Negative result?");
                let y: usize = (y as i32 + dy).try_into().expect("Negative result?");
                self.map[y][x] == acre && (dx != 0 || dy != 0)
            })
            .count()
        })
        .sum()
    }
}

impl Iterator for Map {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("Minute!");
        let mut tree_count = 0;
        let mut lumber_count = 0;

        let new_map = self
            .map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, acre)| {
                        // println!("evolve coord {},{} currently {}", x, y, acre);

                        let new_acre = match acre {
                            AcreState::Open | AcreState::Trees => {
                                // These states both move to their next if they have three or more adjacent in that state, else stay.
                                if self.adjacent(x, y, acre.next()) >= 3 {
                                    acre.next()
                                } else {
                                    *acre
                                }
                            }
                            AcreState::Lumber => {
                                if self.adjacent(x, y, AcreState::Lumber) >= 1
                                    && self.adjacent(x, y, AcreState::Trees) >= 1
                                {
                                    *acre
                                } else {
                                    acre.next()
                                }
                            }
                        };
                        if new_acre == AcreState::Lumber {
                            lumber_count += 1;
                        }
                        if new_acre == AcreState::Trees {
                            tree_count += 1;
                        }
                        new_acre
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.map = new_map;
        // println!("Score: {}", tree_count * lumber_count);
        Some(tree_count * lumber_count)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            for c in line {
                let _ = f.write_fmt(format_args!("{}", c));
            }
            let _ = f.write_char('\n');
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AcreState {
    Open,
    Trees,
    Lumber,
}

impl AcreState {
    fn next(&self) -> Self {
        match self {
            Self::Open => Self::Trees,
            Self::Trees => Self::Lumber,
            Self::Lumber => Self::Open,
        }
    }
}

impl Display for AcreState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcreState::Open => f.write_char('.'),
            AcreState::Trees => f.write_char('|'),
            AcreState::Lumber => f.write_char('#'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day18;
    use crate::utils::load_input;

    #[test]
    fn check_day18_case01() {
        full_test(
            ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.", // INPUT STRING
            "1147", // PART 1 RESULT
            "0",    // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day18(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
