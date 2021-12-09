use std::collections::HashSet;

// Potential improvements:
// 1. Make the four direction stuff commonised (Direction with iter?)
//   - Tests when creating the low_points map
//   - The logic of basin_size()
// 2. Better logic for doing the exploration across the map than HashSet insertions
// 3. Coordinate struct

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

    let low_points = height_map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, space)| {
                    if (j == max_j || space < &height_map[i][j + 1])
                        && (j == 0 || space < &height_map[i][j - 1])
                        && (i == max_i || space < &height_map[i + 1][j])
                        && (i == 0 || space < &height_map[i - 1][j])
                    {
                        Some(LowPoint {
                            i,
                            j,
                            depth: *space,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<LowPoint>>()
        })
        .collect::<Vec<LowPoint>>();

    let part1 = low_points
        .iter()
        .fold(0, |value, point| value + point.depth + 1);

    let mut basin_sizes = low_points
        .iter()
        .map(|low_point| low_point.basin_size(&height_map))
        .collect::<Vec<u64>>();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    let part2 = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    (part1, part2)
}

struct LowPoint {
    i: usize,
    j: usize,
    depth: u64,
}

impl LowPoint {
    fn basin_size(&self, height_map: &[Vec<u64>]) -> u64 {
        let max_j = height_map[0].len() - 1;
        let max_i = height_map.len() - 1;

        let mut all_cells: HashSet<(usize, usize)> = HashSet::new();
        all_cells.insert((self.i, self.j));
        let mut cells_to_check: HashSet<(usize, usize)> = HashSet::new();
        cells_to_check.insert((self.i, self.j));

        while !cells_to_check.is_empty() {
            let mut new_cells: HashSet<(usize, usize)> = HashSet::new();
            cells_to_check.iter().for_each(|(i, j)| {
                let i = *i;
                let j = *j;
                let current_depth = height_map[i][j];
                if i > 0 && height_map[i - 1][j] > current_depth && height_map[i - 1][j] < 9 {
                    new_cells.insert((i - 1, j));
                    all_cells.insert((i - 1, j));
                }
                if i < max_i && height_map[i + 1][j] > current_depth && height_map[i + 1][j] < 9 {
                    new_cells.insert((i + 1, j));
                    all_cells.insert((i + 1, j));
                }
                if j > 0 && height_map[i][j - 1] > current_depth && height_map[i][j - 1] < 9 {
                    new_cells.insert((i, j - 1));
                    all_cells.insert((i, j - 1));
                }
                if j < max_j && height_map[i][j + 1] > current_depth && height_map[i][j + 1] < 9 {
                    new_cells.insert((i, j + 1));
                    all_cells.insert((i, j + 1));
                }
            });
            cells_to_check = new_cells;
        }

        all_cells.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::day09;

    #[test]
    fn check_day09() {
        let input_lines = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day09(&input_lines), (15, 1134));
    }
}
