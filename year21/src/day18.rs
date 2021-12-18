// Potential improvements:
// 1. Actually learn lifetimes and pointers!

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let part1 = input_lines.iter().map(|line| Snumber::new(line)).reduce(|a, b| a.add(&b)).expect("No Snumbers?").magnitude();

    (part1, 0)
}

#[derive(Clone, Debug)]
struct Snumber {
    // value: String
    left: Content,
    right: Content,
}

impl Snumber {
    fn new(line: &str) -> Self {
        Self { left: Content::Number{ value: 0 }, right: Content::Number{ value: 0 } }
    }

    fn reduce(&self) -> Self {
        Self { left: Content::Number{ value: 0 }, right: Content::Number{ value: 0 } }

        // let mut_next_left_digit: Option<char> = None;
        // let mut nesting_count = 0;
        // let mut first_explode: Option<usize> = None;

        // let char_string = self.value.chars().collect::<Vec<char>>();

        // for i in 0..char_string.len() {
        //     match char_string[i] {
        //         '[' => {
        //             nesting_count += 1;
        //             if nesting_count > 4 { 
        //                 // Start explode
        //                 assert_eq!(nesting_count, 5);
        //             }
        //         }
        //         ']' => {
        //             assert_ne!(nesting_count, 0);
        //             nesting_count -= 1
        //         }
        //         '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {

        //         }
        //         _ => panic!(),
        //     }
        // }

        // self
    }

    fn add(&self, other: &Snumber) -> Self {
        // let mut raw_addition = "[".to_string();
        // raw_addition.push_str(&self.value);
        // raw_addition.push_str(",");
        // raw_addition.push_str(&other.value);
        // raw_addition.push_str("]");

        // Self { value: raw_addition }.reduce()
        Self { left: Content::Snumber{ snumber: vec![self.clone()] }, right: Content::Snumber{ snumber: vec![other.clone()] } }
    }

    fn magnitude(&self) -> u64 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
}

#[derive(Clone, Debug)]
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
