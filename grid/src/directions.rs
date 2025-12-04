use std::str::FromStr;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
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
    pub fn _rotate(&self, rot: &Rotation) -> Self {
        match (self, rot) {
            // Could commonise with CompassDirection::rotate but...eh.
            (dir, Rotation::_Straight) => *dir,
            (Direction::Left, Rotation::Left) => Direction::Down,
            (Direction::Left, Rotation::Right) => Direction::Up,
            (Direction::Right, Rotation::Left) => Direction::Up,
            (Direction::Right, Rotation::Right) => Direction::Down,
            (Direction::Up, Rotation::Left) => Direction::Left,
            (Direction::Up, Rotation::Right) => Direction::Right,
            (Direction::Down, Rotation::Left) => Direction::Right,
            (Direction::Down, Rotation::Right) => Direction::Left,
            (Direction::Left, Rotation::_Reverse) => Direction::Right,
            (Direction::Right, Rotation::_Reverse) => Direction::Left,
            (Direction::Up, Rotation::_Reverse) => Direction::Down,
            (Direction::Down, Rotation::_Reverse) => Direction::Up,
        }
    }

    /// Determine the rotation needed to get from Self to Other, as if you were travelling
    /// in the direction of Self (viewed from above) and want to be travelling in the direction
    /// of Other.  'Right' could also be 'Clockwise' and 'Left' 'Anti-Clockwise'.
    // TODO 'could' => 'SHOULD'?!
    pub fn _count_rotation(&self, turn_to: &Self) -> Rotation {
        match (self, turn_to) {
            (Direction::Left, Direction::Left) => Rotation::_Straight,
            (Direction::Left, Direction::Right) => Rotation::_Reverse,
            (Direction::Left, Direction::Up) => Rotation::Right,
            (Direction::Left, Direction::Down) => Rotation::Left,
            (Direction::Right, Direction::Left) => Rotation::_Reverse,
            (Direction::Right, Direction::Right) => Rotation::_Straight,
            (Direction::Right, Direction::Up) => Rotation::Left,
            (Direction::Right, Direction::Down) => Rotation::Right,
            (Direction::Up, Direction::Left) => Rotation::Left,
            (Direction::Up, Direction::Right) => Rotation::Right,
            (Direction::Up, Direction::Up) => Rotation::_Straight,
            (Direction::Up, Direction::Down) => Rotation::_Reverse,
            (Direction::Down, Direction::Left) => Rotation::Right,
            (Direction::Down, Direction::Right) => Rotation::Left,
            (Direction::Down, Direction::Up) => Rotation::_Reverse,
            (Direction::Down, Direction::Down) => Rotation::_Straight,
        }
    }

    /// Produce an iterator of each Direction (starting with Up and moving clockwise)
    pub fn _iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .into_iter()
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum CompassDirection {
    North,
    East,
    South,
    West,
}

impl CompassDirection {
    pub fn opposite(&self) -> Self {
        self.rotate(&Rotation::_Reverse)
    }

    pub fn rotate(&self, rotation: &Rotation) -> Self {
        // Guess I could implement a 'degrees' system to make this a bit less text heavy but...eh
        match (self, rotation) {
            (direction, Rotation::_Straight) => *direction,
            (Self::North, Rotation::Left) => Self::West,
            (Self::North, Rotation::Right) => Self::East,
            (Self::North, Rotation::_Reverse) => Self::South,
            (Self::East, Rotation::Left) => Self::North,
            (Self::East, Rotation::Right) => Self::South,
            (Self::East, Rotation::_Reverse) => Self::West,
            (Self::South, Rotation::Left) => Self::East,
            (Self::South, Rotation::Right) => Self::West,
            (Self::South, Rotation::_Reverse) => Self::North,
            (Self::West, Rotation::Left) => Self::South,
            (Self::West, Rotation::Right) => Self::North,
            (Self::West, Rotation::_Reverse) => Self::East,
        }
    }

    /// Produce an iterator of each CompassDirection (starting with North and moving clockwise)
    pub fn iter() -> impl Iterator<Item = CompassDirection> {
        [
            CompassDirection::North,
            CompassDirection::East,
            CompassDirection::South,
            CompassDirection::West,
        ]
        .into_iter()
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Rotation {
    Left,
    Right,
    _Straight,
    _Reverse,
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
