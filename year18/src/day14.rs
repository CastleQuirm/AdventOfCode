// Potential improvements:
//

pub fn day14(input_lines: &[Vec<String>]) -> (String, String) {
    let num_recipies = input_lines[0][0].parse::<usize>().expect("");
    // This next line breaks if the input starts with a 0 - which doesn't make much sense for the whole puzzle,
    // but does break one of the test examples in part 2.
    let answer2_target = num_recipies.to_string();
    let target_len = num_recipies.to_string().len();
    let mut recipies = vec![3, 7];
    let mut elf_positions = vec![0, 1];
    let mut answer2 = None;

    while (recipies.len() < num_recipies + 10) || answer2.is_none() {
        create_recipies(&mut recipies, &mut elf_positions);
        if answer2.is_none() && recipies.len() > target_len {
            let potential_answer2 = recipies.len() - target_len - 1;
            if digits_as_str(&recipies[potential_answer2..potential_answer2 + target_len])
                == answer2_target
            {
                answer2 = Some(potential_answer2);
            }
            let potential_answer2 = potential_answer2 + 1;
            if digits_as_str(&recipies[potential_answer2..potential_answer2 + target_len])
                == answer2_target
            {
                answer2 = Some(potential_answer2);
            }
        }
    }

    let answer1 = digits_as_str(&recipies[num_recipies..(num_recipies + 10)]);
    (format!("{}", answer1), format!("{}", answer2.unwrap()))
}

fn create_recipies(recipies: &mut Vec<usize>, elf_positions: &mut Vec<usize>) {
    let new_sum = elf_positions
        .iter()
        .map(|&elf| recipies[elf])
        .sum::<usize>();
    assert!(new_sum < 20);
    if new_sum >= 10 {
        recipies.push(1);
        recipies.push(new_sum - 10);
    } else {
        recipies.push(new_sum);
    }

    *elf_positions = elf_positions
        .iter()
        .map(|&elf| (elf + recipies[elf] + 1) % recipies.len())
        .collect::<Vec<usize>>();
}

fn digits_as_str(recipies: &[usize]) -> String {
    recipies
        .iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .concat()
}

#[cfg(test)]
mod tests {
    use super::day14;
    use crate::utils::load_input;

    #[test]
    fn check_day14_case01() {
        full_test(
            "9",          // INPUT STRING
            "5158916779", // PART 1 RESULT
            "0",          // PART 2 RESULT
        )
    }

    #[test]
    fn check_day14_case02() {
        full_test(
            "59414", // INPUT STRING
            "0",     // PART 1 RESULT
            "5",     // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day14(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
