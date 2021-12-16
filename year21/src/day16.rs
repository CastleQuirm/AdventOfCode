// Potential improvements:
//

pub fn day16(input_lines: &[String]) -> (u64, u64) {
    let parsed_input = Packet::parse(&input_lines[0].chars().flat_map(to_binary).collect::<Vec<u64>>());
    println!("{:?}", parsed_input);

    (0, 0)
}

fn to_binary(character: char) -> Vec<u64> {
    match character {
        '0' => vec![0,0,0,0],
        '1' => vec![0,0,0,1],
        '2' => vec![0,0,1,0],
        '3' => vec![0,0,1,1],
        '4' => vec![0,1,0,0],
        '5' => vec![0,1,0,1],
        '6' => vec![0,1,1,0],
        '7' => vec![0,1,1,1],
        '8' => vec![1,0,0,0],
        '9' => vec![1,0,0,1],
        'A' => vec![1,0,1,0],
        'B' => vec![1,0,1,1],
        'C' => vec![1,1,0,0],
        'D' => vec![1,1,0,1],
        'E' => vec![1,1,1,0],
        'F' => vec![1,1,1,1],
        _ => panic!("Didn't recognise hex!"),
    }
}

fn from_binary(binary: &[u64]) -> u64 {
    binary.iter().fold(0, |num, acc| num * 2 + acc)
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    content: PacketBody
}
impl Packet {
    fn parse(binary_input: &[u64]) -> Self {
        let version = from_binary(&binary_input[0..3]);
        let type_id = from_binary(&binary_input[3..6]);

        let content = if type_id == 4 {
            let mut index: usize = 6;
            let mut binary_number: Vec<u64> = Vec::new();
            let mut more = true;
            while more {
                if binary_input[index] == 0 { more = false; }
                (1..5).for_each(|i| binary_number.push(binary_input[index + (i as usize)]));
                index += 5;
            }

            PacketBody::Literal { value: from_binary(&binary_number) }
        } else {
            let length_type = binary_input[6];
            let length = match length_type {
                0 => {
                    let length = from_binary(&binary_input[7..22]);
                    OperatorLength::BitCount { length }
                }
                1 => {
                    let length = from_binary(&binary_input[7..18]);
                    OperatorLength::PacketCount { length }
                }
                _ => panic!("unrecognised length_type")
            };

            PacketBody::Operator{ length, packets: Vec::new() }
        };

        Packet { version, type_id, content }
    }
}

#[derive(Debug)]
enum PacketBody {
    Literal { value: u64 },
    Operator { length: OperatorLength, packets: Vec<Packet> },
}

#[derive(Debug)]
enum OperatorLength {
    BitCount { length: u64 },
    PacketCount { length: u64 }
}

#[cfg(test)]
mod tests {
    use super::day16;

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
