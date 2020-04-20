enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Clone)]
enum CellColour {
    Unpainted,
    Black,
    White,
}

fn main() {
    let mut computer = all_days::define_computer("input/day11.txt");

    let mut map: Vec<Vec<CellColour>> = vec![vec![CellColour::White]];
    let mut robot = Robot {
        x_position: 0,
        y_position: 0,
        min_x_position: 0,
        min_y_position: 0,
        x_out_of_bounds: false,
        y_out_of_bounds: false,
        direction: Direction::North,
    };

    let mut painted_cells = 0;

    loop {
        // Get input
        let (unpainted, input) = match map[robot.y_index()][robot.x_index()] {
            CellColour::Unpainted => (true, 0),
            CellColour::Black => (false, 0),
            CellColour::White => (false, 1),
        };
        let mut inputs = vec![input];
        // Run computer, get output.
        let output_vec = computer.run_computer(&mut inputs);
        // If output is empty, terminate
        match output_vec.len() {
            0 => break,
            2 => (),
            _ => panic!("Got an output vec of length {}", output_vec.len()),
        }
        // Update painting count
        if unpainted {
            painted_cells += 1;
        }
        // Paint cell
        map[robot.y_index()][robot.x_index()] = match output_vec[0] {
            0 => CellColour::Black,
            1 => CellColour::White,
            _ => panic!("Unrecognised colour output {}", output_vec[0]),
        };
        // Rotate robot
        robot.rotate(output_vec[1]);
        // Move robot
        robot.move_bot();
        // Update map
        update_map(&mut robot, &mut map);
    }

    println!("Painted {} cells", painted_cells + 1);
    map.reverse();
    for line in map {
        let mut colour_line: String = String::from("");
        for cell in line {
            let colour_char = match cell {
                CellColour::Unpainted => " ",
                CellColour::Black => " ",
                CellColour::White => "#",
            };
            colour_line.push_str(colour_char)
        }
        println!("{:?}", colour_line);
    }
}

fn update_map(robot: &mut Robot, map: &mut Vec<Vec<CellColour>>) {
    // Update outer layer - number of y rows.
    if robot.y_out_of_bounds {
        let new_row: Vec<CellColour> = vec![CellColour::Unpainted; map[0].len()];
        map.insert(0, new_row);
        robot.y_out_of_bounds = false;
    } else if robot.y_index() >= map.len() {
        let new_row: Vec<CellColour> = vec![CellColour::Unpainted; map[0].len()];
        map.push(new_row);
    }
    // Update inner layer - number of x columns
    if robot.x_out_of_bounds {
        for row in map {
            row.insert(0, CellColour::Unpainted);
        }
        robot.x_out_of_bounds = false;
    } else if robot.x_index() >= map[0].len() {
        for row in map {
            row.push(CellColour::Unpainted);
        }
    }
}

struct Robot {
    x_position: i32,
    y_position: i32,
    min_x_position: i32,
    min_y_position: i32,
    x_out_of_bounds: bool,
    y_out_of_bounds: bool,
    direction: Direction,
}
impl Robot {
    fn x_index(&self) -> usize {
        (self.x_position - self.min_x_position) as usize
    }
    fn y_index(&self) -> usize {
        (self.y_position - self.min_y_position) as usize
    }
    fn rotate(&mut self, turn: i64) {
        if turn == 0 {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            };
        } else if turn == 1 {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
            };
        } else {
            panic!("Unknown turn command {}", turn);
        }
    }
    fn move_bot(&mut self) {
        match self.direction {
            Direction::North => self.y_position += 1,
            Direction::West => {
                self.x_position -= 1;
                if self.x_position < self.min_x_position {
                    self.x_out_of_bounds = true;
                    self.min_x_position = self.x_position;
                }
            }
            Direction::South => {
                self.y_position -= 1;
                if self.y_position < self.min_y_position {
                    self.y_out_of_bounds = true;
                    self.min_y_position = self.y_position;
                }
            }
            Direction::East => self.x_position += 1,
        }
    }
}
