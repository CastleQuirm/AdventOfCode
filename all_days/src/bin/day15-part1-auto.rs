use std::slice::Iter;
use all_days::Computer;
use self::Direction::*;
use self::CellContent::*;
use std::collections::HashMap;

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
        other_paths: vec![Path {
            direction: North,
            length: None,
            destination: None,
        }],
    };

    let mut node_hash: HashMap<(i32, i32), Node> = HashMap::new();
    node_hash.insert((1,1), starting_node);

    explore_map(&mut robot, &mut computer, &mut node_hash);

    println!("Part 1 solution: {}",
             calculate_part1_distance((1,1),
                                      (robot.target_x.expect(""),
                                       robot.target_y.expect("")), node_hash));
}

fn calculate_part1_distance(start_position: (i32, i32),
                            target_position: (i32, i32),
                            node_hash: HashMap<(i32, i32), Node>) -> usize {
    let mut current_position = target_position;
    let mut total_distance: usize = 0;
    while current_position != start_position {
        let node_ptr: &Node = node_hash.get(&current_position).expect("");
        total_distance += node_ptr.in_path.expect("").length.expect("");
        current_position = node_ptr.in_path.expect("").destination.expect("");
    }
    total_distance
}

fn explore_map(robot: &mut Robot,
               computer: &mut Computer,
               node_hash: &mut HashMap<(i32, i32), Node>) {
    loop {
        let direction = choose_direction(node_hash.get(&(robot.x_position, robot.y_position)).expect("We've stopped at a non-node"));
        if direction.is_none() { break; }
        let _distance = explore_single_path(robot, computer, node_hash, direction.expect(""));
    }
    show_display(robot);
}

fn choose_direction(current_node: &Node) -> Option<Direction> {
    // Pick a direction in Other that hasn't got a distance, otherwise use input
    let unexplored_dirs: Vec<Direction> = current_node.other_paths
        .iter()
        .filter(|&path| path.length.is_none())
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
                       orig_direction: Direction) -> usize {
    let mut entry_direction = orig_direction;
    let orig_position = (robot.x_position, robot.y_position);

    // Move once in the given direction.
    // - if the move returns 0, exit the function with distance 0
    let output_val = robot.process_command(computer, &entry_direction);
    if output_val == 0 { return 0; }
    let mut distance_travelled = 1;

    // Loop:
    // - Fill out all four directions
    // - Count how many directions are Open; if != 2, break
    // - Go the way we didn't come in and increment distance by 1
    loop {
        let mut closed_paths = 0;
        let mut new_direction: Direction = entry_direction;
        for test_dir in Direction::iterator() {
            let cell = robot.test_direction(computer, test_dir);
            if cell == Wall {
                closed_paths += 1;
            } else if entry_direction != Direction::invert(test_dir) {
                new_direction = *test_dir;
            }

        }

        if closed_paths != 2 {
            break;
        }

        entry_direction = new_direction;
        robot.process_command(computer, &entry_direction);
        distance_travelled += 1;
    }

    // If we haven't got a node for our location yet, create one and update the
    // original.
    if node_hash.get(&(robot.x_position, robot.y_position)).is_none() {
        let in_direction = Direction::invert(&entry_direction);
        let other_paths: Vec<Path> = Direction::iterator()
            .filter(|dir| robot.check_direction(dir) != Wall && **dir != in_direction)
            .map(|dir| direction_to_path(dir))
            .collect();

        let new_node = Node {
            in_path: Some(Path {
                direction: in_direction,
                length: Some(distance_travelled),
                destination: Some(orig_position),
            }),
            other_paths: other_paths,
        };
        node_hash.insert((robot.x_position, robot.y_position), new_node);

        let old_node = node_hash.remove(&orig_position).expect("where did we come from?");
        let mut updated_old_other_paths: Vec<Path> = old_node.other_paths
            .iter()
            .filter(|&path| path.direction != orig_direction)
            .map(|&path| path)
            .collect::<Vec<Path>>();
        updated_old_other_paths.push(Path {
                direction: orig_direction,
                length: Some(distance_travelled),
                destination: Some((robot.x_position, robot.y_position)),
            });

        let updated_old_node = Node {
            in_path: old_node.in_path,
            other_paths: updated_old_other_paths,
        };
        node_hash.insert(orig_position, updated_old_node);
    }

    distance_travelled
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
                      direction: &Direction) -> CellContent {
        if self.check_direction(direction) == Unexplored {
            if self.process_command(computer, direction) != 0 {
                self.process_command(computer, &Direction::invert(direction));
            }
        }
        self.check_direction(direction)
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
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Path {
    direction: Direction,
    length: Option<usize>,
    destination: Option<(i32, i32)>,
}

fn direction_to_path(direction: &Direction) -> Path {
    Path {
        direction: *direction,
        length: None,
        destination: None,
    }
}
