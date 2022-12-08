pub fn day08(input_lines: &str) -> (String, String) {
    let forest = input_lines
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.try_into().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();
    let (visibles, distances): (Vec<_>, Vec<_>) = (0..forest.len() * forest[0].len())
        .map(|z| {
            let x = z % forest[0].len();
            let y = z / forest[0].len();
            tree_visible_and_scenic_distance(&forest, x, y)
        })
        .unzip();

    let answer1 = visibles.iter().filter(|b| **b).count();
    let answer2 = distances.iter().max().expect("No entries?");
    (format!("{}", answer1), format!("{}", answer2))
}

fn tree_visible_and_scenic_distance(forest: &[Vec<u64>], x: usize, y: usize) -> (bool, usize) {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dx, dy)| visible_from_and_scenic_count(forest, x, y, dx, dy))
        .reduce(|(visible, scenic), (d_visible, d_scenic)| {
            (visible || d_visible, scenic * d_scenic)
        })
        .unwrap()
}

fn visible_from_and_scenic_count(
    forest: &[Vec<u64>],
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> (bool, usize) {
    let test_tree = forest[y][x];
    for i in 1..=usize::max(forest.len(), forest[y].len()) {
        let x_prime: Result<usize, _> =
            (x as isize + i as isize * dx)
                .try_into()
                .and_then(|x_prime| {
                    if x_prime < forest[y].len() {
                        Ok(x_prime)
                    } else {
                        (-1).try_into()
                    }
                });
        let y_prime: Result<usize, _> =
            (y as isize + i as isize * dy)
                .try_into()
                .and_then(|y_prime| {
                    if y_prime < forest.len() {
                        Ok(y_prime)
                    } else {
                        (-1).try_into()
                    }
                });

        if let (Ok(new_x), Ok(new_y)) = (x_prime, y_prime) {
            if forest[new_y][new_x] >= test_tree {
                return (false, i);
            }
        } else {
            // Gone outside the forest, not found anything
            return (true, i - 1);
        }
    }

    // should never reach here
    println!("Panicking: tree at ({x},{y}) with delta ({dx},{dy})");
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(
            day08(
                "30373
25512
65332
33549
35390"
            ),
            ("21".to_string(), "8".to_string())
        )
    }
}
