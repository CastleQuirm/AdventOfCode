// Potential improvements:
//

use std::collections::HashMap;

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let seeds = input_lines[0][0].strip_prefix("seeds: ").expect("Bad start").split_whitespace().map(|seed| seed.parse::<u32>().expect("Didn't parse a seed"));

    let conversion_tables = input_lines[1..].iter().map(|block| ConversionCode::new(&block)).collect::<HashMap<String, ConversionCode>>();

    // let answer1 = seeds.map(|seed| conversion_tables.apply());
    let answer1 = 0;
    let answer2 = 0;
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
                to_type: to_type.strip_suffix(" map:").expect("Title didn't end as expected").to_string(),
                maps,
            }
        )
    }

    fn apply(&self, value: u32) -> u32 {
        self.maps.iter().find_map(|map| map.apply(value)).unwrap_or(value)
    }
}

struct Map {
    dest_base: u32,
    source_base: u32,
    range: u32
}

impl From<&String> for Map {
    fn from(value: &String) -> Self {
        todo!()
    }
}

impl Map {
    fn apply(&self, value: u32) -> Option<u32> {
        if value >= self.source_base && value < self.source_base + self.range {
            Some(self.dest_base + value - self.source_base)
        } else {
            None
        }
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
56 93 4",  // INPUT STRING
            "35", // PART 1 RESULT
            "0", // PART 2 RESULT
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
