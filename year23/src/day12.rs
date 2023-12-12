// Potential improvements:
//

// use itertools::Itertools;

use std::collections::HashMap;

use itertools::Itertools;

pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0].iter().map(|line| solve1(line)).sum::<usize>();
    // let answer2 = 0;
    let answer2 = input_lines[0].iter().map(|line| solve2(line)).sum::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn solve1(line: &str) -> usize {
    // println!("HELLO! Working on {line}");
    let (spring_list, picross_vals) = line.split_once(' ').expect("Unexpected format");
    // calculate(spring_list, picross_vals)
    intelligent_solve(&spring_list, &picross_vals)
}

fn solve2(line: &str) -> usize {
    print!("HELLO! Working on {line}");
    let (spring_list, picross_vals) = line.split_once(' ').expect("Unexpected format");
    let spring_list = expand5(spring_list, "?");
    let picross_vals = expand5(picross_vals, ",");
    // Optimisation 1: if we can spot lines where the joiners _have_ to be `.`, we only need to process them in their basic form and
    // raise the answer ^5? This only catches a few cases though.

    // panic!("Whooooa there this will take way too long");
    // calculate(&spring_list, &picross_vals)
    let answer = intelligent_solve(&spring_list, &picross_vals);
    println!("...got {answer} results");
    answer
    // todo!()
}

fn expand5(compressed: &str, split_char: &str) -> String {
    let compressed = compressed.to_owned() + split_char;
    let compressed = compressed.repeat(5);
    compressed.strip_suffix(split_char).unwrap().to_owned()
}

fn intelligent_solve(spring_list: &str, picross_values: &str) -> usize {
    // Work through step by step?

    // Get the picross values as a vec, reversed for easy popping.
    let picross_values = picross_values.split(',').map(|value| value.parse::<usize>().expect("Not a valid number of springs")).rev().collect::<Vec<usize>>();
    let mut possibilities = HashMap::from([(Possibility { spring_list: spring_list.to_owned() + ".", values: picross_values}, 1)]);
    let mut valid_solutions = 0;

    while !possibilities.is_empty() {
        let possibility = possibilities.keys().clone().sorted_by(|a, b| a.spring_list.len().cmp(&b.spring_list.len())).last().unwrap().clone();
    // while let Some(possibility) = &possibilities.keys().sorted_by(|a, b| a.spring_list.len().cmp(&b.spring_list.len())).last() {
        let multiplier = possibilities.remove(&possibility).unwrap();
        // let multiplier = 1;

        let spring_list = possibility.spring_list.clone();
        // println!("Working possibility: {spring_list}, {:?} - with multiplier {multiplier}", possibility.values);
        let mut remaining_values = possibility.values.clone();
        // If we need new blocks:
        if let Some(next_block) = remaining_values.pop() {
            // println!(" - next_block is {next_block}");
            // ... remove any leading '.'s
            let spring_list = spring_list.trim_start_matches('.').to_owned();
            // ... check the length for the next block:
            if spring_list.len() < next_block {
                // ... - if not enough exist, abort
                // println!("- Abort: not enough space!");
                continue;
            }
            match spring_list.find('.') {
                Some(dot_ix) if dot_ix < next_block => {
                    // ... - if any of them are .s ...
                    if spring_list[0..dot_ix].contains('#') { 
                        // ... and there is at least one # before that, abort
                        // println!("- Abort: . at {dot_ix} but # before that!");
                        continue;
                    } else {
                        // ... and none before that are a #, strip up to and including the .
                        // println!("- Drop the next {dot_ix} + 1 chars and carry on");
                        let new_possibility = Possibility { spring_list: spring_list[dot_ix+1..].to_string(), values: possibility.values.clone() };
                        possibilities.entry(new_possibility).and_modify(|val| *val += multiplier).or_insert(multiplier);
                    }
                }
                _ => {
                    // ... - no .s in the range so maybe a possibility to start here?
                    if spring_list.chars().nth(next_block) == Some('#') {
                        // Can't start here because the immediately after character is a #
                        // println!("- Can't start here because the immediately after character is a #");
                        if spring_list.starts_with('#') {
                            // println!("--- But it needed to start here!");
                            continue;
                        }
                        let new_possibility = Possibility { spring_list: spring_list[1..].to_string(), values: possibility.values.clone() };
                        possibilities.entry(new_possibility).and_modify(|val| *val += multiplier).or_insert(multiplier);
                    } else {
                        // We could start here
                        // println!("- Could start here!");
                        let new_possibility = Possibility { spring_list: spring_list[next_block+1..].to_string(), values: remaining_values };
                        possibilities.entry(new_possibility).and_modify(|val| *val += multiplier).or_insert(multiplier);
                        if spring_list.chars().next() != Some('#') {
                            // ... - if the first character is not a #, we could also choose not to!
                            // println!("- Could ALSO not start here");
                            assert_eq!(spring_list.chars().next(), Some('?'));
                            let new_possibility = Possibility { spring_list: spring_list[1..].to_string(), values: possibility.values.clone() };
                            possibilities.entry(new_possibility).and_modify(|val| *val += multiplier).or_insert(multiplier);
                        }
                    }
                }
            }
        } else {
            // print!("Ran out of blocks...");
            if !spring_list.contains('#') {
                valid_solutions += multiplier;
                // println!("No remaining spaces to fill, it works! Now {valid_solutions} options");
            }
            // println!();
        } 
    }
    
    valid_solutions
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Possibility {
    spring_list: String,
    values: Vec<usize>
}

// fn calculate(spring_list: &str, picross_vals: &str) -> usize {
//     let picross_vals = picross_vals.split(',').map(|value| value.parse::<usize>().expect("Not a valid number of springs")).collect::<Vec<usize>>();
//     let missing_springs = picross_vals.iter().sum::<usize>() - spring_list.matches('#').count();
//     spring_list.match_indices('?').map(|(index, _)| index).combinations(missing_springs).filter(|working_indices| test_working_indices(spring_list, &picross_vals, working_indices)).count()
// }

// fn test_working_indices(spring_list: &str, picross_vals: &[usize], working_indices: &[usize]) -> bool {
//     let mut converted_springs = spring_list.to_owned();
//     for spring in working_indices {
//         converted_springs.replace_range(spring..=spring, "#");
//     }
//     let converted_springs = converted_springs.replace('?', ".");
//     // println!("Check string {:?}", converted_springs);
//     picross_vals == converted_springs.split('.').filter_map(|split| if split.is_empty() { None } else {Some(split.len())}).collect::<Vec<usize>>()
// }

#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
        // full_test(
        //     ".??..??...?##. 1,1,3",  // INPUT STRING
        //     "4", // PART 1 RESULT
        //     "0", // PART 2 RESULT
        // )
        
        full_test(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",  // INPUT STRING
            "21", // PART 1 RESULT
            "525152", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day12(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
