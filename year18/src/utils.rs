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
            grouped_input = Vec::new();
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
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
