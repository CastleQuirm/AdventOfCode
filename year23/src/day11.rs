// Potential improvements:
//

use std::collections::HashSet;

use itertools::Itertools;

use crate::coord::Coord2;

pub fn day11(input_lines: &[Vec<String>]) -> (String, String) {
    let galaxies = input_lines[0].iter().enumerate().flat_map(|(row_ix, row)| {
        row.chars().enumerate().filter_map(|(col_ix, c)| if c == '#' { Some(Coord2 { x: col_ix as i64, y: row_ix as i64 })} else { None }).collect::<HashSet<Coord2>>()
    }).collect::<HashSet<Coord2>>();
    let (cols_with_g, rows_with_g): (HashSet<i64>, HashSet<i64>) = galaxies.iter().map(|g| (g.x, g.y)).unzip();
    let empty_rows = (0i64..(input_lines[0].len() as i64)).filter(|row_ix| !rows_with_g.contains(row_ix)).collect::<Vec<i64>>();
    let empty_cols = (0i64..input_lines[0][0].len() as i64).filter(|col_ix| !cols_with_g.contains(col_ix)).collect::<Vec<i64>>();
    // println!("Empty rows: {:?}", empty_rows);
    // println!("Empty cols: {:?}", empty_cols);
    let answer1 = galaxies.iter().combinations(2).map(|pair| {
        assert_eq!(pair.len(), 2);
        let (galaxy1, galaxy2) = (*pair[0], *pair[1]);
        let basic_dist = galaxy1.manhattan_dist(&galaxy2);
        let (min_x, max_x) = (galaxy1.x.min(galaxy2.x), galaxy1.x.max(galaxy2.x));
        let (min_y, max_y) = (galaxy1.y.min(galaxy2.y), galaxy1.y.max(galaxy2.y));
        let added_rows = (min_y..=max_y).filter(|y| {
            // println!("Consider row {y} - empty? {}", empty_rows.contains(y));
            empty_rows.contains(y)
        }).count();
        let added_cols = (min_x..=max_x).filter(|x| empty_cols.contains(x)).count();
        // println!("{:?} to {:?} has basic {basic_dist} plus {added_rows} rows and {added_cols} cols", galaxy1, galaxy2);
        basic_dist + added_cols as i64 + added_rows as i64
    }).sum::<i64>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day11;
    use crate::utils::load_input;

    #[test]
    fn check_day11_case01() {
        full_test(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",  // INPUT STRING
            "374", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day11(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
