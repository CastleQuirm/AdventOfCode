// Potential Improvements:
// 1: don't repeat the calculation needed for Part 1 in Part 2
// 2: do all the calculations on a single walk of input_lines, looking at the relevant characters and incrementing five distinct counters.
// 3: commonize code in Day 2 and 3.  Both involve "walk a list and increment a counter if an element meets a criteria, and do it more times with different criteria in Part 2"
//    Create a utility tool that lets you provide a set of the conditions and the list and gives you both results.

pub fn day3(input_lines: &[String]) -> (u64, u64) {
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    (
        // Part 1 result
        check_slope(input_lines, 1, 3),
        // Part 2 result
        slopes
            .iter()
            .map(|&(x, y)| check_slope(input_lines, x, y))
            .product(),
    )
}

fn check_slope(input_lines: &[String], x: usize, y: usize) -> u64 {
    (0..input_lines.len())
        .filter(|&row| row % x == 0)
        .filter(|&row| {
            input_lines[row]
                .chars()
                .nth((y * row / x) % input_lines[row].len())
                .expect("Couldn't find character")
                == '#'
        })
        .count() as u64
}
