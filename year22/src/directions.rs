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

// pub enum CompassDirection {
//     North,
//     East,
//     South,
//     West
// }
