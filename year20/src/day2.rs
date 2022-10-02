// Potential improvements:
// 1. Improve the line parsing.  Investigate nom.
// 2. Single walk of the split_lines vector after we've created it, to get both results.

pub fn day2(input_lines: &[String]) -> (u64, u64) {
    let split_lines = input_lines
        .iter()
        .map(|line| parse_rule_and_password(line))
        .collect::<Vec<RuleAndPassword>>();

    (
        split_lines.clone().into_iter().filter(part1check).count() as u64,
        split_lines.into_iter().filter(part2check).count() as u64,
    )
}

#[derive(Clone)]
struct RuleAndPassword {
    num1: usize,
    num2: usize,
    character: char,
    password: String,
}

fn parse_rule_and_password(line: &str) -> RuleAndPassword {
    let params = line.split(|c| c == ' ' || c == '-').collect::<Vec<&str>>();
    RuleAndPassword {
        num1: params[0].parse().expect("First value wasn't a number"),
        num2: params[1].parse().expect("Second value wasn't a number"),
        character: params[2]
            .chars()
            .next()
            .expect("Didn't find the expected character"),
        password: params[3].to_string(),
    }
}

fn part1check(to_check: &RuleAndPassword) -> bool {
    let letter_count =
        to_check.password.len() - to_check.password.replace(to_check.character, "").len();
    letter_count >= to_check.num1 && letter_count <= to_check.num2
}

fn part2check(to_check: &RuleAndPassword) -> bool {
    let first_char = to_check
        .password
        .chars()
        .nth(to_check.num1 - 1)
        .expect("Password not long enough for first value")
        == to_check.character;
    let second_char = to_check
        .password
        .chars()
        .nth(to_check.num2 - 1)
        .expect("Password not long enough for first value")
        == to_check.character;
    first_char ^ second_char
}
