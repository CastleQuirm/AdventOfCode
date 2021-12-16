// Potential improvements:
//

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    let parsed_input = Packet::parse(
        &input_lines[0]
            .chars()
            .flat_map(to_binary)
            .collect::<Vec<u64>>(),
    );

    (
        parsed_input.0.sum_version_nums(),
        parsed_input.0.packet_value(),
    )
}

fn to_binary(character: char) -> Vec<u64> {
    match character {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => panic!("Didn't recognise hex!"),
    }
}

fn from_binary(binary: &[u64]) -> u64 {
    binary.iter().fold(0, |num, acc| num * 2 + acc)
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u64,
    type_id: u64,
    content: PacketBody,
}
impl Packet {
    fn parse(binary_input: &[u64]) -> (Self, usize) {
        let version = from_binary(&binary_input[0..3]);
        let type_id = from_binary(&binary_input[3..6]);
        let mut index: usize = 6;

        let content = if type_id == 4 {
            let mut binary_number: Vec<u64> = Vec::new();
            let mut more = true;
            while more {
                if binary_input[index] == 0 {
                    more = false;
                }
                (1..5).for_each(|i| binary_number.push(binary_input[index + (i as usize)]));
                index += 5;
            }
            PacketBody::Literal {
                value: from_binary(&binary_number),
            }
        } else {
            let length_type = binary_input[6];
            let mut packets: Vec<Packet> = Vec::new();
            let length = match length_type {
                0 => {
                    let length = from_binary(&binary_input[7..22]);
                    index = 22;
                    while index < length as usize + 22 {
                        let (new_packet, moved_bits) = Packet::parse(&binary_input[index..]);
                        packets.push(new_packet);
                        index += moved_bits;
                    }

                    OperatorLength::BitCount { length }
                }
                1 => {
                    let length = from_binary(&binary_input[7..18]);
                    index = 18;
                    while packets.len() < length as usize {
                        let (new_packet, moved_bits) = Packet::parse(&binary_input[index..]);
                        packets.push(new_packet);
                        index += moved_bits;
                    }
                    OperatorLength::PacketCount { length }
                }
                _ => panic!("unrecognised length_type"),
            };

            PacketBody::Operator { length, packets }
        };

        (
            Packet {
                version,
                type_id,
                content,
            },
            index,
        )
    }

    fn sum_version_nums(&self) -> u64 {
        let contained_packet_sum = match &self.content {
            PacketBody::Literal { value: _ } => 0,
            PacketBody::Operator { length: _, packets } => {
                packets.iter().map(|packet| packet.sum_version_nums()).sum()
            }
        };
        self.version + contained_packet_sum
    }

    fn packet_value(&self) -> u64 {
        match &self.content {
            PacketBody::Literal { value } => *value,
            PacketBody::Operator { length: _, packets } => match &self.type_id {
                0 => packets.iter().map(|packet| packet.packet_value()).sum(),
                1 => packets.iter().map(|packet| packet.packet_value()).product(),
                2 => packets
                    .iter()
                    .map(|packet| packet.packet_value())
                    .min()
                    .expect("Operator had no sub-packets"),
                3 => packets
                    .iter()
                    .map(|packet| packet.packet_value())
                    .max()
                    .expect("Operator had no sub-packets"),
                5 => {
                    assert_eq!(packets.len(), 2);
                    if packets[0].packet_value() > packets[1].packet_value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    assert_eq!(packets.len(), 2);
                    if packets[0].packet_value() < packets[1].packet_value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    assert_eq!(packets.len(), 2);
                    if packets[0].packet_value() == packets[1].packet_value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unrecognised operator!"),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum PacketBody {
    Literal {
        value: u64,
    },
    Operator {
        length: OperatorLength,
        packets: Vec<Packet>,
    },
}

#[derive(Debug, PartialEq)]
enum OperatorLength {
    BitCount { length: u64 },
    PacketCount { length: u64 },
}

#[cfg(test)]
mod tests {
    use super::day16;
    use super::to_binary;
    use super::Packet;
    use super::PacketBody;
    // use super::OperatorLength;

    #[test]
    fn check_day16_single_literal() {
        let hex = "D2FE28".to_string();
        let result = Packet::parse(&hex.chars().flat_map(to_binary).collect::<Vec<u64>>());
        assert_eq!(
            result.0,
            Packet {
                version: 6,
                type_id: 4,
                content: PacketBody::Literal { value: 2021 }
            }
        );
        assert_eq!(result.1, 21);
    }

    #[test]
    fn check_day16_case1() {
        let input_lines = "8A004A801A8002F478"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day16(&input_lines), (16, 0));
    }

    #[test]
    fn check_day16_case2() {
        let input_lines = "620080001611562C8802118E34"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day16(&input_lines), (12, 0));
    }

    #[test]
    fn check_day16_case3() {
        let input_lines = "C0015000016115A2E0802F182340"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day16(&input_lines), (23, 0));
    }

    #[test]
    fn check_day16_case4() {
        let input_lines = "A0016C880162017C3686B18A3D4780"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day16(&input_lines), (31, 0));
    }
}
