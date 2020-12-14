use std::collections::HashMap;

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    (part1_calc(input_lines), part2_calc(input_lines))
}

fn part1_calc(input_lines: &[String]) -> u64 {
    let mut mask: Bitmask = Bitmask {
        mask: "".to_string(),
        possible_vals: Vec::new(),
    };
    let mut records: HashMap<u64, u64> = HashMap::new();
    input_lines.iter().for_each(|line| match &line[0..3] {
        "mas" => {
            mask = Bitmask {
                mask: line[7..].to_string(),
                possible_vals: Vec::new(),
            }
        }
        "mem" => {
            records.insert(
                line[4..]
                    .split(']')
                    .next()
                    .expect("Didn't recognise the memory location")
                    .parse::<u64>()
                    .expect("Couldn't recognise number for location"),
                mask.apply_v1_mask(
                    line.split(' ')
                        .next_back()
                        .expect("Didn't find a value to store")
                        .parse::<u64>()
                        .expect("Couldn't parse the value to store"),
                ),
            );
        }
        _ => unreachable!(),
    });
    records.values().sum()
}

fn part2_calc(input_lines: &[String]) -> u64 {
    let mut mask: Bitmask = Bitmask {
        mask: "".to_string(),
        possible_vals: Vec::new(),
    };
    let mut records: HashMap<u64, u64> = HashMap::new();
    input_lines.iter().for_each(|line| match &line[0..3] {
        "mas" => {
            mask = Bitmask {
                mask: line[7..].to_string(),
                possible_vals: Vec::new(),
            };
            // This doesn't work.  We need to keep in with the X values so that we know the 0s in order to or them, while overwritting the Xs.
            mask.generate_new_masks();
        }
        "mem" => {
            let stored_value = line
                .split(' ')
                .next_back()
                .expect("Didn't find a value to store")
                .parse::<u64>()
                .expect("Couldn't parse the value to store");
            // Need to map this in some other way as well?
            mask.possible_vals.iter().for_each(|&location| {
                records.insert(location, stored_value);
            })
        }
        _ => unreachable!(),
    });
    records.values().sum()
}

fn replace_x(val: &str, index: usize, replacement: char) -> String {
    (0..val.len())
        .map(|i| {
            if i == index {
                replacement
            } else {
                val.chars().nth(i).unwrap()
            }
        })
        .collect()
}

struct Bitmask {
    mask: String,
    possible_vals: Vec<u64>,
}
impl Bitmask {
    fn apply_v1_mask(&self, store_num: u64) -> u64 {
        (store_num & self.and_mask()) | self.or_mask()
    }
    fn and_mask(&self) -> u64 {
        isize::from_str_radix(&self.mask.replace("X", "1"), 2).unwrap() as u64
    }
    fn or_mask(&self) -> u64 {
        isize::from_str_radix(&self.mask.replace("X", "0"), 2).unwrap() as u64
    }
    fn generate_new_masks(&mut self) {
        self.possible_vals = Vec::new();
        let mut defined_vals: Vec<String> = vec![self.mask.clone()];
        (0..self.mask.len())
            .filter(|&i| self.mask.chars().nth(i).unwrap() == 'X')
            .for_each(|i| {
                let mut replace_0: Vec<String> = defined_vals
                    .iter()
                    .map(|val| replace_x(val, i, '0'))
                    .collect::<Vec<String>>();
                let mut replace_1: Vec<String> = defined_vals
                    .iter()
                    .map(|val| replace_x(val, i, '1'))
                    .collect::<Vec<String>>();
                replace_0.append(&mut replace_1);
                defined_vals = replace_0;
            });

        self.possible_vals = defined_vals
            .iter()
            .map(|val| isize::from_str_radix(val, 2).unwrap() as u64)
            .collect::<Vec<u64>>();
        self.possible_vals.iter().for_each(|val| {
            println!("Will write to mem {}", val);
        });
        println!("...Based on mask {}", self.mask);
    }

    // fn apply_v2_mask(&self, _mem_num: u64) -> Vec<u64> {

    // }
}

#[cfg(test)]
mod tests {
    use super::{part1_calc, part2_calc};

    #[test]
    fn part1_example() {
        let sample_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(part1_calc(&sample_input), 165);
    }

    #[test]
    fn part2_example() {
        let sample_input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(part2_calc(&sample_input), 208);
    }
}
