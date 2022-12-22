use std::str::FromStr;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Couldn't parse {}", s)),
        }
    }
}

impl Direction {
    pub fn rotate(&self, rot: &Rotation) -> Self {
        match (self, rot) {
            (Direction::Left, Rotation::Left) => Direction::Down,
            (Direction::Left, Rotation::Right) => Direction::Up,
            (Direction::Right, Rotation::Left) => Direction::Up,
            (Direction::Right, Rotation::Right) => Direction::Down,
            (Direction::Up, Rotation::Left) => Direction::Left,
            (Direction::Up, Rotation::Right) => Direction::Right,
            (Direction::Down, Rotation::Left) => Direction::Right,
            (Direction::Down, Rotation::Right) => Direction::Left,
        }
    }
}

// pub enum CompassDirection {
//     North,
//     East,
//     South,
//     West
// }

pub enum Rotation {
    Left,
    Right,
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Couldn't parse {}", s)),
        }
    }
}
