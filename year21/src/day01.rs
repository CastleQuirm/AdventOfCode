// Potential improvements:
// 1. Would be nice to commonize to handle different spacings of numbers to compare?
// 2. Would be nice to do something neater than the i >= X check (probably .nth().is_some() or something?)
// 3. Would be nice to make it a single line.

pub fn day01(input_lines: &[String]) -> (u64, u64) {
    let input_numbers: Vec<u64> = input_lines
        .iter()
        .map(|line| line.parse::<u64>().expect("Line wasn't a number"))
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..input_numbers.len() {
        if i >= 1 && input_numbers[i] > input_numbers[i - 1] { part1 += 1; }
        if i >= 3 && input_numbers[i] > input_numbers[i - 3] { part2 += 1; }
    }
    
    (part1, part2)
}