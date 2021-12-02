// Potential improvements:
// Handle possibility of negative aim (although negative p1_depth would be weird conceptually for Part 1!)
// Rename SubmarineState fields to be more obvious!
// Make the direction values an enum?
// Work out a way to make things less mutable?  e.g. create a new SubmarineState for each line?

pub fn day02(input_lines: &[String]) -> (u64, u64) {
    let mut submarine = SubmarineState::new();
    input_lines
        .iter()
        .for_each(|line| submarine.process_instruction(line));
    submarine.solution()
}

fn direction_and_val(line: &str) -> (&str, u64) {
    let splitline = line.split(' ').collect::<Vec<&str>>();
    let direction = splitline.first().expect("No first part of splitline");
    let val = splitline
        .last()
        .expect("No second part of splitline")
        .parse::<u64>()
        .expect("Couldn't parse the second part as a u64");
    (direction, val)
}
struct SubmarineState {
    // Horizontal distance travelled. This is the same for both parts.
    dist: u64,
    // Depth of the submarine in Part 1; this is also the "aim" in Part 2.
    p1_depth: u64,
    // Depth of the submarine in Part 2.
    p2_depth: u64,
}

impl SubmarineState {
    fn new() -> Self {
        Self {
            dist: 0,
            p1_depth: 0,
            p2_depth: 0,
        }
    }
    fn forward(&mut self, val: u64) {
        self.dist += val;
        self.p2_depth += val * self.p1_depth;
    }
    fn up(&mut self, val: u64) {
        self.p1_depth -= val;
    }
    fn down(&mut self, val: u64) {
        self.p1_depth += val;
    }
    fn process_instruction(&mut self, instruction: &str) {
        let (direction, val) = direction_and_val(instruction);
        match direction {
            "forward" => self.forward(val),
            "up" => self.up(val),
            "down" => self.down(val),
            _ => panic!("Unrecognised direction"),
        }
    }
    fn solution(&self) -> (u64, u64) {
        (self.dist * self.p1_depth, self.dist * self.p2_depth)
    }
}
