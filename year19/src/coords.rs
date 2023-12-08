#[derive(Debug, PartialEq, Eq)]
pub struct Coord2 {
    x: i64,
    y: i64,
}

impl Coord2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_dist(&self, other: &Self) -> i64 {
        let x_diff = if self.x >= other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y_diff = if self.y >= other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        x_diff + y_diff
    }

    pub fn sum(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }
}
