use std::collections::HashSet;

use crate::coord::Coord2;

/// Grids are used for puzzles where the input is a character based two-dimensional matrix.
/// They can be accessed directly in the Vec<Vec<>> format with grid[y][x] but for ease there
/// is also a .get(Coord2) function.
/// Grids are 0-indexed with (0,0) being the top-left-hand corner.

pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T: std::convert::From<u32>> Grid<T> {
    pub fn _from_digits(input_lines: &[String]) -> Self {
        Grid {
            grid: input_lines
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| Into::<T>::into(c.to_digit(10).unwrap()))
                        .collect::<Vec<T>>()
                })
                .collect::<Vec<Vec<T>>>(),
        }
    }
}

impl<T: std::convert::From<char>> Grid<T> {
    pub fn from_input(input_lines: &[String]) -> Self {
        Grid {
            grid: input_lines
                .iter()
                .map(|line| line.chars().map(|c| c.into()).collect::<Vec<T>>())
                .collect::<Vec<Vec<T>>>(),
        }
    }
}

impl<T: std::convert::From<char> + Clone> Grid<T> {
    pub fn from_input_with_border(input_lines: &[String], border: &T) -> Self {
        let line_len = input_lines[0].len();
        let mut grid = Vec::from([vec![border.clone(); line_len + 2]]);
        for line in input_lines {
            let mut new_row = vec![border.clone()];
            for c in line.chars() {
                new_row.push(c.into());
            }
            new_row.push(border.clone());
            grid.push(new_row);
        }
        grid.push(vec![border.clone(); line_len + 2]);
        Grid { grid }
    }
}

impl<T: Clone> Grid<T> {
    /// Initialize a Grid with `len_x` elements in its X co-ord, `len_y` elements in its Y co-ord,
    /// and a starting value of `init_element` in every cell.
    pub fn _initialize(len_x: usize, len_y: usize, init_element: T) -> Self {
        let mut line = Vec::new();
        for _ in 0..len_x {
            line.push(init_element.clone());
        }
        let mut grid = Vec::new();
        for _ in 0..len_y {
            grid.push(line.clone());
        }
        Self { grid }
    }

    // TODO add an initialize_with_border() for single calls and non-mutable versions
    // (plus more efficient code!)

    pub fn add_border(&mut self, border_element: &T) {
        let line_len = self.grid[0].len();
        for row in &mut self.grid {
            row.insert(0, border_element.clone());
            row.push(border_element.clone());
        }
        self.grid
            .insert(0, vec![border_element.clone(); line_len + 2]);
        self.grid.push(vec![border_element.clone(); line_len + 2]);
    }

    pub fn get(&self, coord: &Coord2) -> T {
        let row = TryInto::<usize>::try_into(coord.y).unwrap();
        let column = TryInto::<usize>::try_into(coord.x).unwrap();
        self.grid[row][column].clone()
    }

    pub fn set_cell(&mut self, coord: &Coord2, value: &T) {
        let y =
            TryInto::<usize>::try_into(coord.y).expect("Can't unwrap the y coordinate as a usize");
        let x =
            TryInto::<usize>::try_into(coord.x).expect("Can't unwrap the x coordinate as a usize");
        self.grid[y][x] = value.clone()
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
        }
    }
}

impl<T: Eq> Grid<T> {
    pub fn find_elements(&self, element: &T) -> HashSet<Coord2> {
        self.filter_elements(&(|t: &T| t == element))
    }

    pub fn filter_elements(&self, predicate: &dyn Fn(&T) -> bool) -> HashSet<Coord2> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(row_ix, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(col_ix, entry)| {
                        if predicate(entry) {
                            Some(Coord2 {
                                x: col_ix as i64,
                                y: row_ix as i64,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>()
    }

    pub fn find_single_element(&self, element: &T) -> Option<Coord2> {
        let possible_elements = self.find_elements(element);
        if possible_elements.len() == 1 {
            Some(*possible_elements.iter().next().unwrap())
        } else if possible_elements.is_empty() {
            println!("Found no elements in the grid");
            None
        } else {
            println!("Found more than one element in the grid");
            None
        }
    }
}

impl<T> Grid<T> {
    /// Checks if the given coordinate is within the grid's coordinate space (0-indexed, border included if it is present)
    /// Requires the Grid to have at least one row.
    pub fn in_bounds(&self, coord: &Coord2) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && coord.x < self.grid.len() as i64
            && coord.y < self.grid[0].len() as i64
    }
}
