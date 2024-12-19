// Potential improvements:
//

use std::collections::HashMap;

pub fn day19(input_lines: &[Vec<String>]) -> (String, String) {
    let mut available_designs = input_lines[0][0].split(", ").collect::<Vec<_>>();
    available_designs.sort_by_key(|a| a.len());
    let mut useful_designs = Vec::new();
    for design in &available_designs {
        if !can_make(design, &useful_designs) {
            useful_designs.push(design);
        }
    }

    let possible_designs = input_lines[1]
        .iter()
        .filter(|pattern| can_make(pattern, &useful_designs))
        .collect::<Vec<&String>>();

    let answer1 = possible_designs.len();
    let mut dictionary = HashMap::from([("".to_string(), 1)]);
    for text in useful_designs {
        dictionary.insert(text.to_string(), 1);
    }

    let answer2 = possible_designs
        .iter()
        .map(|pattern| {
            // println!("Pattern");
            (1..pattern.len()).for_each(|len| {
                count_make(pattern, len, &available_designs, &mut dictionary);
            });
            count_make(pattern, pattern.len(), &available_designs, &mut dictionary)
        })
        .sum::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn can_make(pattern: &str, available_designs: &[&str]) -> bool {
    if pattern.is_empty() {
        true
    } else {
        available_designs.iter().any(|prefix| {
            pattern
                .strip_prefix(prefix)
                .is_some_and(|remainder| can_make(remainder, available_designs))
        })
    }
}

fn count_make(
    pattern: &str,
    len: usize,
    available_designs: &[&str],
    dictionary: &mut HashMap<String, usize>,
) -> usize {
    let shortened_pattern = &pattern[0..len];
    if let Some(value) = dictionary.get(shortened_pattern) {
        return *value;
    }
    let solutions = available_designs
        .iter()
        .flat_map(|ending| {
            shortened_pattern
                .strip_suffix(ending)
                .and_then(|earlier| dictionary.get(earlier))
        })
        .sum::<usize>();
    dictionary.insert(shortened_pattern.to_string(), solutions);
    solutions
}

#[cfg(test)]
mod tests {
    use super::day19;
    use crate::utils::load_input;

    #[test]
    fn check_day19_case01() {
        full_test(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb", // INPUT STRING
            "6",  // PART 1 RESULT
            "16", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day19(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
