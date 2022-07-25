// Potential improvements:
//

use crate::utils::Coord;
use regex::Regex;
use std::{str::FromStr, string::ParseError};

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    let stars = input_lines[0]
        .iter()
        .map(|line| line.parse::<Star>().expect("Couldn't parse"))
        .collect::<Vec<Star>>();

    // Pick two stars with large velocities and work out what time gives the minimum distance between them.
    let (star_0, star_1) = (
        stars
            .iter()
            .max_by(|a, b| a.velocity.x.cmp(&b.velocity.x))
            .expect("No stars?"),
        stars
            .iter()
            .max_by(|a, b| a.velocity.y.cmp(&b.velocity.y))
            .expect("No stars?"),
    );

    let mut converging = true;
    let mut last_distance = star_0.start.manhattan_dist(&star_1.start);
    let mut t = 1;
    while converging {
        let new_distance = star_0
            .pos_after_time(t)
            .manhattan_dist(&star_1.pos_after_time(t));
        if new_distance < last_distance {
            t += 1;
            last_distance = new_distance;
        } else {
            converging = false;
        }
    }

    // We now have the t with the min distance for these two stars. This isn't guaranteed to be the correct time for the message,
    // but is almost certainly close.  Find the best t, assuming that it won't be more than 2 out and that it'll be the position
    // with the minimum height over all the positions (this is a reasonable bet but not guaranteed...but does work in this case).
    let delta = (-2..=2)
        .min_by(|i, j| {
            fn row_count_at_t(stars: &[Star], t: i32) -> i32 {
                let moved_stars_y_coords = stars
                    .iter()
                    .map(|star| star.pos_after_time(t).y)
                    .collect::<Vec<i32>>();
                let min_y = moved_stars_y_coords.iter().min().expect("No elements?");
                let max_y = moved_stars_y_coords.iter().max().expect("No elements?");
                max_y - min_y
            }

            row_count_at_t(&stars, t + i).cmp(&row_count_at_t(&stars, t + j))
        })
        .expect("No such delta?");

        
    // It would be nice to have something to read the letters but I'm not excited to
    // implement this or find anything that can.

    // Print out the image.
    print_image_at_time_t(t + delta, &stars);

    (format!("{}", 0), format!("{}", t + delta))
}

fn print_image_at_time_t(t: i32, stars: &[Star]) {
    let stars_at_t = stars.iter().map(|star| star.pos_after_time(t));
    let min_x = stars_at_t.clone().map(|pos| pos.x).min().expect("no min?");
    let min_y = stars_at_t.clone().map(|pos| pos.y).min().expect("no min?");

    let mut normalised_stars_at_t = stars_at_t
        .map(|star| Coord {
            x: star.x - min_x,
            y: star.y - min_y,
        })
        .collect::<Vec<Coord>>();
    normalised_stars_at_t.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    let max_x = normalised_stars_at_t
        .iter()
        .map(|pos| pos.x)
        .max()
        .expect("no max?");
    let max_y = normalised_stars_at_t
        .iter()
        .map(|pos| pos.y)
        .max()
        .expect("no max?");

    let mut star_iter = normalised_stars_at_t.into_iter();
    let mut next_star = star_iter.next();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if next_star == Some(Coord { x, y }) {
                print!("#");
                while next_star == Some(Coord { x, y }) {
                    next_star = star_iter.next();
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Wanted to get recap to work, but I also wanted a better strucutre and couldn't get
// recap to do heirarchies, so resorted to captures.

// #[derive(Debug, Clone, Copy, Deserialize, Recap)]
// #[recap(regex = r#"position=<(?P<start_x>-?\d+),\s*(?P<start_y>-?\d+)> velocity=<(?P<vel_x>-?\d+),\s*(?P<vel_y>-?\d+)>"#)]
// struct Star {
//     start_x: i32,
//     start_y: i32,
//     vel_x: i32,
//     vel_y: i32,
// }

#[derive(Debug, Clone, Copy)]
struct Star {
    start: Coord,
    velocity: Coord,
}

// This takes OVER 1 SECOND to parse all the input in debug build!  (Reasonable time in release build)
impl FromStr for Star {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
            .unwrap();
        re.captures(s)
            .map(|cap| {
                let start_x = cap[1].parse::<i32>().expect("Couldn't parse start_x");
                let start_y = cap[2].parse::<i32>().expect("Couldn't parse start_y");
                let vel_x = cap[3].parse::<i32>().expect("Couldn't parse vel_x");
                let vel_y = cap[4].parse::<i32>().expect("Couldn't parse vel_y");
                Ok(Star {
                    start: Coord {
                        x: start_x,
                        y: start_y,
                    },
                    velocity: Coord { x: vel_x, y: vel_y },
                })
            })
            .expect("Couldn't parse input line")
    }
}

impl Star {
    fn pos_after_time(&self, t: i32) -> Coord {
        Coord {
            x: self.start.x + t * self.velocity.x,
            y: self.start.y + t * self.velocity.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day10;
    use crate::utils::load_input;

    #[test]
    fn check_day10_case01() {
        full_test(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>", // INPUT STRING
            "0", // PART 1 RESULT
            "3", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day10(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
