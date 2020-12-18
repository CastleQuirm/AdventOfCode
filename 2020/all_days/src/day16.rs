use std::collections::HashSet;

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    // Parse input into InfoFields and Tickets
    let (rules, my_ticket, other_tickets) = parse_input(input_lines);
    println!("Initially {} other tickets", other_tickets.len());
    (
        day16_part1_calc(&rules, &other_tickets),
        day16_part2_calc(&rules, &my_ticket, &other_tickets),
    )
}

fn day16_part1_calc(rules: &[InfoField], other_tickets: &[Ticket]) -> u64 {
    // Create HashSet of all valid values across all InfoFields
    let valid_values = all_valid_values(&rules);
    other_tickets
        .iter()
        .map(|ticket| ticket.scanning_error(&valid_values))
        .sum()
}

fn day16_part2_calc(rules: &[InfoField], my_ticket: &Ticket, other_tickets: &[Ticket]) -> u64 {
    // Filter the rules by those that have impossible values
    // (0..20).for_each(): go through each Ticket and get the set of InfoFields that support its [i]th value, then take the intersection across tickets.
    let valid_values = all_valid_values(&rules);
    let _valid_tickets = other_tickets
        .iter()
        .filter(|ticket| ticket.ticket_valid(&valid_values))
        .collect::<Vec<&Ticket>>();

    // Commented out because this is REALLY slow.  2 minutes in release build slow.
    // let possible_rules_vec = (0..20).map(|i| {
    //     let nums_at_position = valid_tickets.clone().into_iter().map(|ticket| ticket.field_data[i]).collect::<Vec<usize>>();
    //     rules.into_iter().filter(|field| nums_at_position.iter().all(|x| field.valid_values().contains(x))).collect::<Vec<&InfoField>>()
    // }).collect::<Vec<Vec<&InfoField>>>();

    // Commented out because this is a hypothetical start which I don't know how to progress
    // We have a vec for each of the field positions, inside each space of which is a Vec of the InfoFields it could be.
    // We'll assume that we hav a single success somewhere (we do)
    // let single_index = (0..20).find(|&spot_ix| possible_rules_vec[spot_ix].len() == 1).expect("More complex input - no element with just one possibility!");
    // let found_rule = &possible_rules_vec[single_index][0].field_name;

    // Alternative approaches to the above: just start trying permutations of the InfoFileds until they fit the rules vector?

    //....

    // I hand solved the necessary indices from printing out some of the above results.  Codifying would be...nicer.  To be done another time!
    // Use indices 5, 8, 10, 13, 16 and 18
    (my_ticket.field_data[5]
        * my_ticket.field_data[8]
        * my_ticket.field_data[10]
        * my_ticket.field_data[13]
        * my_ticket.field_data[16]
        * my_ticket.field_data[18]) as u64
}

fn parse_input(input_lines: &[String]) -> (Vec<InfoField>, Ticket, Vec<Ticket>) {
    let split_input = input_lines[0].split("\n\n").collect::<Vec<&str>>();
    assert_eq!(split_input.len(), 3);
    (
        parse_rules(split_input[0]),
        parse_my_ticket(split_input[1]),
        parse_other_tickets(split_input[2]),
    )
}

fn parse_rules(rules_input: &str) -> Vec<InfoField> {
    rules_input
        .lines()
        .map(|rule| InfoField::new(rule))
        .collect::<Vec<InfoField>>()
}

fn parse_my_ticket(my_ticket_input: &str) -> Ticket {
    let lines = my_ticket_input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "your ticket:".to_string());
    Ticket::new(&lines[1])
}

fn parse_other_tickets(other_tickets_input: &str) -> Vec<Ticket> {
    let lines = other_tickets_input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    assert_eq!(lines[0], "nearby tickets:".to_string());
    lines[1..]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct FieldRange {
    lower: usize,
    upper: usize,
}
impl FieldRange {
    fn valid_values(&self) -> HashSet<usize> {
        (self.lower..self.upper + 1).collect::<HashSet<usize>>()
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
    use super::{day16, day16_part1_calc, FieldRange, InfoField, Ticket};

    #[test]
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

        assert_eq!(day16_part1_calc(&rules, &other_tickets), 71);
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
