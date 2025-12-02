// Potential improvements:
//

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0][0]
        .split(',')
        .map(|range| {
            let (lower, upper) = range.split_once('-').expect("Badly formatted range");
            let lower = lower.parse::<i64>().expect("Bad lower");
            let upper = upper.parse::<i64>().expect("Bad upper");
            (lower..=upper)
                .filter(|id| {
                    let id = id.to_string();
                    let len = id.len();
                    let substr_len = len / 2;
                    if len % 2 != 0 {
                        return false;
                    }
                    id[0..substr_len] == id[substr_len..]
                })
                .sum::<i64>()
        })
        .sum::<i64>();
    let answer2 = input_lines[0][0]
        .split(',')
        .map(|range| {
            let (lower, upper) = range.split_once('-').expect("Badly formatted range");
            let lower = lower.parse::<i64>().expect("Bad lower");
            let upper = upper.parse::<i64>().expect("Bad upper");
            (lower..=upper)
                .filter(|id| {
                    let id = id.to_string();
                    let len = id.len();
                    (1..len).any(|substr_len| {
                        if len % substr_len != 0 {
                            return false;
                        }
                        let candidate_sub = &id[0..substr_len];
                        (1..len / substr_len)
                            .all(|x| &id[(x * substr_len)..((x + 1) * substr_len)] == candidate_sub)
                    })
                })
                .sum::<i64>()
        })
        .sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",  // INPUT STRING
            "1227775554", // PART 1 RESULT
            "4174379265", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
