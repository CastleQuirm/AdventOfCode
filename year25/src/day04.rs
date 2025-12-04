// Potential improvements:
//

// use crate::grid::Grid;
use grid::Grid;

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let layout = Grid::<PaperSpace>::from_input(&input_lines[0]);
    let (answer1, mut new_grid) = iteratatively_remove(layout);
    let mut answer2 = 0;
    let mut newly_removed = answer1;
    while newly_removed > 0 {
        answer2 += newly_removed;
        (newly_removed, new_grid) = iteratatively_remove(new_grid);
    }

    (format!("{}", answer1), format!("{}", answer2))
}

fn iteratatively_remove(layout: Grid<PaperSpace>) -> (usize, Grid<PaperSpace>) {
    let mut updated_layout = layout.clone();
    let removed = layout
        .walk_coords()
        .iter()
        .filter(|coord| {
            let adjacent_count = layout
                .get_adjacents_inc_diagonals(coord)
                .iter()
                .filter(|adj| adj == &&PaperSpace::Paper)
                .count();
            let accessible =
                adjacent_count < 4 && layout.peek_in_bounds(coord).unwrap() == &PaperSpace::Paper;
            if accessible {
                updated_layout.set_cell(coord, &PaperSpace::Space);
            }
            accessible
        })
        .count();
    (removed, updated_layout)
}

#[derive(Clone, Copy, PartialEq)]
enum PaperSpace {
    Space,
    Paper,
    // Accessible,
}

impl From<char> for PaperSpace {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            '.' => Self::Space,
            _ => panic!(),
        }
    }
}

// impl Into<char> for PaperSpace {
//     fn into(self) -> char {
//         match self {
//             PaperSpace::Space => '.',
//             PaperSpace::Paper => '@',
//             PaperSpace::Accessible => 'x',
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", // INPUT STRING
            "13", // PART 1 RESULT
            "43", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
