use std::slice::Iter;
use all_days::Computer;
use self::Direction::*;
use self::CellContent::*;
use std::collections::HashMap;

//* SCC Uses knowledge: map is a tree.
fn main() {
    let mut computer = all_days::define_computer("input/day15.txt");

    let mut robot = Robot {
        x_position: 1,
        y_position: 1,
        min_x_position: 1,
        min_y_position: 1,
        x_out_of_bounds: false,
        y_out_of_bounds: false,
        target_x: None,
        target_y: None,
        map: vec![vec![Unexplored; 3]; 3],
    };
    robot.map[1][1] = Robot;

    let starting_node = Node {
        in_path: None,
        //* SCC Uses knowledge: start is a dead-end with exit N
        other_paths: vec![Path {
            direction: North,
            destination: None,
        }],
        distance: 0
    };

    let mut node_hash: HashMap<(i32, i32), Node> = HashMap::new();
    node_hash.insert((1,1), starting_node);

    explore_map_for_target(&mut robot, &mut computer, &mut node_hash);

    // Clear the map and restart
    node_hash = HashMap::new();
    let starting_node = Node {
        in_path: None,
        //* SCC Uses knowledge: start is a dead-end with exit E
        other_paths: vec![Path {
            direction: East,
            destination: None,
        }],
        distance: 0
    };
    node_hash.insert((robot.target_x.expect(""), robot.target_y.expect("")), starting_node);

    explore_full_map(&mut robot, &mut computer, &mut node_hash);

    println!("Part 1 Solution: {}", node_hash.get(&(1,1)).expect("").distance);
    let distance: usize = node_hash.values().map(|node| node.distance).max().expect("");
    println!("Part 2 Solution: {}", distance);
}

fn explore_map_for_target(robot: &mut Robot,
                          computer: &mut Computer,
                          node_hash: &mut HashMap<(i32, i32), Node>) {
    let mut direction = choose_direction(node_hash.get(&(robot.x_position, robot.y_position)).expect("We've stopped at a non-node"));
    while !robot.at_target() {
        explore_single_path(robot, computer, node_hash, direction.expect(""));
        direction = choose_direction(node_hash.get(&(robot.x_position, robot.y_position)).expect("We've stopped at a non-node"));
    }
}

fn explore_full_map(robot: &mut Robot,
                    computer: &mut Computer,
                    node_hash: &mut HashMap<(i32, i32), Node>) {
    let mut direction = choose_direction(node_hash.get(&(robot.x_position, robot.y_position)).expect("We've stopped at a non-node"));
    while direction.is_some() {
        explore_single_path(robot, computer, node_hash, direction.expect(""));
        direction = choose_direction(node_hash.get(&(robot.x_position, robot.y_position)).expect("We've stopped at a non-node"));
    }

    show_display(robot);
}

fn choose_direction(current_node: &Node) -> Option<Direction> {
    // Pick a direction in Other that hasn't got a distance, otherwise use input
    let unexplored_dirs: Vec<Direction> = current_node.other_paths
        .iter()
        .filter(|&path| path.destination.is_none())
        .map(|&path| path.direction)
        .collect();

    if unexplored_dirs.is_empty() {
        if current_node.in_path.is_none() {
            None
        } else {

            Some(current_node.in_path.expect("Were we back at the start?").direction)
        }
    } else {
        Some(unexplored_dirs[0])
    }
}

fn explore_single_path(robot: &mut Robot,
                       computer: &mut Computer,
                       node_hash: &mut HashMap<(i32, i32), Node>,
                       direction: Direction) {
    let orig_position = (robot.x_position, robot.y_position);

    // Move once in the given direction.
    let output_val = robot.process_command(computer, &direction);
    if output_val == 0 { panic!("Why are we exploring a wall?"); }

    // Fill out all four directions
    for test_dir in Direction::iterator() {
        robot.test_direction(computer, test_dir);
    }

    // If we haven't got a node for our location yet, create one and update the
    // original.
    if node_hash.get(&(robot.x_position, robot.y_position)).is_none() {
        let in_direction = Direction::invert(&direction);

        let old_node = node_hash.remove(&orig_position).expect("where did we come from?");
        let mut updated_old_other_paths: Vec<Path> = old_node.other_paths
            .iter()
            .filter(|&path| path.direction != direction)
            .map(|&path| path)
            .collect::<Vec<Path>>();
        updated_old_other_paths.push(Path {
                direction: direction,
                destination: Some((robot.x_position, robot.y_position)),
            });

        let updated_old_node = Node {
            in_path: old_node.in_path,
            other_paths: updated_old_other_paths,
            distance: old_node.distance,
        };

        let other_paths: Vec<Path> = Direction::iterator()
            .filter(|dir| robot.check_direction(dir) != Wall && **dir != in_direction)
            .map(|dir| direction_to_path(dir))
            .collect();

        let new_node = Node {
            in_path: Some(Path {
                direction: in_direction,
                destination: Some(orig_position),
            }),
            other_paths: other_paths,
            distance: old_node.distance + 1,
        };
        node_hash.insert((robot.x_position, robot.y_position), new_node);
        node_hash.insert(orig_position, updated_old_node);
    }
}

fn show_display(robot: &mut Robot) {
    // print!("{}[2J", 27 as char);
    for line in &mut robot.map {
        let mut display_line: String = String::from("");
        for cell in line {
            let display_char = match cell {
                Unexplored => "#",
                Wall => "#",
                Space => " ",
                Target => "X",
                Robot => "@",
            };
            display_line.push_str(display_char)
        }
        println!("{:?}", display_line);
    }
}

struct Robot {
    x_position: i32,
    y_position: i32,
    min_x_position: i32,
    min_y_position: i32,
    x_out_of_bounds: bool,
    y_out_of_bounds: bool,
    target_x: Option<i32>,
    target_y: Option<i32>,
    map: Vec<Vec<CellContent>>,
}
impl Robot {
    fn x_index(&self) -> usize {
        (self.x_position - self.min_x_position + 1) as usize
    }
    fn y_index(&self) -> usize {
        (self.y_position - self.min_y_position + 1) as usize
    }
    fn check_direction(&self, direction: &Direction) -> CellContent {
        match direction {
            North => self.map[self.y_index() - 1][self.x_index()],
            East => self.map[self.y_index()][self.x_index() + 1],
            South => self.map[self.y_index() + 1][self.x_index()],
            West => self.map[self.y_index()][self.x_index() - 1],
        }
    }
    fn test_direction(&mut self,
                      computer: &mut Computer,
                      direction: &Direction) {
        if self.check_direction(direction) == Unexplored {
            if self.process_command(computer, direction) != 0 {
                self.process_command(computer, &Direction::invert(direction));
            }
        }
    }
    fn move_bot(&mut self, direction: &Direction) {
        // Set existing cell to "Empty"
        let old_x_index = self.x_index();
        let old_y_index = self.y_index();
        self.map[old_y_index][old_x_index] = Space;

        if self.target_x.is_some() {
            if self.target_x.expect("No X!") == self.x_position &&
                self.target_y.expect("No Y!") == self.y_position {
                self.map[old_y_index][old_x_index] = Target;
            }
        }

        // Move robot
        match direction {
            South => self.y_position += 1,
            West => {
                self.x_position -= 1;
                if self.x_position < self.min_x_position {
                    self.x_out_of_bounds = true;
                }
            }
            North => {
                self.y_position -= 1;
                if self.y_position < self.min_y_position {
                    self.y_out_of_bounds = true;
                }
            }
            East => self.x_position += 1,
        }
        // Set new cell to "Robot"
        let new_x_index = self.x_index();
        let new_y_index = self.y_index();
        self.map[new_y_index][new_x_index] = Robot;
    }
    fn update_map(&mut self) {
        // Update outer layer - number of y rows.
        if self.y_out_of_bounds {
            let new_row: Vec<CellContent> = vec![Unexplored; self.map[0].len()];
            self.map.insert(0, new_row);
            self.y_out_of_bounds = false;
            self.min_y_position = self.y_position;
        } else if self.y_index() >= (self.map.len() - 1) {
            let new_row: Vec<CellContent> = vec![Unexplored; self.map[0].len()];
            self.map.push(new_row);
        }
        // Update inner layer - number of x columns
        if self.x_out_of_bounds {
            for row in &mut self.map {
                row.insert(0, Unexplored);
                self.min_x_position = self.x_position;
            }
            self.x_out_of_bounds = false;
        } else if self.x_index() >= (self.map[0].len() - 1) {
            for row in &mut self.map {
                row.push(Unexplored);
            }
        }
    }
    fn process_command(&mut self,
                       mut computer: &mut Computer,
                       direction: &Direction) -> i64 {
        let mut inputs = match direction {
            North => vec![1],
            South => vec![2],
            West => vec![3],
            East => vec![4],
        };

        // Run computer, get output
        let mut output_vec = all_days::run_computer(&mut computer, &mut inputs);
        match output_vec.len() {
            1 => (),
            _ => panic!("Unexpected length of output!"),
        }

        // Act on output
        let output_val = output_vec.pop().expect("Just checked there was an output");
        match output_val {
            0 => self.fill_wall(direction),
            1 => {
                self.move_bot(direction);
                self.update_map();
            }
            2 => {
                self.move_bot(direction);
                self.target_x = Some(self.x_position);
                self.target_y = Some(self.y_position);
                self.update_map();
            }
            _ => panic!("Unknown output"),
        }
        output_val
    }

    fn fill_wall(&mut self, direction: &Direction) {
        // Find cell to fill in
        let (x_index, y_index) = match direction {
            East => (self.x_index() + 1, self.y_index()),
            West => (self.x_index() - 1, self.y_index()),
            North => (self.x_index(), self.y_index() - 1),
            South => (self.x_index(), self.y_index() + 1),
        };

        // Set cell to wall
        self.map[y_index][x_index] = Wall;
    }
    fn at_target(&self) -> bool {
        self.target_x.is_some() &&
            self.target_x.expect("") == self.x_position &&
            self.target_y.expect("") == self.y_position
    }
}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }
    pub fn invert(direction: &Direction) -> Direction {
        match direction {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellContent {
    Unexplored,
    Robot,
    Wall,
    Space,
    Target
}

struct Node {
    in_path: Option<Path>,
    other_paths: Vec<Path>,
    distance: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Path {
    direction: Direction,
    destination: Option<(i32, i32)>,
}

fn direction_to_path(direction: &Direction) -> Path {
    Path {
        direction: *direction,
        destination: None,
    }
}
