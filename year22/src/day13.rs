use std::cmp::Ordering;

use crate::utils::split_input_by_blocks;

pub fn day13(input_lines: &str) -> (String, String) {
    let packet_pairs = split_input_by_blocks(input_lines, parse_packet_pair);
    let all_packets = packet_pairs
        .iter()
        .fold(Vec::new(), |mut collection, (p1, p2)| {
            collection.push(p1);
            collection.push(p2);
            collection
        });
    let answer1 = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| if pair.0 < pair.1 { Some(i + 1) } else { None })
        .sum::<usize>();

    let divider_packet_1 = divider_packet(2);
    let divider_packet_2 = divider_packet(6);

    let mut count_1 = 1;
    let mut count_2 = 2;

    for packet in all_packets {
        if divider_packet_1 > *packet {
            count_1 += 1;
        }
        if divider_packet_2 > *packet {
            count_2 += 1;
        }
    }

    let answer2 = count_1 * count_2;
    (format!("{}", answer1), format!("{}", answer2))
}

fn divider_packet(value: u64) -> Packet {
    Packet {
        contents: vec![Contents::Packet {
            packet: Packet {
                contents: vec![Contents::Value { value }],
            },
        }],
    }
}

fn parse_packet_pair(lines: &[&str]) -> (Packet, Packet) {
    assert_eq!(lines.len(), 2);
    (parse_packet(lines[0]).0, parse_packet(lines[1]).0)
}

#[derive(Debug)]
struct Packet {
    contents: Vec<Contents>,
}

fn parse_packet(text: &str) -> (Packet, usize) {
    let mut char_ptr = 1;
    let mut content_list = Vec::new();
    while text.chars().nth(char_ptr).expect("Looped off the string") != ']' {
        let potential_content = match text.chars().nth(char_ptr).expect("Looped off the string") {
            '[' => {
                let (result, jump) = parse_packet(&text[char_ptr..]);
                char_ptr += jump;
                Some(Contents::Packet { packet: result })
            }
            d if d.is_numeric() => {
                if text
                    .chars()
                    .nth(char_ptr + 1)
                    .expect("Looped off the string")
                    .is_numeric()
                {
                    Some(Contents::Value {
                        value: text[char_ptr..=char_ptr + 1]
                            .parse::<u64>()
                            .expect("Parsing failed"),
                    })
                } else {
                    Some(Contents::Value {
                        value: d.to_string().parse::<u64>().expect("Parsing failed"),
                    })
                }
            }
            ']' => unreachable!(),
            ',' => None,
            _ => panic!(),
        };
        char_ptr += 1;
        if let Some(content) = potential_content {
            content_list.push(content);
        }
    }
    (
        Packet {
            contents: content_list,
        },
        char_ptr,
    )
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

#[derive(Debug)]
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

    // #[test]
    // fn check_day13_part2_case1() {
    //     assert_eq!(day13("").1, "0".to_string())
    // }

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
            ("13".to_string(), "140".to_string())
        )
    }
}
