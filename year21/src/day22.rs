// Potential improvements:
//

pub fn day22(input_lines: &[String]) -> (u64, u64) {
    let part1_result = day22_part1(&input_lines[0..20]);
    
    (part1_result, 0)
}

fn day22_part1(input_lines: &[String]) -> u64 {
    let mut part1_grid = vec![vec![vec![0; 101]; 101]; 101];
    input_lines.iter().for_each(|instruction| {
        let split = instruction.split(' ').collect::<Vec<&str>>();
        let set_value = match split[0] {
            "on" => 1,
            "off" => 0,
            _ => panic!("Unrecognised command!"),
        };
        let split = split[1].split(',').collect::<Vec<&str>>();
        let x_params = &split[0][2..].split("..").collect::<Vec<&str>>();
        let y_params = &split[1][2..].split("..").collect::<Vec<&str>>();
        let z_params = &split[2][2..].split("..").collect::<Vec<&str>>();

        let x_min = x_params[0].parse::<i32>().expect("Couldn't parse").max(-50);
        let y_min = y_params[0].parse::<i32>().expect("Couldn't parse").max(-50);
        let z_min = z_params[0].parse::<i32>().expect("Couldn't parse").max(-50);

        let x_max = x_params[1].parse::<i32>().expect("Couldn't parse").min(50);
        let y_max = y_params[1].parse::<i32>().expect("Couldn't parse").min(50);
        let z_max = z_params[1].parse::<i32>().expect("Couldn't parse").min(50);

        if x_min > 50 || y_min > 50 || z_min > 50 || x_max < -50 || y_max < -50 || z_max < -50 { return; }
        for x in x_min..(x_max + 1) {
            for y in y_min..(y_max + 1) {
                for z in z_min..(z_max + 1) {
                    part1_grid[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = set_value;
                }
            }
        }
    });
    
    part1_grid.iter().map(|plane| {
        plane.iter().map(|line| {
            line.iter().sum::<u64>()
        }).sum::<u64>()
    }).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::day22_part1;

    #[test]
    fn check_day22() {
        let input_lines = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day22_part1(&input_lines), 590784);
    }
}
