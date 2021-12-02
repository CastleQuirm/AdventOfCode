// Thoughts
// Want to parse and analyse the map to create a layered hash-map:
// source key -> (dest key -> (distance, Vec<doors crossed>))
// for all source key, dest key pairs (source keys have to include start, dest don't)

// Then we start building paths
// struct Path: (dest key, distance, [collected keys])
// We iteratively build vectors of possible paths of certain numbers of keys
// Initial path0vec is single element: (start, 0, [])

// Then we call a function on the vec, feeding the output in each time as a new
// input.
// Process: for each element in the vec, determine all next steps
// to get a next step: pick a key not in collected keys, check current dest key -> chosen key in hashmap
// if content of doors crossed for that element includes anything not in collected keys, drop it
// otherwise new path element: (chosen key, distance + path len, collected keys + chosen key)
// Once we've collected all the keys, look for the path with the min distance.

use std::slice::Iter;
use self::Direction::*;
use self::CellContent::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let robot = parse_input_map();

    let key_travels = robot.explore();

    let mut possible_routes: Vec<Route> = vec![ Route {
        endpoint: '@',
        distance: 0,
        keys: Vec::new(),
    }];

    for _i in 0..key_travels.len() - 1 {
        possible_routes = extend_routes(&possible_routes, &key_travels);
    }
    assert_eq!(key_travels.len(), 27);

    let part_1_answer = find_min_distance(possible_routes);

    println!("Part 1 Answer: {}", part_1_answer);
    println!("Part 2 Answer: {}", "unknown");
}

fn parse_input_map() -> PathRobot {
    // SCC TO IMPLEMENT
    let input_string = fs::read_to_string("input/day18.txt").expect("Failed to read file");
    let mut map: Vec<Vec<CellContent>> = Vec::new();

    let mut x = 0;
    let mut y = 0;
    let mut robot_x = 0;
    let mut robot_y = 0;

    for line in input_string.lines() {
        let mut line_vec = Vec::new();
        for cell in line.chars() {
            if cell == '#' {
                line_vec.push(Wall);
            } else if cell == '.' {
                line_vec.push(Space);
            } else if cell.is_ascii_lowercase() {
                line_vec.push(Key(cell));
            } else if cell.is_ascii_uppercase() {
                line_vec.push(Door(cell));
            } else if cell == '@' {
                line_vec.push(Space);
                assert_eq!(robot_x, 0);
                assert_eq!(robot_y, 0);
                robot_x = x;
                robot_y = y;
            } else {
                panic!("Unrecognised character in map {}", cell);
            }
            x += 1;
        }
        map.push(line_vec);
        x = 0;
        y += 1;
    }

    // Hack to make the map a tree: adjust the cell to the left and right of the
    // robot.
    // assert!(map[robot_y][robot_x - 1] == Space);
    // assert!(map[robot_y][robot_x + 1] == Space);
    // map[robot_y][robot_x - 1] = Wall;
    // map[robot_y][robot_x + 1] = Wall;

    PathRobot {
        x_position: robot_x,
        y_position: robot_y,
        map: map,
        node_hash: HashMap::new(),
    }
}

fn extend_routes(old_routes: &Vec<Route>, key_travels: &KeyLookup) -> Vec<Route> {
    // SCC TO IMPLEMENT
    Vec::new()
}

fn find_min_distance(possible_routes: Vec<Route>) -> usize {
    // SCC TO IMPLEMENT
    0
}

// SCC This is an adaptation of the Robot from Day 15, heavily edited
// for use in a known maze and not using an IntComputer.
struct PathRobot {
    x_position: usize,
    y_position: usize,
    map: Vec<Vec<CellContent>>,
    node_hash: HashMap<(usize, usize), Node>,
}
impl PathRobot {
    fn check_direction(&self, direction: &Direction) -> CellContent {
        match direction {
            North => self.map[self.y_position - 1][self.x_position],
            East => self.map[self.y_position][self.x_position + 1],
            South => self.map[self.y_position + 1][self.x_position],
            West => self.map[self.y_position][self.x_position - 1],
        }
    }
    fn move_bot(&mut self, direction: &Direction) {
        // Move robot
        match direction {
            South => self.y_position += 1,
            West => self.x_position -= 1,
            North => self.y_position -= 1,
            East => self.x_position += 1,
        }
    }
    fn explore(&mut self) -> KeyLookup {
        // SCC TO IMPLEMENT
        // Two parts here:
        // - Part 1: Convert Map into a set of nodes (junctions, dead-ends, keys)
        let mut direction = self.choose_direction();
        while direction.is_some() {
            self.explore_single_path(direction.expect(""));
            direction = self.choose_direction();
        }

        // - Part 2: Convert Set into a KeyLookup.
        KeyLookup {
            lookup: HashMap::new(),
        }
    }
    fn choose_direction(&self) -> Option<Direction> {
        // Get the current node
        let current_node = self.node_hash.get(&(self.x_position, self.y_position)).expect("Should be at a node");

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
    fn explore_single_path(&mut self, direction: Direction) {
        // Move until we find ourself at another node.
        let orig_position = (self.x_position, self.y_position);
        let mut move_direction = direction;
        self.move_bot(&move_direction);
        let mut in_direction = Direction::invert(&move_direction);
        let mut distance = 1;
        let mut doors: Vec<char> = Vec::new();
        let mut key: Option<char> = None;

        loop {
            if self.at_key() {
                key = Some(self.map[self.y_position][self.x_position]);
                break;
            }
            let other_directions: Vec<Direction> = Direction::iterator()
                .filter(|&dir| self.check_direction_not_wall(dir) && *dir != in_direction).collect();
            if other_directions.len() != 1 {
                break;
            }
            if self.at_door() {
                // How do we go from the CellContent to the content letter
                doors.push(self.map[self.y_position][self.x_position]);
            }
            move_direction = other_directions[0];
            self.move_bot(&move_direction);
            distance += 1;
        }

        // We've reached a node!
        // If we haven't got a node for our location yet, create one and update the
        // original.
        if self.node_hash.get(&(self.x_position, self.y_position)).is_none() {
            let in_direction = Direction::invert(&move_direction);
            let other_paths: Vec<Path> = Direction::iterator()
                .filter(|dir| self.check_direction_not_wall(dir) && **dir != in_direction)
                .map(|dir| direction_to_path(dir))
                .collect();

            let new_node = Node {
                in_path: Some(Path {
                    direction: in_direction,
                    destination: Some(orig_position),
                }),
                other_paths: other_paths,
                key: key,
                distance: distance
            };
            self.node_hash.insert((self.x_position, self.y_position), new_node);

            let old_node = self.node_hash.remove(&orig_position).expect("where did we come from?");
            let mut updated_old_other_paths: Vec<Path> = old_node.other_paths
                .iter()
                .filter(|&path| path.direction != direction)
                .map(|&path| path)
                .collect::<Vec<Path>>();
            updated_old_other_paths.push(Path {
                    direction: direction,
                    destination: Some((self.x_position, self.y_position)),
                });

            let updated_old_node = Node {
                in_path: old_node.in_path,
                other_paths: updated_old_other_paths,
                key: old_node.key,
                distance: old_node.distance,
            };
            self.node_hash.insert(orig_position, updated_old_node);
        }
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
    Wall,
    Space,
    Door(char),
    Key(char),
}

struct Node {
    in_path: Option<Path>,
    other_paths: Vec<Path>,
    key: Option<char>,
    distance: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Path {
    direction: Direction,
    destination: Option<(usize, usize)>,
}

fn direction_to_path(direction: &Direction) -> Path {
    Path {
        direction: *direction,
        destination: None,
    }
}

struct Route {
    endpoint: char,
    distance: usize,
    keys: Vec<char>
}

struct KeyLookup {
    lookup: HashMap<char, HashMap<char, (usize, Vec<char>)>>
}
impl KeyLookup {
    fn len(&self) -> usize {
        self.lookup.keys().len()
    }
}
