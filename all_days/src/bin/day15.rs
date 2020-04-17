use std::io;

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Clone)]
enum CellContent {
    Unexplored,
    Robot,
    Wall,
    Space,
    Target
}

fn main() {
    let mut computer = all_days::define_computer("input/day15.txt");

    let mut map: Vec<Vec<CellContent>> = vec![vec![CellContent::Unexplored; 3]; 3];
    map[1][1] = CellContent::Robot;
    let mut robot = Robot {
        x_position: 1,
        y_position: 1,
        min_x_position: 1,
        min_y_position: 1,
        x_out_of_bounds: false,
        y_out_of_bounds: false,
    };

    loop {
        // Get input (and save copy!)
        let input: Direction = get_input();
        let mut inputs: Vec<i64> = match input {
            Direction::North => vec![1],
            Direction::South => vec![2],
            Direction::West => vec![3],
            Direction::East => vec![4],
        };

        // Run computer, get output
        let mut output_vec = all_days::run_computer(&mut computer, &mut inputs);
        match output_vec.len() {
            0 => break,
            1 => (),
            _ => panic!("Unexpected length of output!"),
        }

        // Act on output
        match output_vec.pop().expect("Just checked there was an output") {
            0 => fill_wall(&mut robot, &mut map, input),
            1 => {
                robot.move_bot(&mut map, input);
                update_map(&mut robot, &mut map);
            }
            2 => {
                println!("Hooray, we found it!");
                break;
            }
            _ => panic!("Unknown output"),
        }

        // Print map
        show_display(&robot, &map);
    }
}

fn get_input() -> Direction {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(2) => return Direction::South,
            Ok(6) => return Direction::East,
            Ok(8) => return Direction::North,
            Ok(4) => return Direction::West,
            Ok(_) => {println!("Bad input, try again"); continue},
            Err(msg) => {println!("{}", msg); continue},
        }
    }
}

fn fill_wall(robot: &mut Robot, map: &mut Vec<Vec<CellContent>>, direction: Direction) {
    // Find cell to fill in
    let (x_index, y_index) = match direction {
        Direction::East => (robot.x_index() + 1, robot.y_index()),
        Direction::West => (robot.x_index() - 1, robot.y_index()),
        Direction::North => (robot.x_index(), robot.y_index() - 1),
        Direction::South => (robot.x_index(), robot.y_index() + 1),
    };

    // Set cell to wall
    map[y_index][x_index] = CellContent::Wall;
}

// Want to tweak for Day 15 to extend 1 out from robot?
// May need to tweak move_bot() for this too
fn update_map(robot: &mut Robot, map: &mut Vec<Vec<CellContent>>) {
    // Update outer layer - number of y rows.
    if robot.y_out_of_bounds {
        let new_row: Vec<CellContent> = vec![CellContent::Unexplored; map[0].len()];
        map.insert(0, new_row);
        robot.y_out_of_bounds = false;
        robot.min_y_position = robot.y_position;
    } else if robot.y_index() >= (map.len() - 1) {
        let new_row: Vec<CellContent> = vec![CellContent::Unexplored; map[0].len()];
        map.push(new_row);
    }
    // Update inner layer - number of x columns
    if robot.x_out_of_bounds {
        for row in map {
            row.insert(0, CellContent::Unexplored);
            robot.min_x_position = robot.x_position;
        }
        robot.x_out_of_bounds = false;
    } else if robot.x_index() >= (map[0].len() - 1) {
        for row in map {
            row.push(CellContent::Unexplored);
        }
    }
}

fn show_display(robot: &Robot, display: &[Vec<CellContent>]) {
    print!("{}[2J", 27 as char);
    println!("{:?}", robot);
    for line in display {
        let mut display_line: String = String::from("");
        for cell in line {
            let display_char = match cell {
                CellContent::Unexplored => " ",
                CellContent::Wall => "#",
                CellContent::Space => ".",
                CellContent::Target => "!",
                CellContent::Robot => "@",
            };
            display_line.push_str(display_char)
        }
        println!("{:?}", display_line);
    }
}

#[derive(Debug)]
struct Robot {
    x_position: i32,
    y_position: i32,
    min_x_position: i32,
    min_y_position: i32,
    x_out_of_bounds: bool,
    y_out_of_bounds: bool,
}
impl Robot {
    fn x_index(&self) -> usize {
        (self.x_position - self.min_x_position + 1) as usize
    }
    fn y_index(&self) -> usize {
        (self.y_position - self.min_y_position + 1) as usize
    }
    fn move_bot(&mut self, map: &mut Vec<Vec<CellContent>>, direction: Direction) {
        // Set existing cell to "Empty"
        map[self.y_index()][self.x_index()] = CellContent::Space;
        // Move robot
        match direction {
            Direction::South => self.y_position += 1,
            Direction::West => {
                self.x_position -= 1;
                if self.x_position < self.min_x_position {
                    self.x_out_of_bounds = true;
                }
            }
            Direction::North => {
                self.y_position -= 1;
                if self.y_position < self.min_y_position {
                    self.y_out_of_bounds = true;
                }
            }
            Direction::East => self.x_position += 1,
        }
        // Set new cell to "Robot"
        map[self.y_index()][self.x_index()] = CellContent::Robot;
    }
}
