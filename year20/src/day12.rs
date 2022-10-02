pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let mut ship_part1 = Ship {
        north: 0,
        east: 0,
        waypoint_north: 0,
        waypoint_east: 0,
        facing: Direction::East,
    };
    for line in input_lines {
        ship_part1.sail_part1(line)
    }
    let mut ship_part2 = Ship {
        north: 0,
        east: 0,
        waypoint_north: 1,
        waypoint_east: 10,
        facing: Direction::East,
    };
    for line in input_lines {
        ship_part2.sail_part2(line)
    }
    (ship_part1.manhattan(), ship_part2.manhattan())
}

#[derive(Clone, Copy)]
struct Ship {
    north: i64,
    east: i64,
    waypoint_north: i64,
    waypoint_east: i64,
    facing: Direction,
}
impl Ship {
    fn sail_part1(&mut self, line: &str) {
        let instruction = line.chars().next().unwrap();
        let amount = line[1..].parse::<i64>().unwrap();

        match instruction {
            'N' => self.north += amount,
            'E' => self.east += amount,
            'S' => self.north -= amount,
            'W' => self.east -= amount,
            'L' => self.facing = self.facing.turn_left(amount),
            'R' => self.facing = self.facing.turn_left(-amount),
            'F' => self.forwards(amount),
            _ => unreachable!(),
        }
    }

    fn sail_part2(&mut self, line: &str) {
        let instruction = line.chars().next().unwrap();
        let amount = line[1..].parse::<i64>().unwrap();

        match instruction {
            'N' => self.waypoint_north += amount,
            'E' => self.waypoint_east += amount,
            'S' => self.waypoint_north -= amount,
            'W' => self.waypoint_east -= amount,
            'L' => self.waypoint_left(amount),
            'R' => self.waypoint_left(-amount),
            'F' => self.move_to_waypoint(amount),
            _ => unreachable!(),
        }
    }

    fn forwards(&mut self, amount: i64) {
        match self.facing {
            Direction::North => self.north += amount,
            Direction::East => self.east += amount,
            Direction::South => self.north -= amount,
            Direction::West => self.east -= amount,
        }
    }

    fn waypoint_left(&mut self, degrees: i64) {
        assert_eq!(degrees % 90, 0);
        match (degrees % 360) / 90 {
            0 => (),
            // Not clear why doing self.waypoint_left_90().waypoint_left_90() doesn't cause the second one to stick.
            -3 | 1 => {
                self.waypoint_left_90();
            }
            -2 | 2 => {
                self.waypoint_left_90();
                self.waypoint_left_90();
            }
            -1 | 3 => {
                self.waypoint_left_90();
                self.waypoint_left_90();
                self.waypoint_left_90();
            }
            _ => unreachable!(),
        }
    }

    fn waypoint_left_90(&mut self) {
        let original_waypoint_north = self.waypoint_north;
        self.waypoint_north = self.waypoint_east;
        self.waypoint_east = -original_waypoint_north;
    }

    fn move_to_waypoint(&mut self, amount: i64) {
        self.north += amount * self.waypoint_north;
        self.east += amount * self.waypoint_east;
    }

    fn manhattan(self) -> u64 {
        (self.north.abs() + self.east.abs()) as u64
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn turn_left(self, degrees: i64) -> Direction {
        assert_eq!(degrees % 90, 0);
        match (degrees % 360) / 90 {
            0 => self,
            -3 | 1 => self.turn_left_90(),
            -2 | 2 => self.turn_180(),
            -1 | 3 => self.turn_right_90(),
            _ => unreachable!(),
        }
    }
    // I could consolidate turn_left_90, turn_right_90 and turn_180 into a single function by
    // having the callers instead call the single choice multiple times, but this isn't likely to
    // be code I want to extend or change much, so might as well have the fewer calls.
    fn turn_left_90(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn turn_right_90(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn turn_180(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day12;

    #[test]
    fn sample_input() {
        let sample = "F10
N3
F7
R90
F11"
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
        assert_eq!(day12(&sample), (25, 286));
    }
}
