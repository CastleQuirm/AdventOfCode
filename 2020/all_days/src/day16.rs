use std::collections::HashSet;

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    // Parse input into InfoFields and Tickets
    let (rules, my_ticket, other_tickets) = parse_input(input_lines);

    // Filter (other) Tickts based on if their fields are all valid in the set of InfoFields.
    (
        day16_part1_calc(&rules, &other_tickets),
        0
    )
}

fn day16_part1_calc(rules: &[InfoField], other_tickets: &[Ticket]) -> u64 {
    // Create HashSet of all valid values across all InfoFields
    let valid_values = all_valid_values(&rules);
    other_tickets.iter().map(|ticket| ticket.scanning_error(&valid_values)).sum()
}

fn parse_input(input_lines: &[String]) -> (Vec<InfoField>, Ticket, Vec<Ticket>) {
    (vec![], Ticket{ field_data: vec![] }, vec![] )
}

fn all_valid_values(rules: &[InfoField]) -> HashSet::<usize> {
    rules.iter().fold(HashSet::new(), |collection, rule| collection.union(&rule.valid_values()).cloned().collect::<HashSet<usize>>())
}

struct InfoField {
    field_name: String,
    low_range: FieldRange,
    high_range: FieldRange
}
impl InfoField {
    fn valid_values(&self) -> HashSet::<usize> {
        self.low_range.valid_values().union(&self.high_range.valid_values()).cloned().collect::<HashSet<usize>>()
    }
}

struct FieldRange {
    lower: usize,
    upper: usize
}
impl FieldRange {
    fn valid_values(&self) -> HashSet::<usize> {
        (self.lower..self.upper+1).collect::<HashSet<usize>>()
    }
}

struct Ticket {
    field_data: Vec<usize>
}
impl Ticket {
    fn scanning_error(&self, valid_values: &HashSet<usize>) -> u64 {
        self.field_data.iter().filter(|val| !valid_values.contains(val)).sum::<usize>() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::{day16, InfoField, FieldRange, Ticket, day16_part1_calc};

    #[test]
    fn day16_example() {
        let example = 
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day16(&example), (71, 0));
    }

    #[test]
    fn postparsed_part1() {
        let rules = vec![
            InfoField{
                field_name: "class".to_string(),
                low_range: FieldRange {
                    lower: 1,
                    upper: 3
                },
                high_range: FieldRange {
                    lower: 5,
                    upper: 7
                }
            },
            InfoField{
                field_name: "row".to_string(),
                low_range: FieldRange {
                    lower: 6,
                    upper: 11
                },
                high_range: FieldRange {
                    lower: 33,
                    upper: 44
                }
            },
            InfoField{
                field_name: "seat".to_string(),
                low_range: FieldRange {
                    lower: 13,
                    upper: 40
                },
                high_range: FieldRange {
                    lower: 45,
                    upper: 50
                }
            },
        ];
        let other_tickets = vec![
            Ticket {
                field_data: vec![7,3,47]
            },
            Ticket {
                field_data: vec![40,4,50]
            },
            Ticket {
                field_data: vec![55,2,20]
            },
            Ticket {
                field_data: vec![38,6,12]
            },
        ];

        assert_eq!(day16_part1_calc(&rules, &other_tickets), 71);
    }
}
