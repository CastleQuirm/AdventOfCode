// Potential improvements:
//

// use std::collections::HashSet;

// use crate::utils::Coord;
// use regex::Regex;

pub fn day17(_input_lines: &[Vec<String>]) -> (String, String) {
    // Read the input into...just a hash of Coords?
    // let rock: HashSet<Coord> = build_rock_map(&input_lines[0]);
    // let depths = rock.iter().map(|c| c.y);
    // let min_depth = depths.clone().min().unwrap();
    // let max_depth = depths.max().unwrap();

    // let mut water_falling: HashSet<Coord> = HashSet::new();
    // let mut water_standing: HashSet<Coord> = HashSet::new();

    // // first water source: (500,0)
    // let mut water_sources = vec![Coord { x: 500, y: 0 }];

    // if min_depth <= 0 {
    //     water_permeates.insert(Coord { x: 500, y: 0 });
    // }

    // from a source:
    // 'source: loop {
    //     let pour_from = water_sources.pop();
    //     println!("Source loop: pour from {:?}", pour_from);
    //     if let Some(mut source) = pour_from {
    //         while !rock.contains(&source.plus(0, 1)) {
    //             // println!("water falls");
    //             if source.y == max_depth {
    //                 continue 'source; // Water has poured off the grid, can ignore this.
    //             }
    //             water_permeates.insert(source.plus(0, 1));
    //             source = source.plus(0, 1);
    //         }
    //         // Water has hit a basin, start filling up.
    //         let mut new_source = false;
    //         while !new_source {
    //             println!("fill a row from {}", source);
    //             let mut raise_source = true;
    //             // fill left
    //             let mut flow_left = source.clone();
    //             while !rock.contains(&flow_left.plus(-1, 0)) && !water_permeates.contains(&flow_left.plus(-1, 0)) {
    //                 println!("fill left");
    //                 flow_left = flow_left.plus(-1, 0);
    //                 water_permeates.insert(flow_left);
    //                 if !rock.contains(&flow_left.plus(0, 1)) && !water_permeates.contains(&flow_left.plus(0, 1)) {
    //                     water_sources.push(flow_left);
    //                     new_source = true;
    //                     break;
    //                 }
    //             }
    //             // if we stopped because we hit other water, is there another source in that water to keep flowing?
    //             if water_permeates.contains(&flow_left.plus(-1, 0)) {
    //                 let mut check_left = flow_left.plus(-1, 0);
    //                 while !rock.contains(&check_left) {
    //                     if water_sources.contains(&check_left) {
    //                         raise_source = false;
    //                     }
    //                     check_left = check_left.plus(-1, 0);
    //                 }
    //             }

    //             // fill right
    //             let mut flow_right = source.clone();
    //             while !rock.contains(&flow_right.plus(1, 0)) && !water_permeates.contains(&flow_right.plus(1, 0)) {
    //                 println!("fill right");
    //                 flow_right = flow_right.plus(1, 0);
    //                 water_permeates.insert(flow_right);
    //                 if !rock.contains(&flow_right.plus(0, 1)) && !water_permeates.contains(&flow_right.plus(0, 1)) {
    //                     water_sources.push(flow_right);
    //                     new_source = true;
    //                     break;
    //                 }
    //             }

    //             // if we stopped because we hit other water, is there another source in that water to keep flowing?
    //             if water_permeates.contains(&flow_right.plus(1, 0)) {
    //                 let mut check_right = flow_right.plus(1, 0);
    //                 while !rock.contains(&check_right) {
    //                     if water_sources.contains(&check_right) {
    //                         raise_source = false;
    //                     }
    //                     check_right = check_right.plus(1, 0);
    //                 }
    //             }

    //             // Move up the source.
    //             if raise_source {
    //                 source = source.plus(0, -1);
    //             } else {
    //                 continue 'source;
    //             }
    //             // if Some(source) == pour_from {
    //             //     continue 'source;
    //             // }
    //             if water_permeates.len()> 200000 { break 'source; }
    //         }
    //     } else {
    //         break; // No sources to follow, we're done
    //     }
    // }

    // - remove unprocessed sources
    // - waterfall in every space below until the next space below is rock.
    // - then fill that row (move left and right independantly until EITHER rock in the next space in that direction OR no rock underneath)
    //   - if no rock underneath: add new source for said space below.
    //   - check sources.  If empty, nxt iteration of outer loop.
    //   - otherwise: next iteration of inner loop, filling row above (left and right of waterfall) (CHECK WE HAVEN'T GONE ABOVE SOURCE FOR THIS FALL)

    // let min_x = water_permeates.iter().map(|i| i.x).min().unwrap();
    // let max_x = water_permeates.iter().map(|i| i.x).max().unwrap();

    // for y in min_depth..=max_depth {
    //     for x in 400..=600 {
    //         let c = if rock.contains(&Coord{x, y}) {'#'} else if water_permeates.contains(&Coord{x, y}) {'~'} else {' '};
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

// fn build_rock_map(lines: &[String]) -> HashSet<Coord> {
//     let mut rock: HashSet<Coord> = HashSet::new();

//     for line in lines {
//         let re = Regex::new(r"(\w)=(\d+), (\w)=(\d+)..(\d+)").unwrap();
//         re.captures(line).iter()
//             .for_each(|cap| {
//                 let const_letter = cap[1].parse::<char>().unwrap();
//                 let const_val = cap[2].parse::<i32>().unwrap();
//                 let range_letter = cap[3].parse::<char>().unwrap();
//                 let range_low = cap[4].parse::<i32>().unwrap();
//                 let range_high = cap[5].parse::<i32>().unwrap();

//                 assert!((const_letter == 'x' && range_letter == 'y') || (const_letter == 'y' && range_letter == 'x'));

//                 (range_low..=range_high).for_each(|i| {
//                     if const_letter == 'x' {
//                         rock.insert(Coord { x: const_val, y: i });
//                     } else {
//                         rock.insert(Coord { x: i, y: const_val });
//                     }
//                 })
//             });
//     }

//     rock
// }

// fn water_fall(
//     source: Coord,
//     rocks: &HashSet<Coord>,
//     mut standing_water: HashSet<Coord>,
//     mut falling_water: HashSet<Coord>
// ) {
//     // work out whether we'll get a fall or standing water
//     let mut next_water = source.plus(0, 1);

//     if rocks.contains(&next_water) {
//         return;
//     }
//     if rocks.contains(&next_water.plus(0, 1)) {
//         falling_water.insert(next_water.clone());
//     } else {
//         standing_water.insert(next_water.clone());
//     }
// }

#[cfg(test)]
mod tests {
    use super::day17;
    use crate::utils::load_input;

    #[test]
    fn check_day17_case01() {
        full_test(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504", // INPUT STRING
            "57", // PART 1 RESULT
            "0",  // PART 2 RESULT
        )
    }

    #[test]
    fn check_day17_fill_stacked_basins_inner_first() {
        full_test(
            "x=495, y=1..7
y=7, x=495..505
x=505, y=2..7
x=498, y=3..5
y=5, x=498..501
x=501, y=3..5
x=508, y=0..2",
            "0",
            "0",
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day17(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
