use crate::directions::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2 {
    pub x: i64,
    pub y: i64,
}

impl Coord2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn movement(direction: &Direction) -> Self {
        match direction {
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
            Direction::Up => Self { x: 0, y: 1 },
            Direction::Down => Self { x: 0, y: -1 },
        }
    }

    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        let abs_x_diff = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let abs_y_diff = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        (abs_x_diff + abs_y_diff) as i64
    }

    // pub fn plus(&self, x: i64, y: i64) -> Self {
    //     Self {
    //         x: self.x + x,
    //         y: self.y + y,
    //     }
    // }

    pub fn sum(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn moved(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
