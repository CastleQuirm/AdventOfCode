use std::{fmt::Display, str::FromStr};

pub fn load_input(whole_input: &str) -> Vec<Vec<String>> {
    let single_lines: Vec<String> = whole_input
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    let mut grouped_input: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    for line in single_lines {
        if line.is_empty() {
            grouped_input.push(current_group.clone());
            current_group = Vec::new();
        } else {
            current_group.push(line)
        }
    }
    grouped_input.push(current_group);
    grouped_input
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn manhattan_dist(&self, other: &Self) -> u32 {
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
        (abs_x_diff + abs_y_diff) as u32
    }

    pub fn plus(&self, x: i32, y: i32) -> Self {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn sum(&self, other: &Self) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum OpCode {
    AddI,
    AddR,
    MulI,
    MulR,
    BAnI,
    BAnR,
    BOrI,
    BOrR,
    SetI,
    SetR,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl OpCode {
    pub fn act(&self, mem: &[u64], a: u64, b: u64) -> u64 {
        match self {
            OpCode::AddI => mem[a as usize] + b,
            OpCode::AddR => mem[a as usize] + mem[b as usize],
            OpCode::MulI => mem[a as usize] * b,
            OpCode::MulR => mem[a as usize] * mem[b as usize],
            OpCode::BAnI => mem[a as usize] & b,
            OpCode::BAnR => mem[a as usize] & mem[b as usize],
            OpCode::BOrI => mem[a as usize] | b,
            OpCode::BOrR => mem[a as usize] | mem[b as usize],
            OpCode::SetI => a,
            OpCode::SetR => mem[a as usize],
            OpCode::GtIR => {
                if a > mem[b as usize] {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRI => {
                if mem[a as usize] > b {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRR => {
                if mem[a as usize] > mem[b as usize] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqIR => {
                if a == mem[b as usize] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRI => {
                if mem[a as usize] == b {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRR => {
                if mem[a as usize] == mem[b as usize] {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn act_as_usize(&self, mem: &[usize], a: usize, b: usize) -> usize {
        match self {
            OpCode::AddI => mem[a] + b,
            OpCode::AddR => mem[a] + mem[b],
            OpCode::MulI => mem[a] * b,
            OpCode::MulR => mem[a] * mem[b],
            OpCode::BAnI => mem[a] & b,
            OpCode::BAnR => mem[a] & mem[b],
            OpCode::BOrI => mem[a] | b,
            OpCode::BOrR => mem[a] | mem[b],
            OpCode::SetI => a,
            OpCode::SetR => mem[a],
            OpCode::GtIR => {
                if a > mem[b] {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRI => {
                if mem[a] > b {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRR => {
                if mem[a] > mem[b] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqIR => {
                if a == mem[b] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRI => {
                if mem[a] == b {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRR => {
                if mem[a] == mem[b] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl FromStr for OpCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addi" => Ok(Self::AddI),
            "addr" => Ok(Self::AddR),
            "muli" => Ok(Self::MulI),
            "mulr" => Ok(Self::MulR),
            "bani" => Ok(Self::BAnI),
            "banr" => Ok(Self::BAnR),
            "bori" => Ok(Self::BOrI),
            "borr" => Ok(Self::BOrR),
            "seti" => Ok(Self::SetI),
            "setr" => Ok(Self::SetR),
            "gtir" => Ok(Self::GtIR),
            "gtri" => Ok(Self::GtRI),
            "gtrr" => Ok(Self::GtRR),
            "eqir" => Ok(Self::EqIR),
            "eqri" => Ok(Self::EqRI),
            "eqrr" => Ok(Self::EqRR),
            _ => Err("Not a recognised OpCode".to_string()),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            OpCode::AddI => "addi",
            OpCode::AddR => "addr",
            OpCode::MulI => "muli",
            OpCode::MulR => "mulr",
            OpCode::BAnI => "bani",
            OpCode::BAnR => "banr",
            OpCode::BOrI => "bori",
            OpCode::BOrR => "borr",
            OpCode::SetI => "seti",
            OpCode::SetR => "setr",
            OpCode::GtIR => "gtir",
            OpCode::GtRI => "gtri",
            OpCode::GtRR => "gtrr",
            OpCode::EqIR => "eqir",
            OpCode::EqRI => "eqri",
            OpCode::EqRR => "eqrr",
        };
        f.write_str(text)
    }
}
