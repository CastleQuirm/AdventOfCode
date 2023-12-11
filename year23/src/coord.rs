// use once_cell::sync::OnceCell;
// use std::collections::HashSet;

use crate::directions::CompassDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2 {
    pub x: i64,
    pub y: i64,
}

impl Coord2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    // pub fn movement(direction: &Direction) -> Self {
    //     match direction {
    //         Direction::Left => Self { x: -1, y: 0 },
    //         Direction::Right => Self { x: 1, y: 0 },
    //         Direction::Up => Self { x: 0, y: 1 },
    //         Direction::Down => Self { x: 0, y: -1 },
    //     }
    // }

    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // pub fn plus(&self, x: i64, y: i64) -> Self {
    //     Self {
    //         x: self.x + x,
    //         y: self.y + y,
    //     }
    // }

    // pub fn sum(&self, other: &Self) -> Self {
    //     Self {
    //         x: self.x + other.x,
    //         y: self.y + other.y,
    //     }
    // }

    /// Get the coordinates of the next cell in a given cardinal direction, to be used with a grid
    /// N and W reduce indices, S and E increase.
    pub fn compass_sum(&self, direction: &CompassDirection) -> Self {
        match direction {
            CompassDirection::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            CompassDirection::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            CompassDirection::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            CompassDirection::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    // pub fn moved(&mut self, other: &Self) {
    //     self.x += other.x;
    //     self.y += other.y;
    // }

    // pub fn diff(&self, other: &Self) -> Self {
    //     Self {
    //         x: self.x - other.x,
    //         y: self.y - other.y,
    //     }
    // }

    // Determine the cardinal direction (as a Direction) from one coordinate to another, if there is one.
    // Returns None if the two points are the same, or if they're not in a straight line.
    // pub fn cardinal_direction_diff(&self, other: &Self) -> Option<Direction> {
    //     let diff = self.diff(other);

    //     let x = diff.x.checked_div(diff.x.abs()).unwrap_or(0);
    //     let y = diff.y.checked_div(diff.y.abs()).unwrap_or(0);

    //     match (x, y) {
    //         (-1, 0) => Some(Direction::Left),
    //         (1, 0) => Some(Direction::Right),
    //         (0, -1) => Some(Direction::Down),
    //         (0, 1) => Some(Direction::Up),
    //         _ => None,
    //     }
    // }
}

// static RE2: OnceCell<Regex> = OnceCell::new();

// impl FromStr for Coord2 {
//     type Err = ParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(RE2
//             .get_or_init(|| Regex::new(r"(-?\d+),(-?\d+)").unwrap())
//             .captures(s)
//             .map(|cap| Self {
//                 x: cap[1].parse::<i64>().unwrap(),
//                 y: cap[2].parse::<i64>().unwrap(),
//             })
//             .expect("Didn't parse"))
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coord3 {
    // pub fn new(x: i64, y: i64, z: i64) -> Self {
    //     Self { x, y, z }
    // }

    // pub fn movement(direction: &Direction) -> Self {
    //     match direction {
    //         Direction::Left => Self { x: -1, y: 0 },
    //         Direction::Right => Self { x: 1, y: 0 },
    //         Direction::Up => Self { x: 0, y: 1 },
    //         Direction::Down => Self { x: 0, y: -1 },
    //     }
    // }

    // pub fn manhattan_dist(&self, other: &Self) -> i64 {
    //     (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    // }

    // pub fn plus(&self, x: i64, y: i64, z: i64) -> Self {
    //     Self {
    //         x: self.x + x,
    //         y: self.y + y,
    //         z: self.z + z,
    //     }
    // }

    // pub fn sum(&self, other: &Self) -> Self {
    //     Self {
    //         x: self.x + other.x,
    //         y: self.y + other.y,
    //         z: self.z + other.z,
    //     }
    // }

    // pub fn moved(&mut self, other: &Self) {
    //     self.x += other.x;
    //     self.y += other.y;
    //     self.z += other.z;
    // }

    // pub fn diff(&self, other: &Self) -> Self {
    //     Self {
    //         x: self.x - other.x,
    //         y: self.y - other.y,
    //         z: self.z - other.z,
    //     }
    // }

    // pub fn unit_deltas() -> HashSet<Self> {
    //     HashSet::from([
    //         Coord3 { x: -1, y: 0, z: 0 },
    //         Coord3 { x: 1, y: 0, z: 0 },
    //         Coord3 { x: 0, y: -1, z: 0 },
    //         Coord3 { x: 0, y: 1, z: 0 },
    //         Coord3 { x: 0, y: 0, z: -1 },
    //         Coord3 { x: 0, y: 0, z: 1 },
    //     ])
    // }

    // /// Determine the cardinal direction (as a Direction) from one coordinate to another, if there is one.
    // /// Returns None if the two points are the same, or if they're not in a straight line.
    // pub fn cardinal_direction_diff(&self, other: &Self) -> Option<Direction> {
    //     let diff = self.diff(other);

    //     let x = diff.x.checked_div(diff.x.abs()).unwrap_or(0);
    //     let y = diff.y.checked_div(diff.y.abs()).unwrap_or(0);

    //     match (x, y) {
    //         (-1, 0) => Some(Direction::Left),
    //         (1, 0) => Some(Direction::Right),
    //         (0, -1) => Some(Direction::Down),
    //         (0, 1) => Some(Direction::Up),
    //         _ => None,
    //     }
    // }
}

// static RE3: OnceCell<Regex> = OnceCell::new();

// impl FromStr for Coord3 {
//     type Err = ParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(RE3
//             .get_or_init(|| Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap())
//             .captures(s)
//             .map(|cap| Self {
//                 x: cap[1].parse::<i64>().unwrap(),
//                 y: cap[2].parse::<i64>().unwrap(),
//                 z: cap[3].parse::<i64>().unwrap(),
//             })
//             .expect("Didn't parse"))
//     }
// }
