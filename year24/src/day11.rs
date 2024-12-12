// Potential improvements:
//

use std::collections::HashMap;

pub fn day11(input_lines: &[Vec<String>]) -> (String, String) {
    let mut stone_collection = HashMap::new();
    input_lines[0][0]
        .split_ascii_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .for_each(|num| {
            stone_collection
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    (0..25).for_each(|_| {
        stone_collection = blink(&stone_collection);
    });

    let answer1 = stone_collection.values().sum::<i64>();

    (25..75).for_each(|_| {
        stone_collection = blink(&stone_collection);
    });

    let answer2 = stone_collection.values().sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn blink(old_collection: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_collection = HashMap::new();
    for (val, &count) in old_collection {
        if *val == 0 {
            new_collection
                .entry(1)
                .and_modify(|existing_count| *existing_count += count)
                .or_insert(count);
        } else if val.ilog10() % 2 == 1 {
            let factor = 10i64.pow((val.ilog10() + 1) / 2);
            new_collection
                .entry(val / factor)
                .and_modify(|existing_count| *existing_count += count)
                .or_insert(count);
            new_collection
                .entry(val % factor)
                .and_modify(|existing_count| *existing_count += count)
                .or_insert(count);
        } else {
            new_collection
                .entry(val * 2024)
                .and_modify(|existing_count| *existing_count += count)
                .or_insert(count);
        }
    }
    new_collection
}

#[cfg(test)]
mod tests {
    use super::day11;
    use crate::utils::load_input;

    #[test]
    fn check_day11_case01() {
        full_test(
            "125 17",         // INPUT STRING
            "55312",          // PART 1 RESULT
            "65601038650482", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day11(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
