// Potential improvements
// 1: Look to see if we can commonalise Part 2 in this and Day 21 Part 1.
// 2: Make more efficient:
//   a. Might be better to use the is_valid_value() checks rather than 40 HashSet unions of how-large number sets
//   b. See TODOs below: we re-scan the invalid tickets to determine their invalidity score, which is clunky, and
//      we clone tickets we almost certianly don't need to.
// 3: The process of deduction is dependent on there always being at least one fully identifiable rule after each
//    preivous rule elimination, which doesn't feel like it's guaranteed (but probably is for the puzzle). Full
//    sudoku solver would be ideal...

use std::collections::{HashMap, HashSet};

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    // Parse input into InfoFields and Tickets
    let (rules, my_ticket, other_tickets) = parse_input(input_lines);
    let (valid_tickets, scanning_error_rate) = invalid_ticket_scan(&rules, &other_tickets);
    (
        scanning_error_rate,
        day16_part2_calc(&rules, &my_ticket, &valid_tickets),
    )
}

fn invalid_ticket_scan(rules: &[InfoField], other_tickets: &[Ticket]) -> (Vec<Ticket>, u64) {
    // Create HashSet of all valid values across all InfoFields
    let valid_values = all_valid_values(rules);
    let (valid_tickets, invalid_tickets): (Vec<&Ticket>, Vec<&Ticket>) = other_tickets
        .iter()
        .partition(|ticket| ticket.ticket_valid(&valid_values));

    // TODO this is still inefficient: we're scanning every ticket to determine if it's valid
    // then re-scanning the invalid ones for a score on how invalid.  Would be nice to have the
    // scan return an Either of the Ticket or the score, then react appropriately.
    // TODO would also be nice not to have to clone all the tickets in the partition process; would
    // rather just filter them in situ.
    (
        valid_tickets
            .iter()
            .map(|&ticket| ticket.clone())
            .collect::<Vec<Ticket>>(),
        invalid_tickets
            .iter()
            .map(|ticket| ticket.scanning_error(&valid_values))
            .sum(),
    )
}

fn day16_part2_calc(rules: &[InfoField], my_ticket: &Ticket, valid_tickets: &[Ticket]) -> u64 {
    // Create the starting point of all the possible rules for each position.
    let mut ticket_index_to_rules_map: HashMap<usize, HashSet<&InfoField>> = HashMap::new();
    (0..rules.len()).for_each(|pos| {
        let candidate_fields = rules
            .iter()
            .filter(|rule| {
                valid_tickets
                    .iter()
                    .all(|ticket| rule.is_valid_value(ticket.field_data[pos]))
            })
            .collect::<HashSet<&InfoField>>();

        ticket_index_to_rules_map.insert(pos, candidate_fields);
    });

    let mut unprocessed_unique_positions = ticket_index_to_rules_map
        .iter()
        .filter_map(|(index, candidate_rules)| {
            if candidate_rules.len() == 1 {
                Some(*index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    while !unprocessed_unique_positions.is_empty() {
        let new_unique = unprocessed_unique_positions.pop().unwrap();
        let identified_rule_set = ticket_index_to_rules_map
            .get(&new_unique)
            .expect("How isn't this rule set known?")
            .clone();
        assert_eq!(identified_rule_set.len(), 1);
        let identified_rule = identified_rule_set
            .iter().next()
            .expect("How isn't this rule known?");
        (0..rules.len()).for_each(|index| {
            let candidate_rules = ticket_index_to_rules_map
                .get_mut(&index)
                .expect("Must have been a ruleset");
            if candidate_rules.contains(identified_rule) {
                if candidate_rules.len() == 1 {
                    assert_eq!(new_unique, index);
                } else {
                    candidate_rules.remove(identified_rule);
                    if candidate_rules.len() == 1 {
                        unprocessed_unique_positions.push(index);
                    }
                }
            } else {
                assert_ne!(new_unique, index);
            }
        });
    }

    ticket_index_to_rules_map
        .iter()
        .filter_map(|(index, candidate_rules)| {
            assert_eq!(candidate_rules.len(), 1);
            if candidate_rules
                .iter().next()
                .expect("Should have had an element to take")
                .field_name
                .contains("departure")
            {
                Some(my_ticket.field_data[*index] as u64)
            } else {
                None
            }
        })
        .product()
}

fn parse_input(input_lines: &[String]) -> (Vec<InfoField>, Ticket, Vec<Ticket>) {
    let mut state_types: Vec<Vec<String>> = Vec::new();
    let mut lines_in_state: Vec<String> = Vec::new();
    for line in input_lines {
        if line.is_empty() {
            state_types.push(lines_in_state.clone());
            lines_in_state = Vec::new();
        } else {
            lines_in_state.push(line.clone());
        }
    }
    state_types.push(lines_in_state);

    assert_eq!(state_types.len(), 3);
    (
        parse_rules(&state_types[0]),
        parse_my_ticket(&state_types[1]),
        parse_other_tickets(&state_types[2]),
    )
}

fn parse_rules(rules_input: &[String]) -> Vec<InfoField> {
    rules_input
        .iter()
        .map(|rule| InfoField::new(rule))
        .collect::<Vec<InfoField>>()
}

fn parse_my_ticket(my_ticket_input: &[String]) -> Ticket {
    assert_eq!(my_ticket_input.len(), 2);
    assert_eq!(my_ticket_input[0], "your ticket:".to_string());
    Ticket::new(&my_ticket_input[1])
}

fn parse_other_tickets(other_tickets_input: &[String]) -> Vec<Ticket> {
    assert_eq!(other_tickets_input[0], "nearby tickets:".to_string());
    other_tickets_input[1..]
        .iter()
        .map(|ticket_info| Ticket::new(ticket_info))
        .collect::<Vec<Ticket>>()
}

fn all_valid_values(rules: &[InfoField]) -> HashSet<usize> {
    rules.iter().fold(HashSet::new(), |collection, rule| {
        collection
            .union(&rule.valid_values())
            .cloned()
            .collect::<HashSet<usize>>()
    })
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct InfoField {
    field_name: String,
    low_range: FieldRange,
    high_range: FieldRange,
}
impl InfoField {
    fn valid_values(&self) -> HashSet<usize> {
        self.low_range
            .valid_values()
            .union(&self.high_range.valid_values())
            .cloned()
            .collect::<HashSet<usize>>()
    }

    fn is_valid_value(&self, test_value: usize) -> bool {
        self.low_range.is_in_range(test_value) || self.high_range.is_in_range(test_value)
    }

    fn new(rule_text: &str) -> InfoField {
        let field_name = rule_text
            .split(": ")
            .next()
            .expect("Didn't find : in rule")
            .to_string();
        let ranges = rule_text.split(": ").nth(1).unwrap();
        let low_range =
            FieldRange::new(ranges.split(" or ").next().expect("Didn't find or in rule"));
        let high_range =
            FieldRange::new(ranges.split(" or ").nth(1).expect("Didn't find or in rule"));
        InfoField {
            field_name,
            low_range,
            high_range,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct FieldRange {
    lower: usize,
    upper: usize,
}
impl FieldRange {
    fn valid_values(&self) -> HashSet<usize> {
        (self.lower..self.upper + 1).collect::<HashSet<usize>>()
    }

    fn is_in_range(&self, test_value: usize) -> bool {
        test_value >= self.lower && test_value <= self.upper
    }

    fn new(range: &str) -> FieldRange {
        let lower = range
            .split('-')
            .next()
            .expect("Didn't find - in range")
            .parse::<usize>()
            .expect("Couldn't parse range value");
        let upper = range
            .split('-')
            .nth(1)
            .expect("Didn't find - in range")
            .parse::<usize>()
            .expect("Couldn't parse range value");
        FieldRange { lower, upper }
    }
}

#[derive(Clone)]
struct Ticket {
    field_data: Vec<usize>,
}
impl Ticket {
    fn scanning_error(&self, valid_values: &HashSet<usize>) -> u64 {
        self.field_data
            .iter()
            .filter(|val| !valid_values.contains(val))
            .sum::<usize>() as u64
    }

    fn ticket_valid(&self, valid_values: &HashSet<usize>) -> bool {
        self.field_data.iter().all(|val| valid_values.contains(val))
    }

    fn new(ticket_text: &str) -> Ticket {
        Ticket {
            field_data: ticket_text
                .split(',')
                .map(|s| s.parse::<usize>().expect("Couldn't parse"))
                .collect::<Vec<usize>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{invalid_ticket_scan, FieldRange, InfoField, Ticket};

    // #[test]
    //     fn day16_example1() {
    //         let example = vec!["class: 1-3 or 5-7
    // row: 6-11 or 33-44
    // seat: 13-40 or 45-50

    // your ticket:
    // 7,1,14

    // nearby tickets:
    // 7,3,47
    // 40,4,50
    // 55,2,20
    // 38,6,12"
    //             .to_string()];
    //         assert_eq!(day16(&example).0, 71);
    //     }

    //     #[test]
    //     fn day16_example2() {
    //         let example = vec![
    // "class: 0-1 or 4-19
    // row: 0-5 or 8-19
    // seat: 0-13 or 16-19

    // your ticket:
    // 11,12,13

    // nearby tickets:
    // 3,9,18
    // 15,1,5
    // 5,14,9".to_string()];
    //         // assert_eq!(day16(&example).0, 71);
    //     }
    #[test]
    fn postparsed_part1() {
        let rules = vec![
            InfoField {
                field_name: "class".to_string(),
                low_range: FieldRange { lower: 1, upper: 3 },
                high_range: FieldRange { lower: 5, upper: 7 },
            },
            InfoField {
                field_name: "row".to_string(),
                low_range: FieldRange {
                    lower: 6,
                    upper: 11,
                },
                high_range: FieldRange {
                    lower: 33,
                    upper: 44,
                },
            },
            InfoField {
                field_name: "seat".to_string(),
                low_range: FieldRange {
                    lower: 13,
                    upper: 40,
                },
                high_range: FieldRange {
                    lower: 45,
                    upper: 50,
                },
            },
        ];
        let other_tickets = vec![
            Ticket {
                field_data: vec![7, 3, 47],
            },
            Ticket {
                field_data: vec![40, 4, 50],
            },
            Ticket {
                field_data: vec![55, 2, 20],
            },
            Ticket {
                field_data: vec![38, 6, 12],
            },
        ];

        assert_eq!(invalid_ticket_scan(&rules, &other_tickets).1, 71);
    }
}

// Partial hand-solution (until I found the six fields I needed)
// [00]: train
// [01]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival platform/duration/price/seat/train/wagon/zone/
// [02]: duration
// [03]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival platform/duration/price/route/seat/train/type/wagon/zone/
// [04]: arrival location
// [05]: departure station
// [06]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival platform/class/duration/price/route/row/seat/train/type/wagon/zone/
// [07]: price
// [08]: departure location
// [09]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival platform/duration/price/route/row/seat/train/type/wagon/zone/
// [10]: departure time
// [11]: zone
// [12]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival station/arrival platform/arrival track/class/duration/price/route/row/seat/train/type/wagon/zone/
// [13]: departure platform
// [14]: seat
// [15]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival platform/duration/price/seat/train/type/wagon/zone/
// [16]: departure track
// [17]: departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/arrival platform/arrival track/class/duration/price/route/row/seat/train/type/wagon/zone/
// [18]: departure time
// [19]: wagon

// departure location/departure station/departure platform/departure track/departure date/departure time/arrival location/duration/price/seat/train/wagon/zone/
