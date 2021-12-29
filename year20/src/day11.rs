pub fn day11(input_lines: &[String]) -> (u64, u64) {
    let mut changed = true;
    let mut part1_grid = read_grid(input_lines, 1);
    while changed {
        let (new_grid, new_changed) = part1_grid.update_grid();
        part1_grid = new_grid;
        changed = new_changed;
    }

    changed = true;
    let mut part2_grid = read_grid(input_lines, 2);
    while changed {
        let (new_grid, new_changed) = part2_grid.update_grid();
        part2_grid = new_grid;
        changed = new_changed;
    }

    (
        part1_grid.count_total_filled() as u64,
        part2_grid.count_total_filled() as u64,
    )
}

fn read_grid(input_lines: &[String], part: usize) -> Grid {
    let read_grid = input_lines
        .iter()
        .map(|line| line.chars().map(char_to_seat).collect::<Vec<Seat>>())
        .collect::<Vec<Vec<Seat>>>();
    Grid {
        grid: read_grid,
        puzzle_part: part,
    }
}

fn char_to_seat(c: char) -> Seat {
    match c {
        '.' => Seat::Floor,
        'L' => Seat::Empty,
        '#' => Seat::Filled,
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Filled,
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Seat>>,
    puzzle_part: usize,
}
impl Grid {
    fn update_grid(self) -> (Grid, bool) {
        let new_grid = Grid {
            grid: (0..self.grid.len())
                .map(|y| self.update_row(y))
                .collect::<Vec<Vec<Seat>>>(),
            puzzle_part: self.puzzle_part,
        };
        let change = (0..self.grid.len())
            .any(|y| (0..self.grid[y].len()).any(|x| self.grid[y][x] != new_grid.grid[y][x]));
        (new_grid, change)
    }

    fn update_row(&self, y: usize) -> Vec<Seat> {
        (0..self.grid[y].len())
            .map(|x| self.calc_update_cell(x, y))
            .collect::<Vec<Seat>>()
    }

    fn calc_update_cell(&self, x: usize, y: usize) -> Seat {
        assert!(x <= self.grid[0].len());
        assert!(y <= self.grid.len());
        let current_seat = &self.grid[y][x];
        match (current_seat, self.count_surrounding_filled(x, y)) {
            (Seat::Floor, _) => Seat::Floor,
            (Seat::Empty, 0) => Seat::Filled,
            (Seat::Empty, _) => Seat::Empty,
            (Seat::Filled, crowded) if crowded >= (self.puzzle_part + 3) => Seat::Empty,
            (Seat::Filled, _) => Seat::Filled,
        }
    }

    fn get_delta_range(index: usize, max: usize) -> std::ops::Range<isize> {
        match index {
            0 => 0..2,
            last if last == max - 1 => -1..1,
            _ => -1..2,
        }
    }

    fn filled_seat_seen(&self, coords: (usize, usize), (x_delta, y_delta): (isize, isize)) -> bool {
        let mut i = 1;
        // println!("Coords: ({}, {})", coords.0, coords.1);
        if x_delta == 0 && y_delta == 0 {
            return false;
        }
        loop {
            let x = coords.0 as isize + i * x_delta;
            let y = coords.1 as isize + i * y_delta;
            // println!("Look at ({}, {})", x, y);

            if x < 0 || x as usize >= self.grid[0].len() {
                return false;
            }
            if y < 0 || y as usize >= self.grid.len() {
                return false;
            }

            if self.grid[y as usize][x as usize] != Seat::Floor {
                // println!("Found a seat: {:?}", self.grid[y as usize][x as usize]);
                return self.grid[y as usize][x as usize] == Seat::Filled;
            } else {
                i += 1;
                // println!("increase i to {}", i);
            }
        }
    }

    // Part 2 change => need to look further!
    fn count_surrounding_filled(&self, x: usize, y: usize) -> usize {
        if self.puzzle_part == 1 {
            let x_range = Grid::get_range(x, self.grid[0].len());
            let y_range = Grid::get_range(y, self.grid.len());
            let own_space_value = if self.grid[y][x] == Seat::Filled {
                1
            } else {
                0
            };
            self.count_subrange_filled(x_range, y_range) - own_space_value
        } else {
            assert_eq!(self.puzzle_part, 2);
            let x_range = Grid::get_delta_range(x, self.grid[0].len());
            let y_range = Grid::get_delta_range(y, self.grid.len());
            self.count_full_sight_filled((x, y), x_range, y_range)
        }
    }

    fn get_range(index: usize, max: usize) -> std::ops::Range<usize> {
        match index {
            0 => (0..2),
            last if last == max - 1 => (max - 2..max),
            _ => (index - 1..index + 2),
        }
    }

    fn count_total_filled(&self) -> usize {
        self.count_subrange_filled(0..self.grid[0].len(), 0..self.grid.len())
    }

    fn count_full_sight_filled(
        &self,
        coords: (usize, usize),
        x_range: std::ops::Range<isize>,
        y_range: std::ops::Range<isize>,
    ) -> usize {
        y_range
            .map(|y| {
                x_range
                    .clone()
                    .filter(|&x| self.filled_seat_seen(coords, (x, y)))
                    .count()
            })
            .sum::<usize>()
    }

    fn count_subrange_filled(
        &self,
        x_range: std::ops::Range<usize>,
        y_range: std::ops::Range<usize>,
    ) -> usize {
        y_range
            .map(|y| {
                x_range
                    .clone()
                    .filter(|&x| self.grid[y][x] == Seat::Filled)
                    .count()
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::{read_grid, Grid, Seat};

    #[test]
    fn check_grid_count() {
        // #L.
        // L##
        // ..#
        let test_grid = Grid {
            grid: vec![
                vec![Seat::Filled, Seat::Empty, Seat::Floor],
                vec![Seat::Empty, Seat::Filled, Seat::Filled],
                vec![Seat::Floor, Seat::Floor, Seat::Filled],
            ],
            puzzle_part: 1,
        };
        assert_eq!(test_grid.count_surrounding_filled(0, 0), 1);
        assert_eq!(test_grid.count_surrounding_filled(1, 0), 3);
        assert_eq!(test_grid.count_surrounding_filled(2, 0), 2);
        assert_eq!(test_grid.count_surrounding_filled(0, 1), 2);
        assert_eq!(test_grid.count_surrounding_filled(1, 1), 3);
        assert_eq!(test_grid.count_surrounding_filled(2, 1), 2);
        assert_eq!(test_grid.count_surrounding_filled(0, 2), 1);
        assert_eq!(test_grid.count_surrounding_filled(1, 2), 3);
        assert_eq!(test_grid.count_surrounding_filled(2, 2), 2);
        assert_eq!(test_grid.count_total_filled(), 4);
    }

    #[test]
    fn check_grid_update() {
        // #L.#
        // LL##
        // L.##
        let test_grid = Grid {
            grid: vec![
                vec![Seat::Filled, Seat::Empty, Seat::Floor, Seat::Filled],
                vec![Seat::Empty, Seat::Empty, Seat::Filled, Seat::Filled],
                vec![Seat::Empty, Seat::Floor, Seat::Filled, Seat::Filled],
            ],
            puzzle_part: 1,
        };
        assert_eq!(test_grid.count_total_filled(), 6);
        let (new_grid, changed) = test_grid.update_grid();
        assert_eq!(changed, true);
        assert_eq!(new_grid.count_total_filled(), 5);
    }

    #[test]
    fn check_directions() {
        let test_input = ".##.##.
#.###.#
##...##
...L...
##...##
#.#.#.#
.##.##."
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let grid = read_grid(&test_input, 2);
        assert!(grid.count_full_sight_filled((3, 3), -1..2, -1..2) == 1);
    }
}
