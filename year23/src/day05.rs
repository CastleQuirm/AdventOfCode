// Potential improvements:
//

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

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
        conversion_sequence.push(<ConversionCode>::clone(next_table));
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
        .map(|cap| Range {
            start: cap[1].parse::<u64>().expect("Couldn't parse"),
            length: cap[2].parse::<u64>().expect("Couldn't parse"),
        })
        .sorted()
        .collect::<Vec<Range>>();

    let mut assorted_ranges = seed_ranges;
    for conversion_step in conversion_sequence {
        assorted_ranges = conversion_step.apply_to_ranges(assorted_ranges);
    }

    let answer2 = assorted_ranges.first().expect("No resulting ranges?").start;
    // let answer2 = seed_ranges
    //     .iter()
    //     .map(|&seed| {
    //         conversion_sequence
    //             .iter()
    //             .fold(seed, |val, table| table.apply(val))
    //     })
    //     .min()
    //     .expect("No min?");
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone)]
struct ConversionCode {
    to_type: String,
    maps: Vec<Map>,
}

impl ConversionCode {
    fn new(block: &[String]) -> (String, Self) {
        let (from_type, to_type) = block[0].split_once("-to-").expect("Title line was wrong");
        let maps = block[1..]
            .iter()
            .map(Map::from)
            .sorted_by(|map1, map2| map1.range.start.cmp(&map2.range.start))
            .collect::<Vec<Map>>();

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
            .find_map(|map| map.apply_to_value(value))
            .unwrap_or(value)
    }

    fn apply_to_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut ordered_ranges = ranges.clone();
        ordered_ranges.sort();
        assert_eq!(ranges, ordered_ranges);
        for range_pair in ordered_ranges.windows(2) {
            assert_eq!(range_pair.len(), 2);
            assert!(range_pair[0].end_inc() < range_pair[1].start);
        }
        let mut ordered_ranges = ordered_ranges.into_iter().collect::<VecDeque<_>>();

        let mut result_ranges = Vec::new();
        let mut map_iter = self.maps.iter();
        let mut active_map = map_iter.next().expect("No map?");

        // The maps are sorted on creation so we can go through one at a time and apply them to the front of
        // our dynamic ordered ranges.
        while let Some(affected_range) = ordered_ranges.pop_front() {
            let (mapped, unmapped_below, unmapped_above) =
                active_map.apply_to_range(&affected_range);
            if let Some(mapped) = mapped {
                result_ranges.push(mapped);
            }
            if let Some(unmapped_below) = unmapped_below {
                result_ranges.push(unmapped_below);
            }
            if let Some(unmapped_above) = unmapped_above {
                ordered_ranges.push_front(unmapped_above);
                match map_iter.next() {
                    Some(map) => {
                        active_map = map;
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        for range in ordered_ranges {
            result_ranges.push(range);
        }

        result_ranges.sort();
        result_ranges
    }
}

#[derive(Clone, Debug)]
struct Map {
    dest_base: u64,
    range: Range,
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
            range: Range {
                start: values[1],
                length: values[2],
            },
        }
    }
}

impl Map {
    fn apply_to_value(&self, value: u64) -> Option<u64> {
        if value >= self.range.start && value < self.range.start + self.range.length {
            Some(self.dest_base + value - self.range.start)
        } else {
            None
        }
    }

    /// Returns a trio of possible Ranges: the set of mapped numbers,
    /// the set of unchanged numbers below (which can now be ignored)
    /// and the set of unchanged numbers above (which still need to be checked).
    fn apply_to_range(
        &self,
        affected_range: &Range,
    ) -> (Option<Range>, Option<Range>, Option<Range>) {
        // Get range that is mapped and range that isn't - possibly.
        if affected_range.start >= self.range.start {
            // Affected range starts at or above the map
            if affected_range.start > self.range.end_inc() {
                // Affected range is entirely above map range
                (None, None, Some(*affected_range))
            } else {
                // Affected range starts in the map
                if affected_range.end_inc() <= self.range.end_inc() {
                    // Affected range is wholly within the map
                    (
                        Some(Range {
                            start: affected_range.start + self.dest_base - self.range.start,
                            length: affected_range.length,
                        }),
                        None,
                        None,
                    )
                } else {
                    // Affected range extends past the map
                    (
                        Some(Range {
                            start: affected_range.start + self.dest_base - self.range.start,
                            length: self.range.end_inc() + 1 - affected_range.start,
                        }),
                        None,
                        Some(Range {
                            start: self.range.end_inc() + 1,
                            length: affected_range.length
                                - (self.range.end_inc() + 1 - affected_range.start),
                        }),
                    )
                }
            }
        } else {
            // Affected range starts below the map
            if affected_range.end_inc() < self.range.start {
                // Affected range is entirely below the map range
                (None, Some(*affected_range), None)
            } else if affected_range.end_inc() <= self.range.end_inc() {
                // Affected range starts below the map, but ends within it
                (
                    Some(Range {
                        start: self.dest_base,
                        length: affected_range.end_inc() - self.range.start + 1,
                    }),
                    Some(Range {
                        start: affected_range.start,
                        length: self.range.start - affected_range.start,
                    }),
                    None,
                )
            } else {
                // Affected range starts below and ends above
                (
                    Some(Range {
                        start: self.dest_base,
                        length: self.range.length,
                    }),
                    Some(Range {
                        start: affected_range.start,
                        length: self.range.start - affected_range.start,
                    }),
                    Some(Range {
                        start: self.range.end_inc() + 1,
                        length: affected_range.end_inc() - self.range.end_inc(),
                    }),
                )
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn end_inc(&self) -> u64 {
        self.start + self.length - 1
    }
}

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
