// Potential improvements:
//

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    assert_eq!(input_lines.len(), 2);

    // Read out the ranges and order them.
    let mut fresh_ranges = input_lines[0].iter().map(Range::from).collect::<Vec<_>>();
    fresh_ranges.sort();

    // Merge these into the minimum possible set of ranges
    let mut final_fresh_ranges = vec![];
    let mut consuming_range = fresh_ranges[0];
    fresh_ranges[1..].iter().for_each(|range| {
        if let Some(merged_range) = consuming_range.merge_if_poss(range) {
            consuming_range = merged_range;
        } else {
            final_fresh_ranges.push(consuming_range);
            consuming_range = *range;
        }
    });

    // Examine the food ids.
    let answer1 = input_lines[1]
        .iter()
        .filter(|line| {
            let id = line.parse::<u64>().expect("Bad ID");
            final_fresh_ranges.iter().any(|range| range.contains(id))
        })
        .count();

    let answer2 = final_fresh_ranges
        .iter()
        .map(|range| range.total_size())
        .sum::<u64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Range {
    low: u64,
    high: u64,
}

impl From<&String> for Range {
    fn from(range: &String) -> Self {
        let values = range.split_once('-').expect("bad range");
        let low = values.0.parse::<u64>().expect("Bad low");
        let high = values.1.parse::<u64>().expect("Bad high");
        assert!(high >= low);
        Self { low, high }
    }
}

impl Range {
    fn merge_if_poss(&self, other: &Range) -> Option<Range> {
        let (bottom, top) = if self.low <= other.low {
            (self, other)
        } else {
            (other, self)
        };
        if top.low <= bottom.high + 1 {
            Some(Self {
                low: bottom.low,
                high: bottom.high.max(top.high),
            })
        } else {
            None
        }
    }

    fn contains(&self, food_id: u64) -> bool {
        food_id >= self.low && food_id <= self.high
    }

    fn total_size(&self) -> u64 {
        self.high - self.low + 1
    }
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32", // INPUT STRING
            "3",  // PART 1 RESULT
            "14", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day05(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
