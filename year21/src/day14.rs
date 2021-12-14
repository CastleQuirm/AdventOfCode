use std::collections::HashMap;

// Potential improvements:
//

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    let mut polymer = (&input_lines[0]).to_string();
    let rules = input_lines[2..input_lines.len()]
        .iter()
        .map(|line| {
            let mut split_line = line.split(" -> ");
            let pair = split_line.next().expect("Couldn't find a pair").to_string();
            let insertion = split_line
                .next()
                .expect("Couldn't find the target")
                .to_string();
            let new_pair = pair
                .chars()
                .nth(0)
                .expect("No first character in pair")
                .to_string()
                + &insertion;
            (pair, new_pair)
        })
        .collect::<HashMap<String, String>>();

    (0..40).for_each(|_| polymer = extend_polymer(&polymer, &rules));

    let mut element_counts: HashMap<char, u64> = HashMap::new();
    polymer.chars().for_each(|c| {
        let old_count = *element_counts.get(&c).unwrap_or(&0);
        element_counts.insert(c, old_count + 1);
    });

    let mut element_counts = element_counts.values().map(|&v| v).collect::<Vec<u64>>();
    element_counts.sort_unstable();
    // let most_element = *element_counts.last().expect("No most element");
    // let least_element = *element_counts.first().expect("No most element");
    // let part1 = most_element;

    let part1 = *element_counts.last().expect("No most element")
        - *element_counts.first().expect("No least element");

    (part1, 0)
}

fn extend_polymer(old_polymer: &String, rules: &HashMap<String, String>) -> String {
    (0..old_polymer.len())
        .map(|i| {
            if i < old_polymer.len() - 1 {
                rules
                    .get(&old_polymer[i..i + 2])
                    .expect("Didn't find the pair")
                    .to_string()
            } else {
                old_polymer
                    .chars()
                    .last()
                    .expect("No last character")
                    .to_string()
            }
        })
        .collect::<String>()
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
