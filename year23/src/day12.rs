// Potential improvements:
//

use itertools::Itertools;

pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0].iter().map(|line| solve1(line)).sum::<usize>();
    // let answer2 = 0;
    let answer2 = input_lines[0].iter().map(|line| solve2(line)).sum::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn solve1(line: &str) -> usize {
    println!("HELLO! Working on {line}");
    let (spring_list, picross_vals) = line.split_once(' ').expect("Unexpected format");
    calculate(spring_list, picross_vals)
}

fn solve2(line: &str) -> usize {
    let (spring_list, picross_vals) = line.split_once(' ').expect("Unexpected format");
    let spring_list = expand5(spring_list, "?");
    let picross_vals = expand5(picross_vals, ",");
    // Optimisation 1: if we can spot lines where the joiners _have_ to be `.`, we only need to process them in their basic form and
    // raise the answer ^5.
    
    // panic!("Whooooa there this will take way too long");
    // calculate(&spring_list, &picross_vals)
    intelligent_solve(&spring_list, &picross_vals)
    // todo!()
}

fn expand5(compressed: &str, split_char: &str) -> String {
    let compressed = compressed.to_owned() + split_char;
    let compressed = compressed.repeat(5);
    compressed.strip_suffix(split_char).unwrap().to_owned()
}

fn intelligent_solve(spring_list: &str, picross_vals: &str) -> usize {
    // Work through step by step?
    todo!()
}

fn calculate(spring_list: &str, picross_vals: &str) -> usize {
    let picross_vals = picross_vals.split(',').map(|value| value.parse::<usize>().expect("Not a valid number of springs")).collect::<Vec<usize>>();
    let missing_springs = picross_vals.iter().sum::<usize>() - spring_list.matches('#').count();
    spring_list.match_indices('?').map(|(index, _)| index).combinations(missing_springs).filter(|working_indices| test_working_indices(spring_list, &picross_vals, working_indices)).count()
}

fn test_working_indices(spring_list: &str, picross_vals: &[usize], working_indices: &[usize]) -> bool {
    let mut converted_springs = spring_list.to_owned();
    for spring in working_indices {
        converted_springs.replace_range(spring..=spring, "#");
    }
    let converted_springs = converted_springs.replace('?', ".");
    // println!("Check string {:?}", converted_springs);
    picross_vals == converted_springs.split('.').filter_map(|split| if split.is_empty() { None } else {Some(split.len())}).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
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
