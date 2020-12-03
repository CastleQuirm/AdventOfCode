pub fn day3(input_lines: &[String]) -> (u64, u64) {
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    (
        check_slope(input_lines, 1, 3),
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
