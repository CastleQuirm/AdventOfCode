use std::collections::{HashMap, HashSet};

// Potential improvements:
// 1. Lots of code commonisation, in particular:
//    - the repeated lines in determine_map for the six more complicated numbers
//    - the various "alphabetization" functions
// 2. Can we do something better than lots of hash intersections?
// 3. Can we do better for the third of the complicated numbers (each time) in just picking the one of the three we haven't yet picked?
// 4. Naming throughout is a nightmare.

pub fn day08(input_lines: &[String]) -> (u64, u64) {
    // Easy part 1 approach.  We could derive it from the info we get from part 2 but...why, when it's so simple to do direct?
    let part1 = input_lines
        .iter()
        .map(|line| {
            line.split(' ')
                .filter(|x| x.len() != 5 && x.len() != 6)
                .count()
                - 5 // This allows for the four digits we care about appearing in the info section, plus the dividing bar.
        })
        .sum::<usize>() as u64;

    let part2 = input_lines.iter().map(|line| determine_value(line)).sum();

    (part1, part2)
}

fn determine_value(line: &str) -> u64 {
    let line_sections = line.split(" | ").collect::<Vec<&str>>();

    let info_digits = line_sections
        .first()
        .expect("No info!")
        .split(' ')
        .map(|digit| digit.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>();
    let digit_map = determine_map(info_digits);

    let display_val = line_sections
        .last()
        .expect("No display val")
        .split(' ')
        .map(alphabetize_string)
        .collect::<Vec<_>>();

    display_val
        .iter()
        .map(|alphabetic_digit| {
            *digit_map
                .get(alphabetic_digit)
                .expect("Didn't find the digit")
        })
        .fold(0, |number, acc| number * 10 + acc)
}

fn determine_map(info_digits: Vec<HashSet<char>>) -> HashMap<String, u64> {
    let mut segment_map: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();

    info_digits.iter().for_each(|single_digit| {
        let length = single_digit.len();
        segment_map
            .entry(length)
            .or_default()
            .push(single_digit.clone());
    });

    let mut digit_map: HashMap<String, u64> = HashMap::new();
    insert_single_entry(&mut digit_map, &segment_map, &2, 1);
    insert_single_entry(&mut digit_map, &segment_map, &3, 7);
    insert_single_entry(&mut digit_map, &segment_map, &4, 4);
    insert_single_entry(&mut digit_map, &segment_map, &7, 8);

    let one_hash = segment_map
        .get(&2)
        .expect("Didn't find 1")
        .first()
        .expect("More than one two-string");
    let four_hash = segment_map
        .get(&4)
        .expect("Didn't find 4")
        .first()
        .expect("More than one two-string");

    // 5 length: 2, 3, 5
    let five_char_digits = segment_map.get(&5).expect("No 5-character digits");
    // Intersect with 1: 3 has length 2, 2/5 have length 1
    let three_string = five_char_digits
        .iter()
        .find(|&digit| digit.intersection(one_hash).count() == 2)
        .expect("Couldn't find a candidate 3");
    digit_map.insert(hash_set_to_alphabetical_string(three_string), 3);
    // Intersect with 4: 2 has length 2, 3/5 have length 3
    let two_string = five_char_digits
        .iter()
        .find(|&digit| digit.intersection(four_hash).count() == 2)
        .expect("Couldn't find a candidate 2");
    digit_map.insert(hash_set_to_alphabetical_string(two_string), 2);

    let five_string = five_char_digits
        .iter()
        .find(|&digit| {
            digit.intersection(one_hash).count() == 1 && digit.intersection(four_hash).count() == 3
        })
        .expect("Couldn't find a candidate 5");
    digit_map.insert(hash_set_to_alphabetical_string(five_string), 5);

    // 6 length: 0, 6, 9
    let six_char_digits = segment_map.get(&6).expect("No 6-character digits");
    // Intersect with 1: 0/9 have length 2, 6 has length 1
    let six_string = six_char_digits
        .iter()
        .find(|&digit| digit.intersection(one_hash).count() == 1)
        .expect("Couldn't find a candidate 6");
    digit_map.insert(hash_set_to_alphabetical_string(six_string), 6);
    // Intersect with 4: 0/6 have length 3, 9 has length 4
    let nine_string = six_char_digits
        .iter()
        .find(|&digit| digit.intersection(four_hash).count() == 4)
        .expect("Couldn't find a candidate 9");
    digit_map.insert(hash_set_to_alphabetical_string(nine_string), 9);
    let zero_string = six_char_digits
        .iter()
        .find(|&digit| {
            digit.intersection(one_hash).count() == 2 && digit.intersection(four_hash).count() == 3
        })
        .expect("Couldn't find a candidate 0");
    digit_map.insert(hash_set_to_alphabetical_string(zero_string), 0);

    digit_map
}

fn insert_single_entry(
    digit_map: &mut HashMap<String, u64>,
    seg_map: &HashMap<usize, Vec<HashSet<char>>>,
    len: &usize,
    digit: u64,
) {
    let mut char_vec = seg_map
        .get(len)
        .unwrap()
        .first()
        .unwrap()
        .iter()
        .copied()
        .collect::<Vec<char>>();
    char_vec.sort_unstable();
    let alphabetic_string = char_vec.iter().fold("".to_string(), |mut str, &c| {
        str.push(c);
        str
    });
    digit_map.insert(alphabetic_string, digit);
}

fn alphabetize_string(input: &str) -> String {
    let mut char_vec = input.chars().collect::<Vec<char>>();
    char_vec.sort_unstable();
    char_vec.iter().fold("".to_string(), |mut str, &c| {
        str.push(c);
        str
    })
}

fn hash_set_to_alphabetical_string(input: &HashSet<char>) -> String {
    let mut char_vec = input.iter().copied().collect::<Vec<char>>();
    char_vec.sort_unstable();
    char_vec.iter().fold("".to_string(), |mut str, &c| {
        str.push(c);
        str
    })
}

#[cfg(test)]
mod tests {
    use super::day08;

    #[test]
    fn check_day08() {
        let input_lines =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
                .lines()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
        assert_eq!(day08(&input_lines), (26, 61229));
    }
}
