use std::collections::HashMap;

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    (
        calc(input_lines, part1_mem_action),
        calc(input_lines, part2_mem_action),
    )
}

fn calc(input_lines: &[String], calc_func: fn(&mut HashMap<u64, u64>, &str, &mut Bitmask)) -> u64 {
    let mut mask: Bitmask = Bitmask {
        mask: "".to_string(),
    };
    let mut records: HashMap<u64, u64> = HashMap::new();
    input_lines.iter().for_each(|line| match &line[0..3] {
        "mas" => {
            mask = Bitmask {
                mask: line[7..].to_string(),
            }
        }
        "mem" => {
            calc_func(&mut records, line, &mut mask);
        }
        _ => unreachable!(),
    });
    records.values().sum()
}

fn part1_mem_action(records: &mut HashMap<u64, u64>, line: &str, mask: &mut Bitmask) {
    records.insert(
        get_address_from_line(line),
        mask.apply_v1_mask(get_saved_val_from_line(line)),
    );
}

fn part2_mem_action(records: &mut HashMap<u64, u64>, line: &str, mask: &mut Bitmask) {
    let masked_address = mask.mask_address(get_address_from_line(line)); // String including Xs.
    let mut defined_vals: Vec<String> = vec![masked_address.clone()];
    (0..masked_address.len())
        .filter(|&i| masked_address.chars().nth(i).unwrap() == 'X')
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
    defined_vals.iter().for_each(|val| {
        records.insert(
            isize::from_str_radix(val, 2).unwrap() as u64,
            get_saved_val_from_line(line),
        );
    });
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

fn get_address_from_line(line: &str) -> u64 {
    line[4..]
        .split(']')
        .next()
        .expect("Didn't recognise the memory location")
        .parse::<u64>()
        .expect("Couldn't parse the address")
}

fn get_saved_val_from_line(line: &str) -> u64 {
    line.split(' ')
        .next_back()
        .expect("Didn't find a value to store")
        .parse::<u64>()
        .expect("Couldn't parse the value to store")
}

struct Bitmask {
    mask: String,
}
impl Bitmask {
    fn apply_v1_mask(&self, store_num: u64) -> u64 {
        (store_num & self.and_mask()) | self.or_mask()
    }
    fn and_mask(&self) -> u64 {
        isize::from_str_radix(&self.mask.replace('X', "1"), 2).unwrap() as u64
    }
    fn or_mask(&self) -> u64 {
        isize::from_str_radix(&self.mask.replace('X', "0"), 2).unwrap() as u64
    }
    fn mask_address(&self, address: u64) -> String {
        (0..self.mask.len())
            .map(|i| {
                if self.mask.chars().nth(i).unwrap() == '0' {
                    if ((address >> (35 - i)) % 2) == 1 {
                        '1'
                    } else {
                        '0'
                    }
                } else {
                    self.mask.chars().nth(i).unwrap()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{calc, part1_mem_action, part2_mem_action};

    #[test]
    fn part1_example() {
        let sample_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(calc(&sample_input, part1_mem_action), 165);
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
        assert_eq!(calc(&sample_input, part2_mem_action), 208);
    }
}
