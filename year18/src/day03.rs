// Potential improvements:
//

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// My third solution: attempt to just look at every square inch, and for each one, determine if it's part of 2 or more claims.
// It turns out it's really slow for Part 1 as well...but the weird thing is that it's FASTER if we do looping without a break claim
// to skip unnecessary work than when I have the break claim in, which seems wild.

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let all_claims = input_lines[0]
        .iter()
        .map(|line| {
            let claim = Claim::from_line(line);
            (claim.id, claim)
        })
        .collect::<HashMap<usize, Claim>>();
    let (all_upper_x, all_upper_y): (Vec<usize>, Vec<usize>) = all_claims
        .values()
        .map(|claim| (claim.high_x, claim.high_y))
        .unzip();
    let (max_x, max_y) = (
        all_upper_x.iter().max().expect("No max x"),
        all_upper_y.iter().max().expect("No max y"),
    );

    let mut non_clashing_claim_ids: HashSet<usize> = HashSet::from_iter(1..all_claims.len() + 1);
    let mut clashing_squares = 0;

    // This loop is approximately 1 billion entries (each for has about 1000 elements).  This takes nearly a second in release mode if we
    // comment out all the stuff inside the if claim.is_point_in() branch. But if we enable it - and in particular the break - this time DOUBLES
    // Why does it take longer with break enabled?
    for x in 0..*max_x {
        for y in 0..*max_y {
            let mut claimed = false;
            let mut claiming_id = 0;
            for claim in all_claims.values() {
                if claim.is_point_in(x, y) {
                    if claimed {
                        clashing_squares += 1;
                        non_clashing_claim_ids.remove(&claiming_id);
                        non_clashing_claim_ids.remove(&claim.id);
                        break;
                    } else {
                        claimed = true;
                        claiming_id = claim.id;
                    }
                }
            }
        }
    }

    // We've already eliminated any claim which was the first or second ID in a given clash,
    // but we might have claims that only clashed as the third claim or higher.
    // For remaining candidates, loop each one and see if they clash any original claim: we can bail as soon as we find one.
    let answer2 = non_clashing_claim_ids
        .iter()
        .find(|claim_id| {
            let candidate_claim = all_claims.get(claim_id).expect("Couldn't find the claim!");
            all_claims.values().all(|other_claim| {
                candidate_claim.id == other_claim.id
                    || candidate_claim.is_claim_disjoint(other_claim)
            })
        })
        .expect("Couldn't find a disjoint claim");

    // We've skipped checking that there isn't a second one!

    (format!("{}", clashing_squares), format!("{}", answer2))
}

struct Claim {
    id: usize,
    low_x: usize,
    low_y: usize,
    high_x: usize,
    high_y: usize,
}

impl Claim {
    fn new(id: usize, low_x: usize, low_y: usize, high_x: usize, high_y: usize) -> Self {
        Self {
            id,
            low_x,
            low_y,
            high_x,
            high_y,
        }
    }

    fn from_line(line: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        re.captures(line)
            .map(|cap| {
                let id = cap[1].parse::<usize>().expect("Didn't parse ID");
                let x = cap[2].parse::<usize>().expect("Didn't parse x");
                let y = cap[3].parse::<usize>().expect("Didn't parse y");
                let width = cap[4].parse::<usize>().expect("Didn't parse width");
                let height = cap[5].parse::<usize>().expect("Didn't parse height");
                Claim::new(id, x, y, x + width, y + height)
            })
            .expect("Regex didn't match")
    }

    fn is_point_in(&self, x: usize, y: usize) -> bool {
        x >= self.low_x && x < self.high_x && y >= self.low_y && y < self.high_y
    }

    fn is_claim_disjoint(&self, other: &Claim) -> bool {
        self.low_x >= other.high_x
            || other.low_x >= self.high_x
            || self.low_y >= other.high_y
            || other.low_y >= self.high_y
    }
}

// My first/second solution (just look at the 'interesting' co-ordinates, determine a minimal set of overlap info, then size the overlaps)
// This was slower than the version without the 'interesting' filtering, and just counting the rows directly.

// pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
//     // Convert the input into a set of claims.
//     let all_claims = input_lines[0].iter().map(|line| Claim::new(&line)).collect::<Vec<Claim>>();
//     let (x_values, y_values) = all_claims.iter().fold((HashSet::new(), HashSet::new()), |(x_set, y_set), claim| (x_set.union(&claim.x_coords()).cloned().collect(), y_set.union(&claim.y_coords()).cloned().collect()));
//     let mut x_values = x_values.into_iter().collect::<Vec<usize>>();
//     x_values.sort_unstable();
//     let mut y_values = y_values.into_iter().collect::<Vec<usize>>();
//     y_values.sort_unstable();

//     let mut clashing_claim_ids = HashSet::new();

//     // find the rectangles that are shared by a claim.
//     let answer1: usize = x_values.iter().enumerate().map(|(i, x)|
//         y_values.iter().enumerate().filter_map(|(j, y)|
//             {
//                 let claims_containing_point = all_claims.iter().filter(|claim| claim.x_value_in_claim(x) && claim.y_value_in_claim(y)).collect::<Vec<&Claim>>();
//                 if claims_containing_point.len() > 1 {
//                     clashing_claim_ids = clashing_claim_ids.union(&claims_containing_point.iter().map(|claim| claim.id).collect::<HashSet<_>>()).cloned().collect::<HashSet<usize>>();
//                     let width = x_values[i+1] - x;
//                     let height = y_values[j+1] - y;
//                     Some(width * height)
//                 } else {
//                     None
//                 }
//             }
//         ).sum::<usize>()
//     ).sum();

//     let all_ids = all_claims.iter().map(|claim| claim.id).collect::<HashSet<usize>>();
//     let mut unique_ids = all_ids.difference(&clashing_claim_ids);
//     let answer2 = unique_ids.next().expect("Didn't have a single unique ID");
//     assert_eq!(unique_ids.next(), None);

//     (format!("{}", answer1), format!("{}", answer2))
// }

// struct Claim {
//     id: usize,
//     top_left: Coordinate,
//     width: usize,
//     height: usize,
// }

// impl Claim {
//     fn new(line: &str) -> Self {
//         // #2 @ 3,1: 4x4
//         let split_line = line.split(" @ ").collect::<Vec<&str>>();
//         assert_eq!(split_line.len(), 2);

//         let mut id_chars = split_line.first().unwrap().chars();
//         id_chars.next();

//         let split_line = split_line[1].split(": ").collect::<Vec<&str>>();
//         assert_eq!(split_line.len(), 2);

//         let top_left = Coordinate::new(split_line[0]);

//         let split_line = split_line[1].split('x').collect::<Vec<&str>>();
//         assert_eq!(split_line.len(), 2);

//         Self {
//             id: id_chars.as_str().parse::<usize>().expect("Couldn't parse the ID"),
//             top_left,
//             width: split_line[0].parse::<usize>().expect("Couldn't parse width"),
//             height: split_line[1].parse::<usize>().expect("Couldn't parse height"),
//         }
//     }

//     fn x_coords(&self) -> HashSet<usize> {
//         let mut x_coords = HashSet::new();
//         x_coords.insert(self.top_left.x);
//         x_coords.insert(self.top_left.x + self.width);
//         x_coords
//     }
//     fn y_coords(&self) -> HashSet<usize> {
//         let mut y_coords = HashSet::new();
//         y_coords.insert(self.top_left.y);
//         y_coords.insert(self.top_left.y + self.height);
//         y_coords
//     }
//     fn x_value_in_claim(&self, x: &usize) -> bool {
//         *x >= self.top_left.x && *x < (self.top_left.x + self.width)
//     }
//     fn y_value_in_claim(&self, y: &usize) -> bool {
//         *y >= self.top_left.y && *y < (self.top_left.y + self.height)
//     }
// }

// struct Coordinate {
//     x: usize,
//     y: usize,
// }

// impl Coordinate {
//     // Parse from the form "12,34" to Coordinate { x: 12, y: 34 }
//     fn new(values: &str) -> Self {
//         let split_values = values.split(",").collect::<Vec<&str>>();
//         assert_eq!(split_values.len(), 2);
//         Coordinate {
//             x: split_values[0].parse::<usize>().expect("Couldn't parse the coordinate"),
//             y: split_values[1].parse::<usize>().expect("Couldn't parse the coordinate"),
//         }
//     }
// }

// UT

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2", // INPUT STRING
            "4", // PART 1 RESULT
            "3", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// // ASLT's solution (no markups from me)
// // Potential improvements:
// //
// use std::collections::HashSet;
// use itertools::Itertools;
// use regex::Regex;
// use std::str::FromStr;
// use std::time::Instant;

// #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
// struct SquareInch {
//     x: i32,
//     y: i32
// }

// #[derive(Debug, Eq, Hash, PartialEq)]
// struct FabricClaim {
//     id: i32,
//     top_left: SquareInch,
//     top_right: SquareInch,
//     bottom_left: SquareInch,
//     width: i32,
//     height: i32,
// }

// impl FabricClaim {
//     fn new(id: i32, x: i32, y: i32, width: i32, height: i32) -> Self {
//         Self {
//             id,
//             top_left: SquareInch { x, y },
//             top_right: SquareInch { x: x+width - 1, y },
//             bottom_left: SquareInch { x, y: y+height - 1 },
//             width,
//             height,
//         }
//     }

//     fn all_sq_inches(&self) -> Vec<SquareInch> {
//         let mut sq_inches = Vec::with_capacity(
//             self.height as usize * self.width as usize
//         );
//         for x in self.top_left.x..self.top_right.x+1{
//             for y in self.top_left.y..self.bottom_left.y+1 {
//                 sq_inches.push( SquareInch { x, y } );
//             }
//         }
//         sq_inches
//     }

//     fn from_input_line(input_line: &str) -> FabricClaim {
//         let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
//         re.captures(input_line).map(|cap| {
//             let id = FromStr::from_str(&cap[1]).unwrap();
//             let x = FromStr::from_str(&cap[2]).unwrap();
//             let y = FromStr::from_str(&cap[3]).unwrap();
//             let width = FromStr::from_str(&cap[4]).unwrap();
//             let height = FromStr::from_str(&cap[5]).unwrap();
//             FabricClaim::new(id, x, y, width, height)
//         }).unwrap()
//     }

//     fn overlaps(&self, other: &FabricClaim) -> bool {
//         if self.top_left.x > other.top_right.x ||
//         self.top_right.x < other.top_left.x ||
//         self.top_left.y > other.bottom_left.y ||
//         self.bottom_left.y < other.top_left.y {
//             false
//         } else {
//             true
//         }
//     }
// }

// pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
//     let now = Instant::now();

//     let fabric_claims: HashSet<FabricClaim> = input_lines[0]
//         .iter()
//         .fold(HashSet::new(), |mut map, line| {
//             let fabric_claim = FabricClaim::from_input_line(line);
//             map.insert(fabric_claim);
//             map
//         }
//     );

//     println!("Finished parsing input lines after {}ms.", now.elapsed().as_millis());

//     let (_, contested_inches) = fabric_claims
//         .iter()
//         .fold(
//         (HashSet::<SquareInch>::new(), HashSet::<SquareInch>::new()),
//             |(mut claimed, mut contested), claim| {
//             let square_inches = claim.all_sq_inches();
//             for square_inch in square_inches {
//                 if !claimed.insert(square_inch) {
//                     contested.insert(square_inch);
//                 }
//             }
//             (claimed, contested)
//         }
//     );

//     let all_claim_ids: HashSet<i32> = HashSet::from_iter(1..fabric_claims.len() as i32 + 1);
//     let mut contested_claim_ids = HashSet::<i32>::new();
//     for combination in fabric_claims.iter().combinations(2) {
//         let this_claim = combination[0];
//         let that_claim = combination[1];
//         if this_claim.overlaps(that_claim) {
//             contested_claim_ids.insert(this_claim.id);
//             contested_claim_ids.insert(that_claim.id);
//         }
//     }

//     let answer1 = contested_inches.len();
//     let answer2: Vec<&i32> = all_claim_ids.difference(&contested_claim_ids).collect();
//     (format!("{}", answer1), format!("{:?}", answer2[0]))
// }
