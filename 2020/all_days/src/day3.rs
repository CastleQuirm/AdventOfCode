pub fn day3(input_lines: &[String]) -> (u64, u64) {
    let run1 = check_slope(input_lines, 1, 1);
    let run2 = check_slope(input_lines, 1, 3);
    let run3 = check_slope(input_lines, 1, 5);
    let run4 = check_slope(input_lines, 1, 7);
    let run5 = check_slope(input_lines, 2, 1);

    (run2, run1 * run2 * run3 * run4 * run5)
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
