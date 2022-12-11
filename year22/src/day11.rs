use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

use crate::utils::split_input_by_blocks;

pub fn day11(input_lines: &str) -> (String, String) {
    let monkey_troop_and_items = split_input_by_blocks(input_lines, monkey_rules);

    let (monkey_troop, throw_factors): (Vec<_>, HashSet<_>) = monkey_troop_and_items
        .iter()
        .map(|(monkey, _)| (monkey.clone(), monkey.rules.divisor))
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();

    // We should either sort the list of monkeys, or at least assert it's already sorted from the input.
    // It is indeed already sorted, but the simple function for confirming that is unstable, so commented out for now.
    // assert!(monkey_troop_and_items.is_sorted_by_key(|m| m.0.id));
    let mut monkey_state = monkey_troop_and_items
        .iter()
        .map(|(monkey, items)| {
            (
                monkey.id,
                MonkeyState {
                    part1: Part1MonkeyState {
                        items: items.clone(),
                        number_inspections: 0,
                    },
                    part2: Part2MonkeyState {
                        items: items
                            .iter()
                            .map(|i| Item::new(i, &throw_factors))
                            .collect::<Vec<Item>>(),
                        number_inspections: 0,
                    },
                },
            )
        })
        .collect::<HashMap<_, _>>();

    // Part 1
    (1..=20).for_each(|_| {
        // Play a round
        monkey_troop.iter().for_each(|active_monkey| {
            // Take a turn
            // Get the active monkey items as a cloned immutable list before starting adding to other lists
            for item in monkey_state
                .get(&active_monkey.id)
                .expect("Monkey state doesn't exist")
                .part1
                .items
                .clone()
            {
                let new_value = active_monkey.inspect(item) / 3;
                monkey_state
                    .get_mut(&active_monkey.throw(new_value))
                    .expect("New monkey state doesn't exist")
                    .part1
                    .items
                    .push(new_value);
            }
            // Monkey will throw every one of the items; get the now safely mutable state and update it.
            let active_monkey_state = monkey_state
                .get_mut(&active_monkey.id)
                .expect("Monkey state doesn't exist");
            active_monkey_state.part1.number_inspections += active_monkey_state.part1.items.len();
            active_monkey_state.part1.items = Vec::new();
        });
    });

    // Attempt at part 2.

    // // For part 2, we could either run the process 10,000 times with the / 3 removed and each item tracked as a vec of 8 modulos...
    // // ... or we could run each individual item until it hits a loop, and then add the appropriate values to each monkey.
    // // assert!(monkey_troop.is_sorted_by_key(|monkey| &monkey.id));
    // let throw_factors = monkey_troop.iter().map(|m| m.rules.divisor).collect::<Vec<_>>();
    // let mut monkey_throws = monkey_troop.iter().map(|m| (m.id, 0)).collect::<HashMap<usize, u64>>();

    // for monkey in &monkey_troop {
    //     for item in &monkey_state_part2.get(&monkey.id).unwrap().items {
    //         let mut item_mods = throw_factors.iter().map(|p| (*p, item % p)).collect::<HashMap<_, _>>();
    //         // Create a hash of the monkeys it passes through and the
    //         let mut rounds_til_loop = 0;
    //         let mut monkey_val_seq: Vec<(usize, u64)> = Vec::new();

    //         let mut current_state = (monkey.id, *item_mods.get(&monkey.rules.divisor).unwrap());
    //         while !monkey_val_seq.contains(&current_state) {
    //             monkey_val_seq.push(current_state.clone());
    //             monkey.full_inspect(&mut item_mods);
    //             let new_holder = monkey.throw(*item_mods.get(&monkey.rules.divisor).unwrap());
    //             current_state = (new_holder, *item_mods.get(&throw_factors[new_holder]).unwrap());

    //             if new_holder < monkey.id {
    //                 // This will need another round to pass on.
    //                 rounds_til_loop += 1;
    //             }
    //         }

    //         // We've hit a loop.  Pretty sure this can only happen at the start of the loop...
    //         if *monkey_val_seq.first().unwrap() != current_state {
    //             println!("Bad state for monkey {}, original item {}, after {} throws", monkey.id, item, monkey_val_seq.len());
    //             panic!();
    //         }
    //     }
    // }

    // Part 2 is up to 10_000.
    (1..=10_000).for_each(|_| {
        monkey_troop.iter().for_each(|active_monkey| {
            // Take a turn
            // Get the active monkey items as a cloned immutable list before starting adding to other lists
            for item in monkey_state
                .get(&active_monkey.id)
                .expect("Monkey state doesn't exist")
                .part2
                .items
                .clone()
            {
                let new_value = active_monkey.full_inspect(item);
                monkey_state
                    .get_mut(
                        &active_monkey.throw(
                            *new_value
                                .panic_mods
                                .get(&active_monkey.rules.divisor)
                                .unwrap(),
                        ),
                    )
                    .expect("New monkey state doesn't exist")
                    .part2
                    .items
                    .push(new_value);
            }
            // Monkey will throw every one of the items; get the now safely mutable state and update it.
            let mut active_monkey_state = monkey_state
                .get_mut(&active_monkey.id)
                .expect("Monkey state doesn't exist");
            active_monkey_state.part2.number_inspections += active_monkey_state.part2.items.len();
            active_monkey_state.part2.items = Vec::new();
        });
    });

    (
        format!(
            "{}",
            determine_monkey_business(
                monkey_state
                    .values()
                    .map(|state| state.part1.number_inspections)
            )
        ),
        format!(
            "{}",
            determine_monkey_business(
                monkey_state
                    .values()
                    .map(|state| state.part2.number_inspections)
            )
        ),
    )
}

fn determine_monkey_business<I>(state: I) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut counts = state.sorted().rev();
    counts.next().unwrap() * counts.next().unwrap()
}

fn monkey_rules(lines: &[&str]) -> (Monkey, Vec<u64>) {
    // Sample text:

    // Monkey 1:
    //   Starting items: 54, 65, 75, 74
    //   Operation: new = old + 6
    //   Test: divisible by 19
    //     If true: throw to monkey 2
    //     If false: throw to monkey 0

    // Simplifying assumption: the inspection is always of the form "new = old <operand> <value OR old>"
    let operation = lines[2]
        .strip_prefix("  Operation: new = old ")
        .expect("Operation line wrong")
        .split_ascii_whitespace()
        .collect::<Vec<_>>();
    assert_eq!(operation.len(), 2);
    let operand = match operation[0] {
        "+" => Operand::Add,
        "*" => Operand::Multiply,
        _ => panic!(),
    };
    let modifier = if operation[1] == "old" {
        ModifierVal::Itself
    } else {
        ModifierVal::Value {
            modifier: operation[1]
                .parse::<u64>()
                .expect("Couldn't parse the modifier value"),
        }
    };

    let rules = ThrowTest {
        divisor: strip_and_parse::<u64>(lines[3], "  Test: divisible by "),
        if_true: strip_and_parse::<usize>(lines[4], "    If true: throw to monkey "),
        if_false: strip_and_parse::<usize>(lines[5], "    If false: throw to monkey "),
    };

    (
        Monkey {
            id: strip_and_parse::<usize>(lines[0].strip_suffix(':').unwrap(), "Monkey "),
            inspection: (operand, modifier),
            rules,
        },
        lines[1]
            .strip_prefix("  Starting items: ")
            .expect("Starting items line wrong")
            .split(", ")
            .map(|items| items.parse::<u64>().expect("Failed to parse an item"))
            .collect::<Vec<_>>(),
    )
}

fn strip_and_parse<T>(text: &str, prefix: &str) -> T
where
    T: FromStr,
{
    text.strip_prefix(prefix)
        .and_then(|s| s.parse::<T>().ok())
        .unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    id: usize,
    inspection: (Operand, ModifierVal),
    rules: ThrowTest,
}

impl Monkey {
    fn inspect(&self, item: u64) -> u64 {
        match self.inspection {
            (Operand::Add, ModifierVal::Value { modifier: m }) => item + m,
            (Operand::Multiply, ModifierVal::Value { modifier: m }) => item * m,
            (Operand::Multiply, ModifierVal::Itself) => item.pow(2),
            _ => panic!(),
        }
    }

    fn full_inspect(&self, item: Item) -> Item {
        Item {
            panic_mods: item
                .panic_mods
                .iter()
                .map(|(divisor, value)| (*divisor, self.inspect(*value) % divisor))
                .collect::<HashMap<_, _>>(),
        }
    }

    fn throw(&self, item: u64) -> usize {
        if item % self.rules.divisor == 0 {
            self.rules.if_true
        } else {
            self.rules.if_false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Operand {
    Add,
    Multiply,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ModifierVal {
    Value { modifier: u64 },
    Itself,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ThrowTest {
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MonkeyState {
    part1: Part1MonkeyState,
    part2: Part2MonkeyState,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Part1MonkeyState {
    items: Vec<u64>,
    number_inspections: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part2MonkeyState {
    items: Vec<Item>,
    number_inspections: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Item {
    panic_mods: HashMap<u64, u64>,
}

impl Item {
    fn new(panic_level: &u64, divisors: &HashSet<u64>) -> Self {
        Self {
            panic_mods: divisors
                .iter()
                .map(|p| (*p, panic_level % p))
                .collect::<HashMap<_, _>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(
            day11(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            ),
            ("10605".to_string(), "2713310158".to_string())
        )
    }
}
