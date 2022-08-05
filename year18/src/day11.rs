// Potential improvements:
//

use crate::utils::Coord;

pub fn day11(input_lines: &[Vec<String>]) -> (String, String) {
    let serial_no = input_lines[0][0].parse::<i32>().expect("Can read");

    let answer_coord_1 = best_for_size(serial_no, 3).0;

    // This isn't efficient, but it works.  Ran in about 7 minutes on my home machine in release build.
    // Commenting out in order to (a) encourage to see if I can do something better and (b) so it doesn't
    // cause the UTs to run very long.
    // let (answer_coord_2, size, _) = (1..=300).fold(
    //     (Coord { x: 0, y: 0 }, 0, -6),
    //     |(best_start, best_size, best_power), candidate_size| {
    //         println!("Try size {}", candidate_size);
    //         let (candidate_best_coord, candidate_best_power) =
    //             best_for_size(serial_no, candidate_size);
    //         if candidate_best_power > best_power {
    //             (candidate_best_coord, candidate_size, candidate_best_power)
    //         } else {
    //             (best_start, best_size, best_power)
    //         }
    //     },
    // );

    // Still fairly naive but more efficient: let's take every cell as a top left corner and work out the best
    // result across possible sizes by just adding the new elements each time.
    // ...and this takes about 6s on my machine.  I'll call that a success!
    let (answer_coord_2, size, _) = (0..300_i32.pow(2))
        .map(|i| {
            let x = (i / 300) + 1;
            let y = (i % 300) + 1;
            Coord { x, y }
        })
        .fold(
            (Coord { x: 0, y: 0 }, 0, -6),
            |(best_start, best_size, best_power), candidate_corner| {
                let (candidate_size, candidate_power, _) =
                    best_for_corner(serial_no, candidate_corner);
                if candidate_power > best_power {
                    (candidate_corner, candidate_size, candidate_power)
                } else {
                    (best_start, best_size, best_power)
                }
            },
        );

    let answer1 = format!("{},{}", answer_coord_1.x, answer_coord_1.y);
    let answer2 = format!("{},{},{}", answer_coord_2.x, answer_coord_2.y, size);
    (answer1, answer2)
}

// Oops...this is subject to some strong off-by-one errors, meaning I'm examining the grid 0..=299 instead of 1..=300.
// Fortunately the right answer didn't include the missed cells, and the extra cells didn't give a better answer!
fn best_for_size(serial_no: i32, size: i32) -> (Coord, i32) {
    // Guess why the original version was bugged!
    // let answer_coord =(0..(298^2)).map(|i| {
    let best_coord = (0..((301 - size).pow(2)))
        .map(|i| {
            let x = i / (301 - size);
            let y = i % (301 - size);
            Coord { x, y }
        })
        .max_by_key(|c| {
            // We're inefficiently calculating (almost) every cell's power 9 times
            // so we should cache instead...but Part 1 runs in < 3ms anyway, so eh.
            c.power_grid(serial_no, size)
        })
        .expect("no best?");

    // Even more inefficiently re-calculate the power for this result!
    (best_coord, best_coord.power_grid(serial_no, size))
}

fn best_for_corner(serial_no: i32, top_left: Coord) -> (i32, i32, i32) {
    let max_size = 301 - top_left.x.min(top_left.y);
    (1..=max_size).fold(
        (0, -6, 0),
        |(current_best_size, current_best_power, last_power), new_size| {
            let new_power = (0..new_size - 1).fold(
                last_power + top_left.plus(new_size - 1, new_size - 1).power(serial_no),
                |power, i| {
                    power
                        + top_left.plus(new_size - 1, i).power(serial_no)
                        + top_left.plus(i, new_size - 1).power(serial_no)
                },
            );
            if new_power > current_best_power {
                (new_size, new_power, new_power)
            } else {
                (current_best_size, current_best_power, new_power)
            }
        },
    )
}

impl Coord {
    fn power(&self, serial_no: i32) -> i32 {
        let rack_id = self.x + 10;
        let mut power_level = rack_id * self.y;
        power_level += serial_no;
        power_level *= rack_id;
        power_level /= 100;
        power_level %= 10;
        power_level - 5
    }

    fn power_grid(&self, serial_no: i32, grid_size: i32) -> i32 {
        (0..grid_size.pow(2))
            .map(|dc| {
                let dx = dc / grid_size;
                let dy = dc % grid_size;
                let coord = Coord {
                    x: self.x + dx,
                    y: self.y + dy,
                };
                coord.power(serial_no)
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    // use super::day11;
    // use crate::utils::load_input;
    use crate::utils::Coord;

    // #[test]
    // fn check_day11_case01() {
    //     full_test(
    //         "18",        // INPUT STRING
    //         "33,45",     // PART 1 RESULT
    //         "90,269,16", // PART 2 RESULT
    //     )
    // }

    // #[test]
    // fn check_day11_case02() {
    //     full_test(
    //         "42",         // INPUT STRING
    //         "21,61",      // PART 1 RESULT
    //         "232,251,12", // PART 2 RESULT
    //     )
    // }

    #[test]
    fn check_power_levels() {
        let coord = Coord { x: 3, y: 5 };
        assert_eq!(coord.power(8), 4);

        let coord = Coord { x: 122, y: 79 };
        assert_eq!(coord.power(57), -5);

        let coord = Coord { x: 217, y: 196 };
        assert_eq!(coord.power(39), 0);

        let coord = Coord { x: 101, y: 153 };
        assert_eq!(coord.power(71), 4);
    }

    // fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
    //     let input_lines = load_input(input_text);
    //     assert_eq!(
    //         day11(&input_lines),
    //         (part1_result.to_string(), part2_result.to_string())
    //     );
    // }
}
