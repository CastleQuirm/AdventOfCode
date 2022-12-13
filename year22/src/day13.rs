use std::{cmp::Ordering, str::FromStr, string::ParseError};

use crate::utils::split_input_by_blocks;

pub fn day13(input_lines: &str) -> (String, String) {
    let packet_pairs = split_input_by_blocks(input_lines, parse_packet_pair);
    let answer1 = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| if pair.0 < pair.1 { Some(i + 1) } else { None })
        .sum::<usize>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn parse_packet_pair(lines: &[&str]) -> (Packet, Packet) {
    assert_eq!(lines.len(), 2);
    (
        lines[0]
            .parse::<Packet>()
            .expect("Couldn't parse first line of pair"),
        lines[1]
            .parse::<Packet>()
            .expect("Couldn't parse second line of pair"),
    )
}

struct Packet {
    contents: Vec<Contents>,
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..usize::min(self.contents.len(), other.contents.len()) {
            let value_cmp = self.contents[i].cmp(&other.contents[i]);
            if value_cmp != Ordering::Equal {
                return value_cmp;
            }
        }
        self.contents.len().cmp(&other.contents.len())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents
    }
}

enum Contents {
    Value { value: u64 },
    Packet { packet: Packet },
}

impl Ord for Contents {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Contents::Value { value: l_value }, Contents::Value { value: r_value }) => {
                l_value.cmp(r_value)
            }
            (Contents::Value { value: l_value }, Contents::Packet { packet: r_packet }) => Packet {
                contents: vec![Self::Value { value: *l_value }],
            }
            .cmp(r_packet),
            (Contents::Packet { packet: l_packet }, Contents::Value { value: r_value }) => l_packet
                .cmp(&Packet {
                    contents: vec![Self::Value { value: *r_value }],
                }),
            (Contents::Packet { packet: l_packet }, Contents::Packet { packet: r_packet }) => {
                l_packet.cmp(r_packet)
            }
        }
    }
}

impl PartialOrd for Contents {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Contents {}

impl PartialEq for Contents {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value { value: l_value }, Self::Value { value: r_value }) => l_value == r_value,
            (Self::Packet { packet: l_packet }, Self::Packet { packet: r_packet }) => {
                l_packet == r_packet
            }
            (Self::Value { value: l_value }, Self::Packet { packet: r_packet }) => {
                Packet {
                    contents: vec![Self::Value { value: *l_value }],
                } == *r_packet
            }
            (Self::Packet { packet: l_packet }, Self::Value { value: r_value }) => {
                Packet {
                    contents: vec![Self::Value { value: *r_value }],
                } == *l_packet
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day13_part1_case1() {
        let l_packet = Packet {
            contents: vec![
                Contents::Value { value: 1 },
                Contents::Value { value: 1 },
                Contents::Value { value: 3 },
                Contents::Value { value: 1 },
                Contents::Value { value: 1 },
            ],
        };
        let r_packet = Packet {
            contents: vec![
                Contents::Value { value: 1 },
                Contents::Value { value: 1 },
                Contents::Value { value: 5 },
                Contents::Value { value: 1 },
                Contents::Value { value: 1 },
            ],
        };
        assert!(l_packet < r_packet);

        let l_packet = Packet {
            contents: vec![
                Contents::Packet {
                    packet: Packet {
                        contents: vec![Contents::Value { value: 1 }],
                    },
                },
                Contents::Packet {
                    packet: Packet {
                        contents: vec![
                            Contents::Value { value: 2 },
                            Contents::Value { value: 3 },
                            Contents::Value { value: 4 },
                        ],
                    },
                },
            ],
        };
        let r_packet = Packet {
            contents: vec![
                Contents::Packet {
                    packet: Packet {
                        contents: vec![Contents::Value { value: 1 }],
                    },
                },
                Contents::Value { value: 4 },
            ],
        };
        assert!(l_packet < r_packet);

        let l_packet = Packet {
            contents: vec![Contents::Packet {
                packet: Packet {
                    contents: vec![Contents::Packet {
                        packet: Packet {
                            contents: Vec::new(),
                        },
                    }],
                },
            }],
        };
        let r_packet = Packet {
            contents: vec![Contents::Packet {
                packet: Packet {
                    contents: Vec::new(),
                },
            }],
        };
        assert!(l_packet > r_packet);

        // assert_eq!(day13("").0, "0".to_string())
    }

    #[test]
    fn check_day13_part2_case1() {
        assert_eq!(day13("").1, "0".to_string())
    }

    #[test]
    fn check_day13_both_case1() {
        assert_eq!(
            day13(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
            ("13".to_string(), "0".to_string())
        )
    }
}
