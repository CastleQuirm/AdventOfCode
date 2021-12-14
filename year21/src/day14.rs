use std::collections::HashMap;

// Potential improvements:
//

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    let mut polymer = Polymer::new(&input_lines[0]);
    let insertion_map = InsertionMap::new(&input_lines[2..]);
    (0..10).for_each(|_| polymer = polymer.grow(&insertion_map));
    let part1 = polymer.result_size();
    (10..40).for_each(|_| polymer = polymer.grow(&insertion_map));
    let part2 = polymer.result_size();

    (part1, part2)
}

struct Polymer {
    bond_counts: HashMap<Bond, u64>,
    last_char: char,
}
impl Polymer {
    fn new(line: &str) -> Self {
        let mut bond_counts: HashMap<Bond, u64> = HashMap::new();
        for i in 0..line.len() - 1 {
            let bond = Bond {
                first_char: line.chars().nth(i).expect("No ith character"),
                second_char: line.chars().nth(i + 1).expect("no i+1th character"),
            };
            let current_count = *bond_counts.get(&bond).unwrap_or(&0);
            bond_counts.insert(bond, current_count + 1);
        }

        let last_char = line.chars().last().expect("no last char");
        Polymer {
            bond_counts,
            last_char,
        }
    }

    fn grow(&self, map: &InsertionMap) -> Self {
        let mut new_bond_counts: HashMap<Bond, u64> = HashMap::new();
        for (bond, instances) in &self.bond_counts {
            let (new_bond_1, new_bond_2) = map.map.get(bond).expect("Didn't find a bond mapping");
            let current_count = *new_bond_counts.get(new_bond_1).unwrap_or(&0);
            new_bond_counts.insert(*new_bond_1, current_count + instances);
            let current_count = *new_bond_counts.get(new_bond_2).unwrap_or(&0);
            new_bond_counts.insert(*new_bond_2, current_count + instances);
        }

        Polymer {
            bond_counts: new_bond_counts,
            last_char: self.last_char,
        }
    }

    fn result_size(&self) -> u64 {
        let mut element_counts: HashMap<char, u64> = HashMap::new();
        for (bond, number) in &self.bond_counts {
            let current_count = *element_counts.get(&bond.first_char).unwrap_or(&0);
            element_counts.insert(bond.first_char, current_count + number);
        }

        let current_count = *element_counts.get(&self.last_char).unwrap_or(&0);
        element_counts.insert(self.last_char, current_count + 1);

        let max_element = element_counts.values().max().expect("No max element");
        let min_element = element_counts.values().min().expect("No min element");

        max_element - min_element
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Bond {
    first_char: char,
    second_char: char,
}

struct InsertionMap {
    map: HashMap<Bond, (Bond, Bond)>,
}
impl InsertionMap {
    fn new(input_lines: &[String]) -> Self {
        let map = input_lines
            .iter()
            .map(|line| {
                let mut split_line = line.split(" -> ");
                let mut original = split_line.next().expect("No from bond").chars();
                let original_first = original.next().expect("no first char");
                let original_second = original.next().expect("no second char");
                let new_element = split_line
                    .next()
                    .expect("No to element")
                    .chars()
                    .next()
                    .expect("No first element in target");
                let original_bond = Bond {
                    first_char: original_first,
                    second_char: original_second,
                };
                let first_new = Bond {
                    first_char: original_first,
                    second_char: new_element,
                };
                let second_new = Bond {
                    first_char: new_element,
                    second_char: original_second,
                };
                (original_bond, (first_new, second_new))
            })
            .collect::<HashMap<Bond, (Bond, Bond)>>();
        InsertionMap { map }
    }
}

#[cfg(test)]
mod tests {
    use super::day14;

    #[test]
    fn check_day14() {
        let input_lines = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day14(&input_lines), (1588, 2188189693529));
    }
}

// Original naive implementation:

// pub fn day14(input_lines: &[String]) -> (u64, u64) {
//     let mut polymer = (&input_lines[0]).to_string();
//     let rules = input_lines[2..input_lines.len()]
//         .iter()
//         .map(|line| {
//             let mut split_line = line.split(" -> ");
//             let pair = split_line.next().expect("Couldn't find a pair").to_string();
//             let insertion = split_line
//                 .next()
//                 .expect("Couldn't find the target")
//                 .to_string();
//             let new_pair = pair
//                 .chars()
//                 .nth(0)
//                 .expect("No first character in pair")
//                 .to_string()
//                 + &insertion;
//             (pair, new_pair)
//         })
//         .collect::<HashMap<String, String>>();

//     (0..40).for_each(|_| polymer = extend_polymer(&polymer, &rules));

//     let mut element_counts: HashMap<char, u64> = HashMap::new();
//     polymer.chars().for_each(|c| {
//         let old_count = *element_counts.get(&c).unwrap_or(&0);
//         element_counts.insert(c, old_count + 1);
//     });

//     let mut element_counts = element_counts.values().map(|&v| v).collect::<Vec<u64>>();
//     element_counts.sort_unstable();
//     // let most_element = *element_counts.last().expect("No most element");
//     // let least_element = *element_counts.first().expect("No most element");
//     // let part1 = most_element;

//     let part1 = *element_counts.last().expect("No most element")
//         - *element_counts.first().expect("No least element");

//     (part1, 0)
// }

// fn extend_polymer(old_polymer: &String, rules: &HashMap<String, String>) -> String {
//     (0..old_polymer.len())
//         .map(|i| {
//             if i < old_polymer.len() - 1 {
//                 rules
//                     .get(&old_polymer[i..i + 2])
//                     .expect("Didn't find the pair")
//                     .to_string()
//             } else {
//                 old_polymer
//                     .chars()
//                     .last()
//                     .expect("No last character")
//                     .to_string()
//             }
//         })
//         .collect::<String>()
// }
