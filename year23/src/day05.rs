// Potential improvements:
//

use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let conversion_tables = input_lines[1..]
        .iter()
        .map(|block| ConversionCode::new(block))
        .collect::<HashMap<String, ConversionCode>>();

    let mut conversion_sequence = Vec::new();
    let mut next_type = "seed";

    while next_type != "location" {
        let next_table = conversion_tables
            .get(next_type)
            .expect("Couldn't find next mapper");
        conversion_sequence.push(next_table.clone());
        next_type = &next_table.to_type;
    }

    let seed_line = input_lines[0][0]
        .strip_prefix("seeds: ")
        .expect("Bad start");
    let answer1 = seed_line
        .split_whitespace()
        .map(|seed| {
            conversion_sequence.iter().fold(
                seed.parse::<u64>().expect("Didn't parse a seed"),
                |val, table| table.apply(val),
            )
        })
        .min()
        .expect("No min?");

    let seed_pairs_regex = Regex::new(r"(\d+) (\d+)").unwrap();
    let seed_ranges = seed_pairs_regex
        .captures_iter(seed_line)
        .map(|cap| {
            let start_seed = cap[1].parse::<u64>().expect("Couldn't parse");
            let range = cap[2].parse::<u64>().expect("Couldn't parse");
            let mut seed_set = HashSet::new();
            for i in 0..range {
                seed_set.insert(start_seed + i);
            }
            seed_set
        })
        .reduce(|set, next| set.union(&next).cloned().collect::<HashSet<_>>())
        .expect("no seed sets?");

    let answer2 = seed_ranges
        .iter()
        .map(|&seed| {
            conversion_sequence
                .iter()
                .fold(seed, |val, table| table.apply(val))
        })
        .min()
        .expect("No min?");
    (format!("{}", answer1), format!("{}", answer2))
}

struct ConversionCode {
    to_type: String,
    maps: Vec<Map>,
}

impl ConversionCode {
    fn new(block: &[String]) -> (String, Self) {
        let (from_type, to_type) = block[0].split_once("-to-").expect("Title line was wrong");
        let maps = block[1..].iter().map(Map::from).collect::<Vec<Map>>();

        (
            from_type.to_string(),
            Self {
                to_type: to_type
                    .strip_suffix(" map:")
                    .expect("Title didn't end as expected")
                    .to_string(),
                maps,
            },
        )
    }

    fn apply(&self, value: u64) -> u64 {
        self.maps
            .iter()
            .find_map(|map| map.apply(value))
            .unwrap_or(value)
    }
}

struct Map {
    dest_base: u64,
    source_base: u64,
    range: u64,
}

impl From<&String> for Map {
    fn from(value: &String) -> Self {
        let values = value
            .split_whitespace()
            .map(|val| val.parse::<u64>().expect("Couldn't parse"))
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 3);
        Self {
            dest_base: values[0],
            source_base: values[1],
            range: values[2],
        }
    }
}

impl Map {
    fn apply(&self, value: u64) -> Option<u64> {
        if value >= self.source_base && value < self.source_base + self.range {
            Some(self.dest_base + value - self.source_base)
        } else {
            None
        }
    }

    // fn apply_range(&self, range)
}

// struct Range {
//     start: u64,
//     length: u64
// }

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4", // INPUT STRING
            "35", // PART 1 RESULT
            "46", // PART 2 RESULT
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
