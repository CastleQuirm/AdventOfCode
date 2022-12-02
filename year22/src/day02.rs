pub fn day02(input_lines: &str) -> (String, String) {
    let (scores1, scores2): (Vec<u64>, Vec<u64>) =
        input_lines.lines().map(hardcoded_matching).unzip();
    (
        format!("{}", scores1.iter().sum::<u64>()),
        format!("{}", scores2.iter().sum::<u64>()),
    )
}

fn hardcoded_matching(line: &str) -> (u64, u64) {
    // part 1: (A, B, C) = (X, Y, Z) = (Rock, Paper, Scissors) = (1, 2, 3)
    // part 2: (x, Y, Z) = (lose, draw, win)
    match line {
        "A X" => (4, 3),
        "A Y" => (8, 4),
        "A Z" => (3, 8),
        "B X" => (1, 1),
        "B Y" => (5, 5),
        "B Z" => (9, 9),
        "C X" => (7, 2),
        "C Y" => (2, 6),
        "C Z" => (6, 7),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day02_both_case1() {
        assert_eq!(
            day02(
                "A Y
B X
C Z"
            ),
            ("15".to_string(), "12".to_string())
        )
    }
}
