// Potential improvements:
//

use std::collections::{HashMap, HashSet};

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let mut rules = HashMap::<i64, Vec<i64>>::new();
    for line in &input_lines[0] {
        let (before, after) = line.split_once('|').expect("weird");
        let before = before.parse::<i64>().unwrap();
        let after = after.parse::<i64>().unwrap();
        rules
            .entry(before)
            .and_modify(|list| list.push(after))
            .or_insert(Vec::from([after]));
    }

    // println!("{:?}", rules);
    // let mut rules_list = rules.clone().into_iter().collect::<Vec<(i64, Vec<i64>)>>();
    // rules_list.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    // assert!((0..rules_list.len()-1).all(|i| {
    //     let mut smaller_hash = rules_list[i].1.iter().collect::<HashSet<&i64>>();
    //     smaller_hash.insert(&rules_list[i].0);
    //     if smaller_hash == rules_list[i+1].1.iter().collect::<HashSet<&i64>>() {
    //         true
    //     } else {
    //         println!("current row: {:?}, next row {:?}", rules_list[i], rules_list[i+1]);
    //         false
    //     }
    // }));

    let mut answer1 = 0;
    let mut answer2 = 0;

    input_lines[1].iter().for_each(|line| {
        let mut already_printed_pages = HashSet::<i64>::new();
        let pages = line
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut error = false;
        for page in &pages {
            if rules
                .get(page)
                .unwrap_or(&Vec::new())
                .iter()
                .any(|p| already_printed_pages.contains(p))
            {
                error = true;
                break;
            } else {
                already_printed_pages.insert(*page);
            }
        }
        if !error {
            answer1 += pages[pages.len() / 2];
        } else {
            let mut reordered_seq = Vec::new();
            for page in &pages {
                let insert_index = (0..reordered_seq.len())
                    .find(|i| {
                        rules
                            .get(page)
                            .unwrap_or(&Vec::new())
                            .contains(reordered_seq[*i])
                    })
                    .unwrap_or(reordered_seq.len());
                reordered_seq.insert(insert_index, page);
            }
            answer2 += reordered_seq[pages.len() / 2];
        }
    });

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47", // INPUT STRING
            "143", // PART 1 RESULT
            "123", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day05(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
