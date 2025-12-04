pub mod coord;
pub mod directions;

use std::{collections::HashSet, fmt::Display};

use crate::coord::{Coord2, DELTAS_ORTH_AND_DIAG, DELTAS_ORTH_ONLY};

/// Grids are used for puzzles where the input is a character based two-dimensional matrix.
/// They can be accessed directly in the Vec<Vec<>> format with grid[y][x] but for ease there
/// is also a .get(Coord2) function.
/// Grids are 0-indexed with (0,0) being the top-left-hand corner.

pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub has_border: bool,
}

impl<T: std::convert::From<u32>> Grid<T> {
    pub fn from_digits(input_lines: &[String]) -> Self {
        Grid {
            grid: input_lines
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| Into::<T>::into(c.to_digit(10).unwrap()))
                        .collect::<Vec<T>>()
                })
                .collect::<Vec<Vec<T>>>(),
            has_border: false,
        }
    }
}

impl<T: std::convert::From<u32> + Clone> Grid<T> {
    pub fn from_digits_with_border(input_lines: &[String], border: &T) -> Self {
        let line_len = input_lines[0].len();
        let mut grid = Vec::from([vec![border.clone(); line_len + 2]]);
        for line in input_lines {
            let mut new_row = vec![border.clone()];
            for c in line.chars() {
                new_row.push(Into::<T>::into(c.to_digit(10).unwrap()));
            }
            new_row.push(border.clone());
            grid.push(new_row);
        }
        grid.push(vec![border.clone(); line_len + 2]);
        Grid {
            grid,
            has_border: true,
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
            has_border: false,
        }
    }
}

impl<T: std::convert::Into<char> + Copy> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.grid.len()).for_each(|y| {
            let line = (0..self.grid[0].len())
                .map(|x| {
                    Into::into(
                        *self
                            .peek_in_bounds(&Coord2::from((
                                x.try_into().unwrap(),
                                y.try_into().unwrap(),
                            )))
                            .unwrap(),
                    )
                })
                .collect::<String>();
            let _ = writeln!(f, "{}", line);
        });
        Ok(())
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
        Grid {
            grid,
            has_border: true,
        }
    }
}

impl<T: Clone> Grid<T> {
    /// Initialize a Grid with `len_x` elements in its X co-ord, `len_y` elements in its Y co-ord,
    /// and a starting value of `init_element` in every cell.
    pub fn initialize(len_x: usize, len_y: usize, init_element: T) -> Self {
        let mut line = Vec::new();
        for _ in 0..len_x {
            line.push(init_element.clone());
        }
        let mut grid = Vec::new();
        for _ in 0..len_y {
            grid.push(line.clone());
        }
        Self {
            grid,
            has_border: false,
        }
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
        self.has_border = true;
    }

    pub fn get(&self, coord: &Coord2) -> T {
        self.peek(coord).clone()
    }

    pub fn set_cell(&mut self, coord: &Coord2, value: &T) {
        let y =
            TryInto::<usize>::try_into(coord.y).expect("Can't unwrap the y coordinate as a usize");
        let x =
            TryInto::<usize>::try_into(coord.x).expect("Can't unwrap the x coordinate as a usize");
        self.grid[y][x] = value.clone()
    }

    /// Returns a Vec (in reading order) of the adjacent cells, including diagonals.
    /// Corner and edge cells will return shorter vecs.
    pub fn get_adjacents_inc_diagonals(&self, coord: &Coord2) -> Vec<T> {
        DELTAS_ORTH_AND_DIAG
            .iter()
            .filter_map(|delta| self.peek_in_bounds(&coord.sum(delta)).cloned())
            .collect::<Vec<T>>()
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            has_border: self.has_border,
        }
    }
}

impl<T: Eq> Grid<T> {
    /// Find the coordinates of every cell that has a given value in the grid and
    /// returns them in a HashSet.
    ///
    /// Indifferent to borders.
    pub fn find_elements(&self, element: &T) -> HashSet<Coord2> {
        self.filter_elements(&(|t: &T| t == element))
    }

    /// Find the coordinates of every cell that meets a given predicate in the grid and
    /// returns them in a HashSet.
    ///
    /// Indifferent to borders.
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

    /// Find the coordinates of the expected single instance of a given element in the grid.
    /// Returns None if there are no such or multiple such elements, printing a statement to
    /// stdout in such a case, but in either case this suggests a bug in the usage.
    ///
    /// Indifferent to borders.
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

    /// Takes a starting Coord2 and explores outward, returning a HashSet of every cell with
    /// the same value as the starting coordinate and reachable through orthogonal connections
    /// through cells of the same value. Also returns the perimeter of the region found.
    ///
    /// This function utilises borders: cell coordinates will be 0-indexed without, 1-indexed with
    /// a border (as usual) and exploration will be successfully handled at the edges in cases
    /// without a border.
    pub fn find_region(&self, start_coord: &Coord2) -> (HashSet<Coord2>, usize) {
        let area_val = self.peek(start_coord);
        let mut region = HashSet::from([*start_coord]);
        let mut perimeter: usize = 4;
        let mut explore_from = region.clone();

        while !explore_from.is_empty() {
            let mut next_explore = HashSet::new();
            explore_from.iter().for_each(|node| {
                DELTAS_ORTH_ONLY.iter().for_each(|delta| {
                    let new_coord = node.sum(delta);
                    if self.in_bounds(&new_coord) && self.peek(&new_coord) == area_val {
                        if region.insert(new_coord) {
                            // This coordinate was new
                            perimeter = perimeter.checked_add(3).expect("Perimeter overflowed");
                            next_explore.insert(new_coord);
                        } else {
                            // This coordinate has already been included
                            perimeter = perimeter.checked_sub(1).expect("Perimeter went below 0?");
                        }
                    }
                })
            });
            explore_from = next_explore;
        }

        (region, perimeter)
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

    /// Gets a reference to the contents of the cell at a given coordinate. This coordinate must be in bounds.
    pub fn peek(&self, coord: &Coord2) -> &T {
        let row = TryInto::<usize>::try_into(coord.y).unwrap();
        let column = TryInto::<usize>::try_into(coord.x).unwrap();
        &self.grid[row][column]
    }

    /// Gets a reference to the contents of the cell at a given coordinate if it is in bounds, or None if the cell
    /// is out of bounds.
    pub fn peek_in_bounds(&self, coord: &Coord2) -> Option<&T> {
        if self.in_bounds(coord) {
            Some(self.peek(coord))
        } else {
            None
        }
    }

    /// Returns a single vec over all coordinates in bounds in the grid, ordered by row then column.
    pub fn walk_coords(&self) -> Vec<Coord2> {
        (0..self.grid[0].len())
            .flat_map(|y| {
                (0..self.grid.len())
                    .map(|x| {
                        Coord2::from((
                            TryInto::<i64>::try_into(x).unwrap(),
                            TryInto::<i64>::try_into(y).unwrap(),
                        ))
                    })
                    .collect::<Vec<Coord2>>()
            })
            .collect::<Vec<Coord2>>()
    }

    /// Returns the width of the grid, including the border if there is one.
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    /// Returns the height of the grid, including the border if there is one.
    pub fn height(&self) -> usize {
        self.grid.len()
    }
}
