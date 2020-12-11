pub fn day11(_input_lines: &[String]) -> (u64, u64) {
    // Read grid
    // Iteratively run change until it stabilises
    // Report count

    (0, 0)
}

#[derive(Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Filled,
}

struct Grid {
    grid: Vec<Vec<Seat>>,
}
impl Grid {
    fn update_grid(self) -> (Grid, bool) {
        let new_grid = Grid {
            grid: (0..self.grid.len())
                .map(|y| self.update_row(y))
                .collect::<Vec<Vec<Seat>>>(),
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
            (Seat::Filled, crowded) if crowded >= 4 => Seat::Empty,
            (Seat::Filled, _) => Seat::Filled,
        }
    }

    fn count_surrounding_filled(&self, x: usize, y: usize) -> usize {
        let x_range = Grid::get_range(x, self.grid[0].len() - 1);
        let y_range = Grid::get_range(y, self.grid.len() - 1);
        let own_space_value = if self.grid[y][x] == Seat::Filled {
            1
        } else {
            0
        };
        self.count_subrange_filled(x_range, y_range) - own_space_value
    }

    fn get_range(index: usize, max: usize) -> std::ops::Range<usize> {
        match index {
            0 => (0..2),
            last if last == max => (max - 1..max + 1),
            _ => (index - 1..index + 2),
        }
    }

    fn count_total_filled(&self) -> usize {
        self.count_subrange_filled(0..self.grid[0].len(), 0..self.grid.len())
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
    use super::{Grid, Seat};

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
        };
        assert_eq!(test_grid.count_total_filled(), 6);
        let (new_grid, changed) = test_grid.update_grid();
        assert_eq!(changed, true);
        assert_eq!(new_grid.count_total_filled(), 5);
    }
}
