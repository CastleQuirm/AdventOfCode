use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/input/input02")
        .expect("Failed to read file")
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    println!("Day 2 Part 1: {}", count_valid_passwords(&input));
    println!("Day 2 Part 2: {}", "???");

}

fn count_valid_passwords(input: &Vec<String>) -> usize {
    input.clone().into_iter().filter(|password| validate_password(password.to_string())).count()
}

fn validate_password(_password: String) -> bool {
    // Parse the line.  Example: 4-7 z: zzzfzlzzz
    // <lower>-<upper> <letter>: <passwordString>

    true
}