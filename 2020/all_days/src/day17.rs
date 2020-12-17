// Potential improvements:
// 1: commonise somewhat with Day 11
// 2: Get the iterator working
// 3: do the multiple dimensions properly instead of copy-paste-extend
// 4: maybe allow negative indices, or start from a sensible location, instead of weird index juggling.

pub fn day17(input_lines: &[String]) -> (u64, u64) {
    let mut grid = Grid::new(input_lines);
    // grid.nth(6);
    (0..6).for_each(|_| {
        grid.update();
    });

    let mut grid4 = Grid4::new(input_lines);
    // grid.nth(6);
    (0..6).for_each(|_| {
        grid4.update();
    });

    (grid.count_active(), grid4.count_active())
}

fn get_delta_range(index: usize, max: usize) -> std::ops::Range<isize> {
    let lower_bound = match index {
        0 => 1,
        1 => 0,
        _ => -1,
    };
    let upper_bound = match index {
        last if last == max + 1 => 0,
        last if last == max => 1,
        _ => 2,
    };
    lower_bound..upper_bound
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Vec<bool>>>, // z,y,x
}
impl Grid {
    fn new(input: &[String]) -> Grid {
        Grid {
            grid: vec![input
                .iter()
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>()],
        }
    }
    fn count_active(self) -> u64 {
        (0..self.grid.len())
            .map(|z| {
                (0..self.grid[0].len())
                    .map(|y| {
                        (0..self.grid[0][0].len())
                            .filter(|&x| self.grid[z][y][x])
                            .count() as u64
                    })
                    .sum::<u64>()
            })
            .sum::<u64>()
    }
    fn determine_new_cell(&self, x: usize, y: usize, z: usize) -> bool {
        let current_x_max = self.grid[0][0].len();
        let current_y_max = self.grid[0].len();
        let current_z_max = self.grid.len();
        let x_delta = get_delta_range(x, current_x_max);
        let y_delta = get_delta_range(y, current_y_max);
        let z_delta = get_delta_range(z, current_z_max);

        let adjacent_count = z_delta
            .map(|dz| {
                y_delta
                    .clone()
                    .map(|dy| {
                        x_delta
                            .clone()
                            .filter(|&dx| {
                                ((dx != 0) || (dy != 0) || (dz != 0))
                                    && self.grid[(z as isize - 1 + dz) as usize]
                                        [(y as isize - 1 + dy) as usize]
                                        [(x as isize - 1 + dx) as usize]
                            })
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        let current_spot = x * y * z != 0
            && x <= current_x_max
            && y <= current_y_max
            && z <= current_z_max
            && self.grid[z - 1][y - 1][x - 1];
        adjacent_count == 3 || (adjacent_count == 2 && current_spot)
    }
    fn update(&mut self) {
        let x_range = self.grid[0][0].len();
        let y_range = self.grid[0].len();
        let z_range = self.grid.len();

        let new_grid = (0..z_range + 2)
            .map(|z| {
                (0..y_range + 2)
                    .map(|y| {
                        (0..x_range + 2)
                            .map(|x| self.determine_new_cell(x, y, z))
                            .collect::<Vec<bool>>()
                    })
                    .collect::<Vec<Vec<bool>>>()
            })
            .collect::<Vec<Vec<Vec<bool>>>>();
        // optionally trim off excess?
        self.grid = new_grid;
    }
}
// This iterator doesn't work because I need to return...something, but I'm not clear what.
// Apparently it can't be itself due to lifetime issues, or something.
// impl Iterator for Grid {
//     type Item = Grid;

//     fn next(&mut self) -> Option<Self::Item> {
//         let x_range = self.grid[0][0].len();
//         let y_range = self.grid[0].len();
//         let z_range = self.grid.len();

//         let new_grid = (0..z_range + 2)
//             .map(|z| {
//                 (0..y_range + 2)
//                     .map(|y| {
//                         (0..x_range + 2)
//                             .map(|x| self.determine_new_cell(x, y, z))
//                             .collect::<Vec<bool>>()
//                     })
//                     .collect::<Vec<Vec<bool>>>()
//             })
//             .collect::<Vec<Vec<Vec<bool>>>>();
//         // optionally trim off excess?
//         self.grid = new_grid;
//         None
//     }
// }

#[derive(Clone)]
struct Grid4 {
    grid: Vec<Vec<Vec<Vec<bool>>>>, // w,z,y,x
}
impl Grid4 {
    fn new(input: &[String]) -> Grid4 {
        Grid4 {
            grid: vec![vec![input
                .iter()
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>()]],
        }
    }
    fn count_active(self) -> u64 {
        (0..self.grid.len())
            .map(|w| {
                (0..self.grid[0].len())
                    .map(|z| {
                        (0..self.grid[0][0].len())
                            .map(|y| {
                                (0..self.grid[0][0][0].len())
                                    .filter(|&x| self.grid[w][z][y][x])
                                    .count() as u64
                            })
                            .sum::<u64>()
                    })
                    .sum::<u64>()
            })
            .sum::<u64>()
    }
    fn determine_new_cell(&self, x: usize, y: usize, z: usize, w: usize) -> bool {
        let current_x_max = self.grid[0][0][0].len();
        let current_y_max = self.grid[0][0].len();
        let current_z_max = self.grid[0].len();
        let current_w_max = self.grid.len();
        let x_delta = get_delta_range(x, current_x_max);
        let y_delta = get_delta_range(y, current_y_max);
        let z_delta = get_delta_range(z, current_z_max);
        let w_delta = get_delta_range(w, current_w_max);

        let adjacent_count = w_delta
            .map(|dw| {
                z_delta
                    .clone()
                    .map(|dz| {
                        y_delta
                            .clone()
                            .map(|dy| {
                                x_delta
                                    .clone()
                                    .filter(|&dx| {
                                        ((dx != 0) || (dy != 0) || (dz != 0) || (dw != 0))
                                            && self.grid[(w as isize - 1 + dw) as usize]
                                                [(z as isize - 1 + dz) as usize]
                                                [(y as isize - 1 + dy) as usize]
                                                [(x as isize - 1 + dx) as usize]
                                    })
                                    .count()
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        let current_spot = x * y * z * w != 0
            && x <= current_x_max
            && y <= current_y_max
            && z <= current_z_max
            && w <= current_w_max
            && self.grid[w - 1][z - 1][y - 1][x - 1];
        adjacent_count == 3 || (adjacent_count == 2 && current_spot)
    }
    fn update(&mut self) {
        let x_range = self.grid[0][0][0].len();
        let y_range = self.grid[0][0].len();
        let z_range = self.grid[0].len();
        let w_range = self.grid.len();

        let new_grid = (0..w_range + 2)
            .map(|w| {
                (0..z_range + 2)
                    .map(|z| {
                        (0..y_range + 2)
                            .map(|y| {
                                (0..x_range + 2)
                                    .map(|x| self.determine_new_cell(x, y, z, w))
                                    .collect::<Vec<bool>>()
                            })
                            .collect::<Vec<Vec<bool>>>()
                    })
                    .collect::<Vec<Vec<Vec<bool>>>>()
            })
            .collect::<Vec<Vec<Vec<Vec<bool>>>>>();

        // optionally trim off excess?
        self.grid = new_grid;
    }
}
