use std::collections::HashMap;

// Potential improvements:
// This could do with a fair bit of cleanup, not least consolodating part 2 into part 1 (which would...probably be faster?)
// At the least, there's definitely repeated code between the two parts even if we can't cheaply pull the first result out of the second answer.
// There's also a lot of "do the following for each of x,y,z" which isn't looped because of wanting multiple variable names,
// but should definitely reuse code through a subfunction or something.
// And it's slow: even with the compressed grid making part 2 solvable, it takes ~13.5 seconds to run.

pub fn day22(input_lines: &[String]) -> (u64, u64) {
    let part1_result = day22_part1(input_lines);

    let part2_result = day22_part2(input_lines);

    (part1_result, part2_result)
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

        if x_min > 50 || y_min > 50 || z_min > 50 || x_max < -50 || y_max < -50 || z_max < -50 {
            return;
        }
        for x in x_min..(x_max + 1) {
            for y in y_min..(y_max + 1) {
                for z in z_min..(z_max + 1) {
                    part1_grid[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = set_value;
                }
            }
        }
    });

    part1_grid
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|line| line.iter().sum::<u64>())
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn day22_part2(input_lines: &[String]) -> u64 {
    let boxes = input_lines
        .iter()
        .map(|line| InstructionBox::new(line))
        .collect::<Vec<InstructionBox>>();

    // For each coordinate, create an ordered list without duplicates of all values referenced and maps to and from the index in the ordering.
    let mut x_coords = boxes
        .iter()
        .flat_map(|ibox| vec![ibox.x_min, ibox.x_max])
        .collect::<Vec<i32>>();
    x_coords.sort_unstable();
    x_coords.dedup();
    let x_order_to_coord = x_coords
        .iter()
        .enumerate()
        .map(|(i, x)| (i, *x))
        .collect::<HashMap<usize, i32>>();
    let x_coord_to_order = x_coords
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect::<HashMap<i32, usize>>();

    let mut y_coords = boxes
        .iter()
        .flat_map(|ibox| vec![ibox.y_min, ibox.y_max])
        .collect::<Vec<i32>>();
    y_coords.sort_unstable();
    y_coords.dedup();
    let y_order_to_coord = y_coords
        .iter()
        .enumerate()
        .map(|(i, y)| (i, *y))
        .collect::<HashMap<usize, i32>>();
    let y_coord_to_order = y_coords
        .iter()
        .enumerate()
        .map(|(i, y)| (*y, i))
        .collect::<HashMap<i32, usize>>();

    let mut z_coords = boxes
        .iter()
        .flat_map(|ibox| vec![ibox.z_min, ibox.z_max])
        .collect::<Vec<i32>>();
    z_coords.sort_unstable();
    z_coords.dedup();
    let z_order_to_coord = z_coords
        .iter()
        .enumerate()
        .map(|(i, z)| (i, *z))
        .collect::<HashMap<usize, i32>>();
    let z_coord_to_order = z_coords
        .iter()
        .enumerate()
        .map(|(i, z)| (*z, i))
        .collect::<HashMap<i32, usize>>();

    // For each box, light and unlight the cells in a squashed grid.
    let mut squashed_grid = vec![vec![vec![0; z_coords.len()]; y_coords.len()]; x_coords.len()];
    boxes.iter().for_each(|ibox| {
        let x_min_pos = *x_coord_to_order.get(&ibox.x_min).expect("Unknown x min");
        let x_max_pos = *x_coord_to_order.get(&ibox.x_max).expect("Unknown x max");
        let y_min_pos = *y_coord_to_order.get(&ibox.y_min).expect("Unknown y min");
        let y_max_pos = *y_coord_to_order.get(&ibox.y_max).expect("Unknown y max");
        let z_min_pos = *z_coord_to_order.get(&ibox.z_min).expect("Unknown z min");
        let z_max_pos = *z_coord_to_order.get(&ibox.z_max).expect("Unknown z max");
        for plane in squashed_grid.iter_mut().take(x_max_pos).skip(x_min_pos) {
            for line in plane.iter_mut().take(y_max_pos).skip(y_min_pos) {
                for point in line.iter_mut().take(z_max_pos).skip(z_min_pos) {
                    *point = ibox.set_value;
                }
            }
        }
    });

    // For each cell in the squashed grid that is lit, calculate its size and sum up!
    squashed_grid
        .iter()
        .enumerate()
        .map(|(x, plane)| {
            plane
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(z, point)| {
                            if *point == 1 {
                                let x_dim =
                                    (x_order_to_coord.get(&(x + 1)).expect("Unknown x position")
                                        - x_order_to_coord.get(&x).expect("Unknown x position"))
                                        as u64;
                                let y_dim =
                                    (y_order_to_coord.get(&(y + 1)).expect("Unknown y position")
                                        - y_order_to_coord.get(&y).expect("Unknown y position"))
                                        as u64;
                                let z_dim =
                                    (z_order_to_coord.get(&(z + 1)).expect("Unknown z position")
                                        - z_order_to_coord.get(&z).expect("Unknown z position"))
                                        as u64;

                                x_dim * y_dim * z_dim
                            } else {
                                assert_eq!(*point, 0);
                                0_u64
                            }
                        })
                        .sum::<u64>()
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct InstructionBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    set_value: i32,
}

impl InstructionBox {
    fn new(instruction: &str) -> Self {
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

        let x_min = x_params[0].parse::<i32>().expect("Couldn't parse");
        let y_min = y_params[0].parse::<i32>().expect("Couldn't parse");
        let z_min = z_params[0].parse::<i32>().expect("Couldn't parse");

        let x_max = x_params[1].parse::<i32>().expect("Couldn't parse") + 1;
        let y_max = y_params[1].parse::<i32>().expect("Couldn't parse") + 1;
        let z_max = z_params[1].parse::<i32>().expect("Couldn't parse") + 1;

        assert_ne!(x_min, x_max);
        assert_ne!(y_min, y_max);
        assert_ne!(z_min, z_max);

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            set_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day22_part1;
    use super::day22_part2;
    use super::InstructionBox;

    #[test]
    fn check_day22_part1() {
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

    #[test]
    fn check_day22_part2() {
        let input_lines = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day22_part1(&input_lines), 474140);
        assert_eq!(day22_part2(&input_lines), 2758514936282235);
    }

    #[test]
    fn day22_create_ibox() {
        assert_eq!(
            InstructionBox::new("on x=-111166..-40997,y=-71714..2688,z=5609..50954"),
            InstructionBox {
                x_min: -111166,
                x_max: -40996,
                y_min: -71714,
                y_max: 2689,
                z_min: 5609,
                z_max: 50955,
                set_value: 1
            }
        )
    }
}
