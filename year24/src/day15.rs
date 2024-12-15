// Potential improvements:
//

use std::collections::HashSet;

use crate::{coord::Coord2, directions::CompassDirection, grid::Grid};

pub fn day15(input_lines: &[Vec<String>]) -> (String, String) {
    let mut warehouse = Grid::<WarehouseSpace>::from_input(&input_lines[0]);
    let answer1 = process_moves(&mut warehouse, &input_lines[1]);

    let mut large_layout = input_lines[0].clone();
    large_layout.iter_mut().for_each(|line| {
        *line = line
            .replace('#', "##")
            .replace('O', "[]")
            .replace('.', "..")
            .replace('@', "@.");
    });
    warehouse = Grid::<WarehouseSpace>::from_input(&large_layout);
    let answer2 = process_moves(&mut warehouse, &input_lines[1]);

    (format!("{}", answer1), format!("{}", answer2))
}

fn process_moves(warehouse: &mut Grid<WarehouseSpace>, moves: &[String]) -> i64 {
    let mut robot_loc = warehouse
        .find_single_element(&WarehouseSpace::Robot)
        .expect("Not exactly one robot");

    // Once we know where the robot starts, we don't have to track it in the warehouse with the enum
    warehouse.set_cell(&robot_loc, &WarehouseSpace::Space);

    for line in moves {
        for movement in line.chars() {
            // print_grid(warehouse);
            let direction = match movement {
                '<' => CompassDirection::West,
                '>' => CompassDirection::East,
                '^' => CompassDirection::North,
                'v' => CompassDirection::South,
                _ => panic!(),
            };

            let target_dest = robot_loc.compass_sum(&direction);
            match warehouse.peek(&target_dest) {
                WarehouseSpace::Wall => (),                       // No-op
                WarehouseSpace::Space => robot_loc = target_dest, // Just a simple move
                WarehouseSpace::SingleBox => {
                    // Find the first cell in this direction that isn't a box.
                    let mut end_boxes = target_dest;
                    while warehouse.peek(&end_boxes) == &WarehouseSpace::SingleBox {
                        end_boxes = end_boxes.compass_sum(&direction);
                    }
                    match warehouse.peek(&end_boxes) {
                        WarehouseSpace::Wall => (), // There was nothing to do after all!
                        WarehouseSpace::Space => {
                            warehouse.set_cell(&end_boxes, &WarehouseSpace::SingleBox);
                            warehouse.set_cell(&target_dest, &WarehouseSpace::Space);
                            robot_loc = target_dest;
                        }
                        WarehouseSpace::BoxLeft
                        | WarehouseSpace::BoxRight
                        | WarehouseSpace::SingleBox
                        | WarehouseSpace::Robot => panic!(),
                    }
                }
                WarehouseSpace::BoxLeft | WarehouseSpace::BoxRight => {
                    if direction == CompassDirection::West || direction == CompassDirection::East {
                        // Just a straight line, but allowing for multiple boxes with different sides
                        // Find the first cell in this direction that isn't a box.
                        let mut end_boxes = target_dest;
                        let mut dist = 1;
                        while warehouse.peek(&end_boxes) == &WarehouseSpace::BoxLeft
                            || warehouse.peek(&end_boxes) == &WarehouseSpace::BoxRight
                        {
                            end_boxes = end_boxes.compass_sum(&direction);
                            dist += 1;
                        }
                        match warehouse.peek(&end_boxes) {
                            WarehouseSpace::Wall => (), // There was nothing to do after all!
                            WarehouseSpace::Space => {
                                robot_loc = target_dest;
                                let mut cell_was = warehouse.get(&target_dest);
                                warehouse.set_cell(&target_dest, &WarehouseSpace::Space);

                                (1..dist).for_each(|i| {
                                    let changed_cell =
                                        target_dest.sum(&Coord2::from_compass(&direction).mult(i));
                                    warehouse.set_cell(&changed_cell, &cell_was);
                                    cell_was = if cell_was == WarehouseSpace::BoxLeft {
                                        WarehouseSpace::BoxRight
                                    } else {
                                        WarehouseSpace::BoxLeft
                                    };
                                });
                            }
                            WarehouseSpace::BoxLeft
                            | WarehouseSpace::BoxRight
                            | WarehouseSpace::SingleBox
                            | WarehouseSpace::Robot => panic!(),
                        }
                    } else {
                        // More complicated!
                        let mut examine_boxes = HashSet::from([target_dest]);
                        if warehouse.peek(&target_dest) == &WarehouseSpace::BoxLeft {
                            assert_eq!(
                                warehouse.peek(&target_dest.compass_sum(&CompassDirection::East)),
                                &WarehouseSpace::BoxRight
                            );
                            examine_boxes.insert(target_dest.compass_sum(&CompassDirection::East));
                        } else {
                            assert_eq!(
                                warehouse.peek(&target_dest.compass_sum(&CompassDirection::West)),
                                &WarehouseSpace::BoxLeft
                            );
                            examine_boxes.insert(target_dest.compass_sum(&CompassDirection::West));
                        }

                        let mut hit_wall = false;
                        let mut moving_cells_by_row = Vec::<HashSet<Coord2>>::new();
                        while !hit_wall && !examine_boxes.is_empty() {
                            moving_cells_by_row.push(examine_boxes.clone());
                            examine_boxes = examine_boxes
                                .iter()
                                .flat_map(|space| {
                                    let next_space = space.compass_sum(&direction);
                                    match warehouse.peek(&next_space) {
                                        WarehouseSpace::Wall => {
                                            hit_wall = true;
                                            HashSet::new()
                                        }
                                        WarehouseSpace::SingleBox | WarehouseSpace::Robot => {
                                            panic!()
                                        }
                                        WarehouseSpace::Space => HashSet::new(),
                                        WarehouseSpace::BoxLeft => HashSet::from([
                                            next_space,
                                            next_space.compass_sum(&CompassDirection::East),
                                        ]),
                                        WarehouseSpace::BoxRight => HashSet::from([
                                            next_space,
                                            next_space.compass_sum(&CompassDirection::West),
                                        ]),
                                    }
                                })
                                .collect::<HashSet<_>>();
                        }

                        if !hit_wall {
                            // We can actually move.
                            while let Some(box_row) = moving_cells_by_row.pop() {
                                box_row.iter().for_each(|cell| {
                                    warehouse.set_cell(
                                        &cell.compass_sum(&direction),
                                        &warehouse.get(cell),
                                    );
                                    warehouse.set_cell(cell, &WarehouseSpace::Space);
                                });
                            }
                            robot_loc = target_dest;
                        }
                    }
                }
                WarehouseSpace::Robot => panic!(), // It's...us!
            }
            // print_grid(warehouse);
            // panic!()
        }
    }

    warehouse
        .filter_elements(
            &(|t: &WarehouseSpace| {
                t == &WarehouseSpace::SingleBox || t == &WarehouseSpace::BoxLeft
            }),
        )
        .iter()
        .map(|box_coord| box_coord.y * 100 + box_coord.x)
        .sum::<i64>()
}

// fn print_grid(warehouse: &Grid<WarehouseSpace>) {
//     for line in &warehouse.grid {
//         for cell in line {
//             let character = match cell {
//                 WarehouseSpace::Wall => '#',
//                 WarehouseSpace::SingleBox => 'O',
//                 WarehouseSpace::Robot => '@',
//                 WarehouseSpace::Space => '.',
//                 WarehouseSpace::BoxLeft => '[',
//                 WarehouseSpace::BoxRight => ']',
//             };
//             print!("{}", character);
//         }
//         println!();
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WarehouseSpace {
    Wall,
    SingleBox,
    Robot,
    Space,
    BoxLeft,
    BoxRight,
}

impl From<char> for WarehouseSpace {
    fn from(value: char) -> Self {
        match value {
            '#' => WarehouseSpace::Wall,
            'O' => WarehouseSpace::SingleBox,
            '.' => WarehouseSpace::Space,
            '@' => WarehouseSpace::Robot,
            '[' => WarehouseSpace::BoxLeft,
            ']' => WarehouseSpace::BoxRight,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day15;
    use crate::utils::load_input;

    #[test]
    fn check_day15_case01() {
        full_test(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<", // INPUT STRING
            "2028", // PART 1 RESULT
            "1751", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day15_case02() {
        full_test(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^", // INPUT STRING
            "10092", // PART 1 RESULT
            "9021",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day15(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
