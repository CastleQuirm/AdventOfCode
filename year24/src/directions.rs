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
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum CompassDirection {
    North,
    _East,
    _South,
    _West,
}

impl CompassDirection {
    pub fn _opposite(&self) -> Self {
        self.rotate(&Rotation::_Reverse)
    }

    pub fn rotate(&self, rotation: &Rotation) -> Self {
        // Guess I could implement a 'degrees' system to make this a bit less text heavy but...eh
        match (self, rotation) {
            (direction, Rotation::_Straight) => *direction,
            (Self::North, Rotation::Left) => Self::_West,
            (Self::North, Rotation::Right) => Self::_East,
            (Self::North, Rotation::_Reverse) => Self::_South,
            (Self::_East, Rotation::Left) => Self::North,
            (Self::_East, Rotation::Right) => Self::_South,
            (Self::_East, Rotation::_Reverse) => Self::_West,
            (Self::_South, Rotation::Left) => Self::_East,
            (Self::_South, Rotation::Right) => Self::_West,
            (Self::_South, Rotation::_Reverse) => Self::North,
            (Self::_West, Rotation::Left) => Self::_South,
            (Self::_West, Rotation::Right) => Self::North,
            (Self::_West, Rotation::_Reverse) => Self::_East,
        }
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
