// Potential improvements:
// 1. Would be nice to commonize to handle different spacings of numbers to compare?
// 2. Would be nice to do something neater than the i >= X check (probably .nth().is_some() or something?)
// 3. Would be nice to make it a single line.
use itertools::Itertools;

pub fn day01(input_lines: &[String]) -> (u64, u64) {
    let input_numbers: Vec<u64> = input_lines
        .iter()
        .map(|line| line.parse::<u64>().expect("Line wasn't a number"))
        .collect();

    // NOTE for part 2, we don't need to track rolling sums: we can just compare to the number three before (as opposed to 1 before)
    // because checking if a + b + c is greater to or less than b + c + d is the same as checking if a is greater to or less than d.

    // Solution v1
    // let mut part1 = 0;
    // let mut part2 = 0;
    // for i in 0..input_numbers.len() {
    //     if i >= 1 && input_numbers[i] > input_numbers[i - 1] { part1 += 1; }
    //     if i >= 3 && input_numbers[i] > input_numbers[i - 3] { part2 += 1; }
    // }

    // Solution v2
    let part1 = input_numbers
        .iter()
        .tuple_windows()
        .filter(|(&a, &b)| a < b)
        .count() as u64;
    let part2 = input_numbers
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(&a, _, _, &d)| a < d)
        .count() as u64;

    // Solution v3 (not working)
    // let (part1, part2) = input_numbers.iter().fold((0, 0), |counts, i| {
    //     if i.next() > i { counts.0 += 1; }
    //     if i.next().next().next() > i { counts.1 += 1; }
    // })

    (part1, part2)
}
