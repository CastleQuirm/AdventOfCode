// Potential improvements:
// 1. Actually learn lifetimes and pointers!

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let part1 = input_lines
        .iter()
        .map(|line| Snumber::new(line))
        .reduce(|a, b| a.add(&b))
        .expect("No Snumbers?")
        .magnitude();

    (part1, 0)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snumber {
    left: Content,
    right: Content,
}

impl Snumber {
    fn new(line: &str) -> Self {
        // TODO
        Self {
            left: Content::Number { value: 0 },
            right: Content::Number { value: 0 },
        }
    }

    fn reduce(&self) -> Self {
        // TODO
        let mut reduced_snumber = self.clone();
        let mut candidate_split: Option<Vec<Direction>> = None;
        let mut direction_vector: Vec<Direction> = self.find_leftmost_direction();

        // Walk right until one of three things happens:
        // We reach a Content::Number whose value is > 9 => if candidate_split.is_none() { candidate_split = Some(direction_vector.clone()) }.
        // We reach a Content::Snumber with a direction_vector.len() > 4 (should only ever be 5) => ditch canidate_split, explode, start from the top
        // We reach the end of the Snumber: in which case - if candidate.split().is_some(), split that number; else end.

        while !direction_vector.is_empty() {
            if direction_vector.len() > 4 {
                assert_eq!(direction_vector.len(), 5);
                candidate_split = None;
                // TODO EXPLODE
            }
            match self.get_content(&direction_vector) {
                Content::Number { value } if *value > 9 && candidate_split.is_none() => {
                    candidate_split = Some(direction_vector.clone());
                }
                _ => (),
            }
            self.walk_right(&mut direction_vector);
        }

        if !candidate_split.is_none() {
            // TODO SPLIT
            // TODO GO BACK TO THE START
        }

        reduced_snumber
    }

    fn find_leftmost_direction(&self) -> Vec<Direction> {
        let mut directions = vec![Direction::Left];
        let mut snumber_reached = self;
        loop {
            match &snumber_reached.left {
                Content::Number { value: _ } => return directions,
                Content::Snumber {
                    snumber: next_snumber,
                } => {
                    directions.push(Direction::Left);
                    snumber_reached = &next_snumber[0];
                }
            }
        }
    }

    fn walk_right(&self, directions: &mut Vec<Direction>) {
        // If the last direction is Left, it becomes Right and we check if we need to add any more lefts after it
        // If the last direction is Right, strip it and walk right from the parent (assuming it's not empty).
        let last_direction = directions
            .pop()
            .expect("Tried to walk right on an empty vec");
        match last_direction {
            Direction::Left => {
                directions.push(Direction::Right);
                let starting_root = self.get_content(directions);
                match starting_root {
                    Content::Number { value: _ } => (),
                    Content::Snumber {
                        snumber: next_snumber,
                    } => {
                        let mut further_directions = next_snumber[0].find_leftmost_direction();
                        directions.append(&mut further_directions);
                    }
                }
            }
            Direction::Right => {
                if !directions.is_empty() {
                    self.walk_right(directions);
                }
            }
        }
    }

    fn get_content(&self, directions: &Vec<Direction>) -> &Content {
        let mut content: Option<&Content> = None;
        let mut current_snumber = self;
        directions.iter().for_each(|path| {
            match content {
                Some(Content::Snumber {
                    snumber: next_snumber,
                }) => current_snumber = &next_snumber[0],
                Some(_) => panic!(),
                None => (),
            }
            match path {
                Direction::Left => content = Some(&current_snumber.left),
                Direction::Right => content = Some(&current_snumber.right),
            }
        });
        content.expect("Didn't get any content!")
    }

    fn add(&self, other: &Snumber) -> Self {
        Self {
            left: Content::Snumber {
                snumber: vec![self.clone()],
            },
            right: Content::Snumber {
                snumber: vec![other.clone()],
            },
        }
        .reduce()
    }

    fn magnitude(&self) -> u64 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Content {
    Snumber { snumber: Vec<Snumber> }, // We store this as a vec because that tricks Rust into allowing me to have recursive Snumbers!
    Number { value: u64 },
}
impl Content {
    fn magnitude(&self) -> u64 {
        match self {
            Content::Number { value } => *value,
            Content::Snumber { snumber } => snumber[0].magnitude(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::day18;

    #[test]
    fn check_day18() {
        let input_lines = ""
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day18(&input_lines), (0, 0));
    }
}
