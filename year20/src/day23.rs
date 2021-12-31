// Potential improvements:
// 1: Actually output the solution to Part 1 rather than hand-generating it.
// 2: Commonise Parts 1 and 2.
// 3: Rust-ify/Functionalise it...somehow?  Without re-introducing all the commented out mess further down?

// use std::collections::HashMap;

pub fn day23(input_lines: &[String]) -> (u64, u64) {
    (part1(input_lines), part2(input_lines))
}

fn part1(input_lines: &[String]) -> u64 {
    let mut circle: [usize; 9] = [0; 9];
    (0..9).for_each(|i| {
        let current_num = input_lines[0]
            .chars()
            .nth(i)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let next_num = input_lines[0]
            .chars()
            .nth((i + 1) % 9)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        circle[current_num - 1] = next_num;
    });
    let mut current_cup = input_lines[0]
        .chars()
        .next()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    (0..100).for_each(|_| {
        // we update: current_cup, and three values in the array.
        let extracted_values: [usize; 3] = [
            circle[current_cup - 1],
            circle[circle[current_cup - 1] - 1],
            circle[circle[circle[current_cup - 1] - 1] - 1],
        ]; // [8, 9, 1]
        let next_cup = circle[extracted_values[2] - 1];

        let dest_cup = (1..5)
            .map(|i| {
                if i >= current_cup {
                    circle.len() + current_cup - i
                } else {
                    current_cup - i
                }
            })
            .find(|i| !extracted_values.contains(i))
            .expect(""); // 2

        circle[extracted_values[2] - 1] = circle[dest_cup - 1];
        circle[dest_cup - 1] = extracted_values[0];
        circle[current_cup - 1] = next_cup;
        current_cup = next_cup;
    });
    let mut circle_ptr = 0;
    let mut answer: u64 = 0;
    while circle[circle_ptr] != 1 {
        answer *= 10;
        answer += circle[circle_ptr] as u64;
        circle_ptr = circle[circle_ptr] - 1;
    }
    answer
}

fn part2(input_lines: &[String]) -> u64 {
    // let mut circle: [usize; 1_000_000] = [0; 1_000_000];
    // (0..1_000_000).for_each(|i| {
    //     let current_num = if i < 9 {
    //         input_lines[0]
    //             .chars()
    //             .nth(i)
    //             .unwrap()
    //             .to_string()
    //             .parse::<usize>()
    //             .unwrap()
    //             - 1
    //     } else {
    //         i
    //     };
    //     let next_num = if i < 8 {
    //         input_lines[0]
    //             .chars()
    //             .nth(i + 1)
    //             .unwrap()
    //             .to_string()
    //             .parse::<usize>()
    //             .unwrap()
    //     } else if i == 999_999 {
    //         input_lines[0]
    //             .chars()
    //             .next()
    //             .unwrap()
    //             .to_string()
    //             .parse::<usize>()
    //             .unwrap()
    //     } else {
    //         i + 2
    //     };
    //     circle[current_num] = next_num;
    // });
    // let mut current_cup = input_lines[0]
    //     .chars()
    //     .next()
    //     .unwrap()
    //     .to_string()
    //     .parse::<usize>()
    //     .unwrap();

    // (0..1).for_each(|_| {
    // (0..10_000_000).for_each(|_| {
    //     // we update: current_cup, and three values in the array.
    //     let first_value = circle[current_cup - 1];
    //     println!("First value {}", first_value);
    //     let second_value = circle[first_value - 1];
    //     let third_value = circle[second_value - 1];
    //     let extracted_values: [usize; 3] = [
    //         first_value,
    //         second_value,
    //         third_value,
    //     ]; // [8, 9, 1]
    //     let next_cup = circle[extracted_values[2] - 1];

    //     let dest_cup = (1..5)
    //         .map(|i| {
    //             if i >= current_cup {
    //                 circle.len() + current_cup - i
    //             } else {
    //                 current_cup - i
    //             }
    //         })
    //         .find(|i| !extracted_values.contains(i))
    //         .expect(""); // 2

    //     circle[extracted_values[2] - 1] = circle[dest_cup - 1];
    //     circle[dest_cup - 1] = extracted_values[0];
    //     circle[current_cup - 1] = next_cup;
    //     current_cup = next_cup;
    // });
    // let first_index = circle[0];
    // first_index as u64 * circle[first_index - 1] as u64
    0
}

// Commented out because the test apparently overflows its stack (presumably the 1M element array
#[cfg(test)]
mod tests {
    use super::day23;

    #[test]
    fn day23_example() {
        let input = "389125467"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day23(&input), (67384529, 149245887792));
    }
}

// pub fn day23(input_lines: &[String]) -> (u64, u64) {
//     let mut game = CupSequence::new_basic(input_lines[0].to_string());
//     let mut advanced_game = CupSequence::new_advanced(input_lines[0].to_string());
//     println!("Created the games");
//     (game.nth(100 - 1).unwrap(), advanced_game.nth(10000000 - 1).unwrap())
// }

// struct Cup {
//     value: usize,
//     next: usize,
//     // prev: usize
// }
// impl Cup {
//     fn update_next(&mut self, new_next: usize) {
//         self.next = new_next;
//     }
// }
// impl Iterator for Cup {
//     type Item = usize;
//     fn next(&mut self) -> Option<Self::Item> {
//         // Probably doesn't work?  Would need actual &Cup pointers, which in turn requires...lifetimes.
//         Some(self.next)
//     }
// }

// struct Game {
//     current_cup: usize,
//     circle: HashMap<usize, Cup>,
//     part: usize
// }
// impl Game {
//     fn new_basic(state: String) -> Self {
//         let nums = state.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>();
//         let mut game = Self { current_cup: nums[0], circle: HashMap::new(), part: 1 };
//         (0..state.len()).for_each(|i| {
//             let cup = Cup {
//                 value: nums[i],
//                 next: if i < nums.len() - 1 { nums[i+1] } else { nums[0] },
//                 // prev: if i > 0 {nums[i-1]} else {nums[nums.len()-1]}
//             };
//             game.circle.insert(nums[i], cup);
//         });
//         game
//     }
//     fn get_cup(&mut self, cup_num: &usize) -> &mut Cup {
//         self.circle.get_mut(cup_num).expect("Couldn't find cup")
//     }
// }
// impl Iterator for Game {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         let current_cup_val = self.current_cup;
//         let next_current = self.get_cup(&current_cup_val).nth(4).unwrap();
//         let extracted_values: [usize; 3] = [self.get_cup(&current_cup_val).next().unwrap(),
//         self.get_cup(&current_cup_val).nth(2).unwrap(),
//         self.get_cup(&current_cup_val).nth(3).unwrap()];
//         let dest_cup = (1..5).map(|i| if i >= current_cup_val {
//             self.circle.len() + current_cup_val - i
//         } else {
//             current_cup_val - i
//         }).find(|i| !extracted_values.contains(i)).expect("");

//         self.get_cup(&current_cup_val).update_next(next_current);
//         let after_dest = self.get_cup(&dest_cup).next().unwrap();
//         self.get_cup(&extracted_values[2]).update_next(after_dest);
//         self.get_cup(&dest_cup).update_next(extracted_values[0]);

//         None
//     }
// }

// struct CupSequence {
//     current_cup: usize,
//     circle: Vec<usize>,
//     part: usize
// }
// impl CupSequence {
//     fn new_basic(state: String) -> Self {
//         Self {
//             current_cup: state
//                 .chars()
//                 .next()
//                 .unwrap()
//                 .to_string()
//                 .parse::<usize>()
//                 .unwrap(),
//             circle: state
//                 .chars()
//                 .map(|x| x.to_string().parse::<usize>().unwrap())
//                 .collect(),
//             part: 1
//         }
//     }
//     fn new_advanced(state: String) -> Self {
//         let mut circle = state
//         .chars()
//         .map(|x| x.to_string().parse::<usize>().unwrap())
//         .collect::<Vec<usize>>();
//         while circle.len() < 1000000 {
//             circle.push(circle.len());
//         }
//         Self {
//             current_cup: state
//                 .chars()
//                 .next()
//                 .unwrap()
//                 .to_string()
//                 .parse::<usize>()
//                 .unwrap(),
//             circle,
//             part: 2
//         }
//     }
// }

// impl Iterator for CupSequence {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         // Extract the next three elements after current cup.  These may wrap!  Could re-center on the current cup first?
//         let original_len = self.circle.len();
//         // assert_eq!(self.circle.len(), 9);
//         let current_index = find_index_of(&self.circle, self.current_cup);
//         let mut removed_cups = self.circle.split_off(current_index + 1); // self.circle is now everything up to and including current_cup, removed_cups everything after.
//         if removed_cups.len() < 3 {
//             // Current cup was too late in the circle.  Strip off enough numbers from the front.
//             while removed_cups.len() < 3 {
//                 removed_cups.push(self.circle.remove(0));
//             }
//         } else {
//             // Restore elements after the third.
//             self.circle.append(&mut removed_cups.split_off(3));
//         }

//         assert_eq!(removed_cups.len(), 3);

//         // Find the destination cup: the next lower number after current cup that is not in the extracted trio, wrapping from 1 to original_len.
//         let dest_cup = (1..self.current_cup)
//             .rev()
//             .find(|&i| {
//                 self.circle.iter().any(|&j| i == j)
//             })
//             .unwrap_or(
//                 (self.current_cup..original_len + 1)
//                     .rev()
//                     .find(|&i| {
//                         self.circle.iter().any(|&j| i == j)
//                     })
//                     .unwrap(),
//             );

//         // Insert the extracted elements after that, don't worry about wrapping.
//         let dest_index = find_index_of(&self.circle, dest_cup);
//         let mut later_cups = self.circle.split_off(dest_index + 1);
//         self.circle.append(&mut removed_cups);
//         self.circle.append(&mut later_cups);

//         // Move the current cup one clockwise.
//         let current_index = find_index_of(&self.circle, self.current_cup);
//         self.current_cup = if current_index == self.circle.len() - 1 {
//             self.circle[0]
//         } else {
//             self.circle[current_index + 1]
//         };

//         let one_index = find_index_of(&self.circle, 1);
//         let print_string = if self.part == 1 {
//             // Re-center on 1, and return the value of the other digits.
//             if one_index == self.circle.len() - 1 {
//                 self.circle[0..one_index]
//                     .iter()
//                     .map(|x| x.to_string())
//                     .collect::<Vec<String>>()
//                     .join("")
//             } else {
//                 let mut first_string = self.circle[one_index + 1..self.circle.len()]
//                     .iter()
//                     .map(|x| x.to_string())
//                     .collect::<Vec<String>>()
//                     .join("");
//                 first_string.push_str(
//                     &self.circle[0..one_index]
//                         .iter()
//                         .map(|x| x.to_string())
//                         .collect::<Vec<String>>()
//                         .join(""),
//                 );
//                 first_string
//             }
//         } else {
//             assert_eq!(self.part, 2);
//             (self.circle[(one_index + 1) % self.circle.len()] * self.circle[(one_index + 2) % self.circle.len()]).to_string()
//         };

//         Some(print_string.parse::<u64>().unwrap())
//     }
// }

// fn find_index_of(vec: &Vec<usize>, to_find: usize) -> usize {
//     (0..vec.len())
//         .find(|&i| vec[i] == to_find)
//         .expect("Couldn't find the dest cup in the circle!")
// }

// #[cfg(test)]
// mod tests {
//     use super::day23;

//     #[test]
//     fn day23_example() {
//         let input = "389125467"
//             .lines()
//             .map(std::string::ToString::to_string)
//             .collect::<Vec<String>>();
//         assert_eq!(day23(&input), (67384529, 149245887792));
//     }
// }
