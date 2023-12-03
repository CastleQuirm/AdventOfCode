use crate::coord::Coord2;

/// Grids are used for puzzles where the input is a character based two-dimensional matrix.
/// They can be accessed directly in the Vec<Vec<>> format with grid[y][x] but for ease there
/// is also a .get(Coord2) function.
/// Grids are 0-indexed with (0,0) being the top-right-hand corner.
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T: std::convert::From<char> + Copy> Grid<T> {
    pub fn from_input(input_lines: &[String]) -> Self {
        Grid {
            grid: input_lines
                .iter()
                .map(|line| line.chars().map(|c| c.into()).collect::<Vec<T>>())
                .collect::<Vec<Vec<T>>>(),
        }
    }

    pub fn add_border(&mut self, border_element: T) {
        let line_len = self.grid[0].len();
        for row in &mut self.grid {
            row.insert(0, border_element);
            row.push(border_element);
        }
        self.grid.insert(0, vec![border_element; line_len + 2]);
        self.grid.push(vec![border_element; line_len + 2]);
    }
}

impl<T: Copy> Grid<T> {
    pub fn get(&self, coord: Coord2) -> T {
        let row = TryInto::<usize>::try_into(coord.y).unwrap();
        let column = TryInto::<usize>::try_into(coord.x).unwrap();
        self.grid[row][column]
    }
}
