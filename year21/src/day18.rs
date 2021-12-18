// Potential improvements:
// 1. Actually learn lifetimes and pointers!

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let part1 = input_lines
        .iter()
        .map(|line| Snumber::new(&line.chars().collect::<Vec<char>>()).0)
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
    fn new(line: &[char]) -> (Self, usize) {
        let mut index: usize = 0;
        assert_eq!(line[index], '[');
        index += 1;
        let left = match line[index] {
            '[' => {
                let (sub_snumber, index_moved) = Snumber::new(&line[index..]);
                index += index_moved;
                Content::Snumber { snumber: vec![sub_snumber]}
            }
            c => {
                index += 1;
                Content::Number { value: c.to_string().parse::<u64>().expect("Didn't parse what should be a digit")}
            }
        };
        assert_eq!(line[index], ',');
        index += 1;
        let right = match line[index] {
            '[' => {
                let (sub_snumber, index_moved) = Snumber::new(&line[index..]);
                index += index_moved;
                Content::Snumber { snumber: vec![sub_snumber]}
            }
            c => {
                index += 1;
                Content::Number { value: c.to_string().parse::<u64>().expect("Didn't parse what should be a digit")}
            }
        };
        assert_eq!(line[index], ']');

        (Self {
            left: Content::Number { value: 0 },
            right: Content::Number { value: 0 },
        },
        index)
    }

    fn reduce(&mut self) {
        // TODO
        let mut candidate_split: Option<Vec<Direction>> = None;
        let mut direction_vector: Vec<Direction> = self.find_leftmost_direction();

        // Walk right until one of three things happens:
        // We reach a Content::Number whose value is > 9 => if candidate_split.is_none() { candidate_split = Some(direction_vector.clone()) }.
        // We reach a Content::Snumber with a direction_vector.len() > 4 (should only ever be 5) => ditch canidate_split, explode, start from the top
        // We reach the end of the Snumber: in which case - if candidate.split().is_some(), split that number; else end.

        while !direction_vector.is_empty() {
            if direction_vector.len() > 4 {
                assert_eq!(direction_vector.len(), 5);
                assert_eq!(direction_vector.pop(), Some(Direction::Left));
                let exploding_snumber = match self
                    .get_content(&direction_vector)
                    .expect("didn't find content")
                {
                    Content::Snumber { snumber } => snumber[0].clone(),
                    Content::Number { value: _ } => panic!(),
                };
                let left_explode = match exploding_snumber.left {
                    Content::Number { value } => value,
                    Content::Snumber { snumber: _ } => panic!(),
                };
                let right_explode = match exploding_snumber.right {
                    Content::Number { value } => value,
                    Content::Snumber { snumber: _ } => panic!(),
                };

                self.update_content(&direction_vector, Content::Number { value: 0 });

                let mut left_direction = direction_vector.clone();
                self.walk_left(&mut left_direction);
                match self.get_content(&left_direction) {
                    Some(Content::Snumber { snumber: _ }) => panic!("walk left got a snumber"),
                    Some(Content::Number { value }) => {
                        let new_value = left_explode + *value;
                        if candidate_split.is_none() && new_value > 9 {
                            candidate_split = Some(left_direction.clone())
                        }
                        self.update_content(&left_direction, Content::Number { value: new_value })
                    }
                    None => (),
                }

                let right_direction = direction_vector.clone();
                self.walk_right(&mut right_direction.clone());
                match self.get_content(&right_direction) {
                    Some(Content::Snumber { snumber: _ }) => panic!("walk right got a snumber"),
                    Some(Content::Number { value }) => {
                        let new_value = right_explode + *value;
                        self.update_content(&right_direction, Content::Number { value: new_value })
                    }
                    None => (),
                }
            }
            match self
                .get_content(&direction_vector)
                .expect("didn't find content")
            {
                Content::Number { value } if *value > 9 && candidate_split.is_none() => {
                    candidate_split = Some(direction_vector.clone());
                }
                _ => (),
            }
            self.walk_right(&mut direction_vector);
        }

        if let Some(split_spot) = candidate_split {
            match self
                .get_content(&split_spot)
                .expect("Didn't actually have a split")
            {
                Content::Snumber { snumber } => panic!("Trying to split a snumber"),
                Content::Number { value } => self.update_content(
                    &split_spot,
                    Content::Snumber {
                        snumber: vec![Snumber {
                            left: Content::Number { value: *value / 2 },
                            right: Content::Number {
                                value: *value - (*value / 2),
                            },
                        }],
                    },
                ),
            }
            // TODO GO BACK TO THE START
        }
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

    fn find_rightmost_direction(&self) -> Vec<Direction> {
        let mut directions = vec![Direction::Right];
        let mut snumber_reached = self;
        loop {
            match &snumber_reached.right {
                Content::Number { value: _ } => return directions,
                Content::Snumber {
                    snumber: next_snumber,
                } => {
                    directions.push(Direction::Right);
                    snumber_reached = &next_snumber[0];
                }
            }
        }
    }

    fn walk_right(&self, directions: &mut Vec<Direction>) {
        // If the last direction is Left, it becomes Right and we check if we need to add any more Lefts after it
        // If the last direction is Right, strip it and walk Right from the parent (assuming it's not empty).
        let last_direction = directions
            .pop()
            .expect("Tried to walk right on an empty vec");
        match last_direction {
            Direction::Left => {
                directions.push(Direction::Right);
                match self.get_content(directions).expect("Didn't find content") {
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

    fn walk_left(&self, directions: &mut Vec<Direction>) {
        // If the last direction is Right, it becomes Left and we check if we need to add any more Rights after it
        // If the last direction is Left, strip it and walk Left from the parent (assuming it's not empty).
        let last_direction = directions
            .pop()
            .expect("Tried to walk right on an empty vec");
        match last_direction {
            Direction::Right => {
                directions.push(Direction::Left);
                match self.get_content(directions).expect("Didn't find content") {
                    Content::Number { value: _ } => (),
                    Content::Snumber {
                        snumber: next_snumber,
                    } => {
                        let mut further_directions = next_snumber[0].find_rightmost_direction();
                        directions.append(&mut further_directions);
                    }
                }
            }
            Direction::Left => {
                if !directions.is_empty() {
                    self.walk_left(directions);
                }
            }
        }
    }

    fn get_content(&mut self, directions: &Vec<Direction>) -> Option<&mut Content> {
        let mut content: Option<&mut Content> = None;
        let mut current_snumber = self;
        directions.iter().for_each(|path| {
            match content {
                Some(Content::Snumber {
                    snumber: next_snumber,
                }) => current_snumber = &mut next_snumber[0],
                Some(_) => panic!(),
                None => (),
            }
            match path {
                Direction::Left => content = Some(&mut current_snumber.left),
                Direction::Right => content = Some(&mut current_snumber.right),
            }
        });
        content
    }

    fn update_content(&mut self, directions: &Vec<Direction>, new_content: Content) {
        let mut directions = directions.clone();
        let last_direction = directions.pop().expect("No last direction");
        let content = self.get_content(&directions);
        let content_snumber = match content {
            Some(Content::Snumber { snumber }) => &snumber[0],
            Some(Content::Number { value: _ }) => panic!(),
            None => self,
        };
        match last_direction {
            Direction::Left => content_snumber.left = new_content.clone(),
            Direction::Right => content_snumber.right = new_content.clone(),
        }
    }

    fn add(&self, other: &Snumber) -> Self {
        let result = Self {
            left: Content::Snumber {
                snumber: vec![self.clone()],
            },
            right: Content::Snumber {
                snumber: vec![other.clone()],
            },
        };
        result.reduce();
        result
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
    use super::Snumber;
    use super::Content;

    #[test]
    fn check_day18() {
        let input_lines = ""
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day18(&input_lines), (0, 0));
    }

    #[test]
    fn check_day18_new_snumbers() {
        let input = "[1,2]".to_string().chars().collect::<Vec<char>>();
        assert_eq!(Snumber::new(&input), (Snumber { left: Content::Number { value: 1} , right: Content::Number { value: 2 } }, 5));
    }
}
