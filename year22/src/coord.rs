use once_cell::sync::OnceCell;
use std::{str::FromStr, string::ParseError};

use regex::Regex;

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

    pub fn plus(&self, x: i64, y: i64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

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

    pub fn diff(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Determine the cardinal direction (as a Direction) from one coordinate to another, if there is one.
    /// Returns None if the two points are the same, or if they're not in a straight line.
    pub fn cardinal_direction_diff(&self, other: &Self) -> Option<Direction> {
        let diff = self.diff(other);

        let x = diff.x.checked_div(diff.x.abs()).unwrap_or(0);
        let y = diff.y.checked_div(diff.y.abs()).unwrap_or(0);

        match (x, y) {
            (-1, 0) => Some(Direction::Left),
            (1, 0) => Some(Direction::Right),
            (0, -1) => Some(Direction::Down),
            (0, 1) => Some(Direction::Up),
            _ => None,
        }
    }
}

static RE: OnceCell<Regex> = OnceCell::new();

impl FromStr for Coord2 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RE
            .get_or_init(|| Regex::new(r"(-?\d+),(-?\d+)").unwrap())
            .captures(s)
            .map(|cap| Self {
                x: cap[1].parse::<i64>().unwrap(),
                y: cap[2].parse::<i64>().unwrap(),
            })
            .expect("Didn't parse"))
    }
}
