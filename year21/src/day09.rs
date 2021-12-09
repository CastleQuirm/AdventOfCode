// Potential improvements:
//

pub fn day09(input_lines: &[String]) -> (u64, u64) {
    let height_map: Vec<Vec<u64>> = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<u64>()
                        .expect("Couldn't parse character")
                })
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let max_j = height_map[0].len() - 1;
    let max_i = height_map.len() - 1;

    let part1 = height_map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, space)| {
                    if (j == max_j || space < &height_map[i][j + 1])
                        && (j == 0 || space < &height_map[i][j - 1])
                        && (i == max_i || space < &height_map[i + 1][j])
                        && (i == 0 || space < &height_map[i - 1][j])
                    {
                        Some(space + 1)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        })
        .sum();
    (part1, 0)
}
