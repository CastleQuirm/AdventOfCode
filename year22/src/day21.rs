use std::collections::HashMap;

pub fn day21(input_lines: &str) -> (String, String) {
    let mut declared_monkeys = HashMap::new();
    let mut pending_monkeys = HashMap::new();
    let mut lookup_monkeys = HashMap::new();
    for line in input_lines.lines() {
        let line_parts = line
            .split(": ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(line_parts.len(), 2);
        let (monkey_name, shout) = (line_parts[0].clone(), line_parts[1].clone());
        if let Ok(value) = shout.parse::<i64>() {
            assert!(declared_monkeys.insert(monkey_name, value).is_none());
        } else {
            assert!(pending_monkeys
                .insert(monkey_name.clone(), shout.clone())
                .is_none());
            let conditional_monkeys = shout
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            assert_eq!(conditional_monkeys.len(), 3);
            let precursor_monkey: &mut Vec<String> = lookup_monkeys
                .entry(conditional_monkeys[0].clone())
                .or_default();
            precursor_monkey.push(monkey_name.clone());
            let precursor_monkey: &mut Vec<String> = lookup_monkeys
                .entry(conditional_monkeys[2].clone())
                .or_default();
            precursor_monkey.push(monkey_name.clone());
        }
    }
    // println!("Declared monkeys: {:?}", declared_monkeys);
    // println!("Pending monkeys: {:?}", pending_monkeys);
    // println!("Lookup monkeys: {:?}", lookup_monkeys);

    let mut declared_minus_human = declared_monkeys.clone();
    declared_minus_human.remove("humn");
    // let mut starting_pending = pending_monkeys.clone();
    // let mut starting_lookup = lookup_monkeys.clone();

    solve_forwards(&mut declared_monkeys, &pending_monkeys, &lookup_monkeys);
    let answer1 = declared_monkeys.get("root").expect("Haven't learnt root");

    solve_forwards(&mut declared_minus_human, &pending_monkeys, &lookup_monkeys);
    assert!(!declared_minus_human.contains_key("root"));
    let root_dependants = pending_monkeys
        .get("root")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    assert_eq!(root_dependants.len(), 3);
    let (mut target_value, mut prev_name) = match (
        declared_minus_human.get(&root_dependants[0]),
        declared_minus_human.get(&root_dependants[2]),
    ) {
        (Some(val), None) => (*val, root_dependants[2].clone()),
        (None, Some(val)) => (*val, root_dependants[0].clone()),
        _ => unreachable!(),
    };

    while prev_name != "humn" {
        let resolving_shout = pending_monkeys
            .get(&prev_name)
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(resolving_shout.len(), 3);
        let (known_val, unknown_name, position) = match (
            declared_minus_human.get(&resolving_shout[0]),
            declared_minus_human.get(&resolving_shout[2]),
        ) {
            (Some(val), None) => (*val, resolving_shout[2].clone(), Position::First),
            (None, Some(val)) => (*val, resolving_shout[0].clone(), Position::Second),
            _ => unreachable!(),
        };
        prev_name = unknown_name;
        target_value = match (resolving_shout[1].as_ref(), position) {
            ("+", _) => target_value - known_val, // a = b + x | a = x + b => x = a - b
            ("-", Position::First) => known_val - target_value, // a = b - x => x = b - a
            ("-", Position::Second) => target_value + known_val, // a = x - b => x = a + b
            ("*", _) => target_value / known_val, // a = b * x | a = x * b => x = a / b
            ("/", Position::First) => known_val / target_value, // a = b / x => x = b / a
            ("/", Position::Second) => target_value * known_val, // a = x / b => x = a * b
            _ => unreachable!(),
        }
    }

    let answer2 = target_value;
    (format!("{}", answer1), format!("{}", answer2))
}

enum Position {
    First,
    Second,
}

fn solve_forwards(
    declared_monkeys: &mut HashMap<String, i64>,
    pending_monkeys: &HashMap<String, String>,
    lookup_monkeys: &HashMap<String, Vec<String>>,
) {
    let mut new_lookups = declared_monkeys
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    while !new_lookups.is_empty() {
        let mut newly_learnt = Vec::new();
        for known_monkey in &new_lookups {
            if let Some(partially_resolved) = lookup_monkeys.get(known_monkey) {
                for candidate_resolved in partially_resolved {
                    if let Some(shout) = pending_monkeys.get(candidate_resolved) {
                        let conditional_monkeys = shout
                            .split_ascii_whitespace()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        assert_eq!(conditional_monkeys.len(), 3);
                        assert!(
                            conditional_monkeys[0] == *known_monkey
                                || conditional_monkeys[2] == *known_monkey
                        );
                        if let (Some(monkey1_val), Some(monkey2_val)) = (
                            declared_monkeys.get(&conditional_monkeys[0]),
                            declared_monkeys.get(&conditional_monkeys[2]),
                        ) {
                            let new_val = match conditional_monkeys[1].as_ref() {
                                "+" => monkey1_val + monkey2_val,
                                "-" => monkey1_val - monkey2_val,
                                "*" => monkey1_val * monkey2_val,
                                "/" => monkey1_val / monkey2_val,
                                _ => unreachable!(),
                            };
                            // assert!(declared_monkeys.insert(candidate_resolved.clone(), new_val).is_none());
                            declared_monkeys.insert(candidate_resolved.clone(), new_val);
                            newly_learnt.push(candidate_resolved.clone());
                        }
                    }
                }
            }
        }
        new_lookups = newly_learnt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_day21_part1_case1() {
    //     assert_eq!(day21("").0, "0".to_string())
    // }

    // #[test]
    // fn check_day21_part2_case1() {
    //     assert_eq!(day21("").1, "0".to_string())
    // }

    #[test]
    fn check_day21_both_case1() {
        // assert_eq!(day21("dbpl: 5"), ("152".to_string(), "0".to_string()));

        assert_eq!(
            day21(
                "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            ),
            ("152".to_string(), "301".to_string())
        )
    }
}
