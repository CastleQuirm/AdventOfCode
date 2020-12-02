use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("src/bin/input/input02")
        .expect("Failed to read file")
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    println!("Day 2 Part 1: {}", count_valid_passwords(&input));
    println!("Day 2 Part 2: {}", count_actual_valid_passwords(&input));
}

fn count_valid_passwords(input: &[String]) -> usize {
    input.into_iter().filter(|line| validate_password(line.to_string())).count()
}

fn validate_password(line: String) -> bool {
    // Parse the line.  Example: 4-7 z: zzzfzlzzz
    // <lower>-<upper> <letter>: <passwordString>
    // Then: (a) search for <letter> in <passwordString> and (b) confirm the number of occurences is within [lower, upper]

    let line_parts: Vec<&str> = line.split(|c| c == ' ' || c == '-').collect();
    assert!(line_parts.len() == 4);
    let lower_bound = line_parts[0].parse().expect("Lower bound not a number");
    let upper_bound = line_parts[1].parse().expect("Upper bound not a number");
    let required_letter = line_parts[2].chars().nth(0).expect("Mis-parsed line");
    let password = line_parts[3];
    
    let letter_count = password.len() - password.replace(required_letter, "").len();

    letter_count >= lower_bound && letter_count <= upper_bound
}

fn count_actual_valid_passwords(input: &[String]) -> usize {
    input.into_iter().filter(|line| validate_real_password(line.to_string())).count()
}

fn validate_real_password(line: String) -> bool {
    // Parse the line.  Example: 4-7 z: zzzfzlzzz
    // <lower>-<upper> <letter>: <passwordString>
    // Then: (a) search for <letter> in <passwordString> and (b) confirm the number of occurences is within [lower, upper]

    let line_parts: Vec<&str> = line.split(|c| c == ' ' || c == '-').collect();
    assert!(line_parts.len() == 4);
    let lower_bound: usize = line_parts[0].parse().expect("Lower bound not a number");
    let upper_bound: usize = line_parts[1].parse().expect("Upper bound not a number");
    let required_letter = line_parts[2].chars().nth(0).expect("Mis-parsed line");
    let password = line_parts[3];

    let first_is_char = password.chars().nth(lower_bound-1).expect("Password not long enough for lower bound") == required_letter;
    let second_is_char = password.chars().nth(upper_bound-1).expect("Password not long enough for upper bound") == required_letter;

    (first_is_char || second_is_char) && !(first_is_char && second_is_char)
}