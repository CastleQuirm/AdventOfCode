// Potential improvements:
//

pub fn day02(input_lines: &[String]) -> (u64, u64) {
    let (part1_distance, part1_depth) = input_lines.iter().fold((0,0), |acc, instruction| {
        let (direction, val) = direction_and_val(instruction);
        match direction {
            "forward" => (acc.0 + val, acc.1),
            "up" => (acc.0, acc.1 - val),
            "down" => (acc.0, acc.1 + val),
            _ => panic!("Unrecognised direction"),
        }
    });
    
    let (part2_distance, part2_depth, _) = input_lines.iter().fold((0,0,0), |acc, instruction| {
        let (direction, val) = direction_and_val(instruction);
        match direction {
            "forward" => (acc.0 + val, acc.1 + val * acc.2, acc.2),
            "up" => (acc.0, acc.1, acc.2 - val),
            "down" => (acc.0, acc.1, acc.2 + val),
            _ => panic!("Unrecognised direction"),
        }
    });

    
    
    ((part1_distance * part1_depth), (part2_distance * part2_depth))
}

fn direction_and_val(line: &String) -> (&str, u64) {
    let splitline = line.split(" ").collect::<Vec<&str>>();
    let direction = splitline.first().expect("No first part of splitline");
    let val = splitline.last().expect("No second part of splitline").to_string().parse::<u64>().expect("Couldn't parse the second part as a u64");
    (direction, val)
}