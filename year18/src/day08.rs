// Potential improvements:
// 1. Could/should cache node's values for efficiency.

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    // Read the input
    let digits = input_lines[0][0]
        .split(' ')
        .map(|s| s.parse::<usize>().expect("Didn't parse a number"))
        .collect::<Vec<usize>>();
    let (root, parsed_digits) = Node::parse(&digits);
    assert_eq!(parsed_digits, digits.len());

    // Get the answers
    let answer1 = root.metadata_sum();
    let answer2 = root.value();
    (format!("{}", answer1), format!("{}", answer2))
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(digits: &[usize]) -> (Self, usize) {
        assert!(digits.len() >= 3);

        let number_of_children = digits[0];
        let number_of_metadata = digits[1];
        let mut consumed_digits = 2;

        let mut children: Vec<Node> = Vec::new();
        for _ in 0..number_of_children {
            let (child, movement) = Node::parse(&digits[consumed_digits..]);
            consumed_digits += movement;
            children.push(child);
        }

        let metadata: Vec<usize> =
            digits[consumed_digits..(consumed_digits + number_of_metadata)].to_vec();
        consumed_digits += number_of_metadata;

        (Node { children, metadata }, consumed_digits)
    }

    fn metadata_sum(&self) -> usize {
        self.children
            .iter()
            .fold(self.metadata.iter().sum(), |acc, child| {
                acc + child.metadata_sum()
            })
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|child_ix| self.children.get(*child_ix - 1))
                .map(|child| child.value())
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", // INPUT STRING
            "138",                                 // PART 1 RESULT
            "66",                                  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day08(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
