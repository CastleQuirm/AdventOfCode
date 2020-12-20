// Possible Part 1 alternative: create a flat vector of all the side values across each tile, then find the unique elements.
// Then for each tile, count how many of its '8' sides were in theat unique list, and filter to just those tiles with 4 such elements.
// Hope we have four tiles left, in which case those are the corners!  (Shouldn't be able to have more; might have fewer.)

use enum_iterator::IntoEnumIterator;

pub fn day20(input_lines: &[String]) -> (u64, u64) {
    // Read the input into a set of Tiles
    let tileset: Vec<Tile> = input_lines[0]
        .split("\n\n")
        .map(|tile| Tile::new(tile))
        .collect::<Vec<Tile>>();
    let mut placed_tiles: Vec<(&Tile, Rotation)> = Vec::new();

    if !fill_space(&tileset, &mut placed_tiles) {
        println!("Couldn't find an answer");
        unreachable!();
    }

    let part1_answer = (placed_tiles[0].0.index
        * placed_tiles[11].0.index
        * placed_tiles[132].0.index
        * placed_tiles[143].0.index) as u64;

    let total_hashes_in_map = placed_tiles
        .iter()
        .map(|(tile, _)| {
            tile.main_tile
                .iter()
                .map(|line| line.chars().filter(|&c| c == '#').count() as u64)
                .sum::<u64>()
        })
        .sum::<u64>();

    let constructed_map: Vec<Vec<char>> = (0..96)
        .map(|y| {
            // Create the vec of chars for line y.
            (0..96)
                .map(|x| {
                    // Create the char for spot x,y.
                    let (tile, rot) = &placed_tiles[12 * (y / 8) + (x / 8)];
                    let raw_map: &Vec<String> = &tile.main_tile;
                    let (x_index, y_index) = match rot {
                        Rotation::Original => (x % 8, y % 8),
                        Rotation::Original90 => (y % 8, 7 - x % 8),
                        Rotation::Original180 => (7 - x % 8, 7 - y % 8),
                        Rotation::Original270 => (7 - y % 8, x % 8),
                        Rotation::Reflect => (7 - x % 8, y % 8),
                        Rotation::Reflect90 => (y % 8, x % 8),
                        Rotation::Reflect180 => (x % 8, 7 - y % 8),
                        Rotation::Reflect270 => (7 - y % 8, 7 - x % 8),
                    };
                    raw_map[y_index].chars().nth(x_index).unwrap()
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let num_seamonsters = Rotation::into_enum_iter()
        .map(|rot| count_seamonsters(&constructed_map, rot))
        .sum::<u64>();

    let part2_answer = total_hashes_in_map - 15 * num_seamonsters;

    (part1_answer, part2_answer)
}

fn count_seamonsters(full_map: &[Vec<char>], rot: Rotation) -> u64 {
    let max = full_map.len();
    match rot {
        Rotation::Original => (1..max - 1)
            .map(|y| {
                (0..max - 19)
                    .filter(|&x| is_seamonster(full_map, x, y, 1, 1))
                    .count() as u64
            })
            .sum(),
        Rotation::Reflect => (1..max - 1)
            .map(|y| {
                (19..max)
                    .filter(|&x| is_seamonster(full_map, x, y, -1, 1))
                    .count() as u64
            })
            .sum(),
        Rotation::Reflect180 => (1..max - 1)
            .map(|y| {
                (0..max - 19)
                    .filter(|&x| is_seamonster(full_map, x, y, 1, -1))
                    .count() as u64
            })
            .sum(),
        Rotation::Original180 => (1..max - 1)
            .map(|y| {
                (19..max)
                    .filter(|&x| is_seamonster(full_map, x, y, -1, -1))
                    .count() as u64
            })
            .sum(),
        Rotation::Original90 => (1..max - 1)
            .map(|y| {
                (19..max)
                    .filter(|&x| is_rotated_seamonster(full_map, x, y, -1, 1))
                    .count() as u64
            })
            .sum(),
        Rotation::Original270 => (1..max - 1)
            .map(|y| {
                (0..max - 19)
                    .filter(|&x| is_rotated_seamonster(full_map, x, y, 1, -1))
                    .count() as u64
            })
            .sum(),
        Rotation::Reflect90 => (1..max - 1)
            .map(|y| {
                (0..max - 19)
                    .filter(|&x| is_rotated_seamonster(full_map, x, y, 1, 1))
                    .count() as u64
            })
            .sum(),
        Rotation::Reflect270 => (1..max - 1)
            .map(|y| {
                (19..max)
                    .filter(|&x| is_rotated_seamonster(full_map, x, y, -1, -1))
                    .count() as u64
            })
            .sum(),
    }
}

fn is_seamonster(full_map: &[Vec<char>], x: usize, y: usize, x_mult: isize, y_mult: isize) -> bool {
    let monster_cells: Vec<char> = vec![
        full_map[y][x],
        full_map[ind(y, 1, y_mult)][ind(x, 1, x_mult)],
        full_map[ind(y, 1, y_mult)][ind(x, 4, x_mult)],
        full_map[y][ind(x, 5, x_mult)],
        full_map[y][ind(x, 6, x_mult)],
        full_map[ind(y, 1, y_mult)][ind(x, 7, x_mult)],
        full_map[ind(y, 1, y_mult)][ind(x, 10, x_mult)],
        full_map[y][ind(x, 11, x_mult)],
        full_map[y][ind(x, 12, x_mult)],
        full_map[ind(y, 1, y_mult)][ind(x, 13, x_mult)],
        full_map[ind(y, 1, y_mult)][ind(x, 16, x_mult)],
        full_map[y][ind(x, 17, x_mult)],
        full_map[y][ind(x, 18, x_mult)],
        full_map[y][ind(x, 19, x_mult)],
        full_map[ind(y, 1, -y_mult)][ind(x, 18, x_mult)],
    ];

    monster_cells.iter().all(|&c| c == '#')
}

fn is_rotated_seamonster(
    full_map: &[Vec<char>],
    x: usize,
    y: usize,
    x_mult: isize,
    y_mult: isize,
) -> bool {
    let monster_cells: Vec<char> = vec![
        full_map[x][y],
        full_map[ind(x, 1, x_mult)][ind(y, 1, y_mult)],
        full_map[ind(x, 4, x_mult)][ind(y, 1, y_mult)],
        full_map[ind(x, 5, x_mult)][y],
        full_map[ind(x, 6, x_mult)][y],
        full_map[ind(x, 7, x_mult)][ind(y, 1, y_mult)],
        full_map[ind(x, 10, x_mult)][ind(y, 1, y_mult)],
        full_map[ind(x, 11, x_mult)][y],
        full_map[ind(x, 12, x_mult)][y],
        full_map[ind(x, 13, x_mult)][ind(y, 1, y_mult)],
        full_map[ind(x, 16, x_mult)][ind(y, 1, y_mult)],
        full_map[ind(x, 17, x_mult)][y],
        full_map[ind(x, 18, x_mult)][y],
        full_map[ind(x, 19, x_mult)][y],
        full_map[ind(x, 18, x_mult)][ind(y, 1, -y_mult)],
    ];

    monster_cells.iter().all(|&c| c == '#')
}

fn ind(num: usize, modif: usize, mult: isize) -> usize {
    (num as isize + (modif as isize) * mult) as usize
}

fn fill_space<'a>(tileset: &'a [Tile], already_placed: &mut Vec<(&'a Tile, Rotation)>) -> bool {
    let place_number = already_placed.len();
    let mut overall_result = false;
    assert!(place_number < 144);

    // Try each tile
    tileset.iter().for_each(|tile| {
        // Ignore this tile if it's already placed
        if !already_placed
            .iter()
            .map(|(t, _)| t.index)
            .any(|x| x == tile.index)
        {
            // Try each rotation of the tile
            Rotation::into_enum_iter().for_each(|rot|
                // Escape route for when we've got a successful placement
                if !overall_result {
                    // See if we can place this tile in this rotation - nothing to do if not
                    if try_placing_tile((tile, &rot), already_placed) {
                        // Put this (tile, rotation) pair on to the end of our list
                        already_placed.push((tile, rot));
                        // If we hadn't already placed 143 tiles, continue.  Otherwise we're done!
                        if place_number < 143 {
                            // Try the next tile.  If it bails out eventually, remove this last tile; otherwise start unravelling the stack.
                            if !fill_space(tileset, already_placed) {
                                already_placed.pop();
                            } else {
                                overall_result = true;
                            }
                        } else {
                            overall_result = true;
                        }
                    }
                }
            )
        }
    });
    overall_result
}

fn try_placing_tile(candidate: (&Tile, &Rotation), already_placed: &[(&Tile, Rotation)]) -> bool {
    let place_number = already_placed.len();
    // If this tile isn't on the top row, check its North edge.
    if place_number >= 12 {
        let existing = &already_placed[place_number - 12];
        if candidate.0.get_edge(Direction::North, &candidate.1)
            != existing.0.get_flipped_edge(Direction::South, &existing.1)
        {
            return false;
        }
    }
    // If this tile isn't on the left row, check its West edge.
    if place_number % 12 != 0 {
        let existing = &already_placed[place_number - 1];
        if candidate.0.get_edge(Direction::West, &candidate.1)
            != existing.0.get_flipped_edge(Direction::East, &existing.1)
        {
            return false;
        }
    }
    // If each of the above tests wasn't run or passed, return success.
    true
}

#[derive(Debug)]
struct Tile {
    index: usize,
    north: usize,
    east: usize,
    south: usize,
    west: usize,
    r_north: usize,
    r_east: usize,
    r_south: usize,
    r_west: usize,
    main_tile: Vec<String>,
}
impl Tile {
    fn new(lines: &str) -> Tile {
        let first_line_split = lines
            .lines()
            .next()
            .expect("No title line")
            .split(|c| c == ' ' || c == ':')
            .collect::<Vec<&str>>();
        let index = first_line_split
            .get(1)
            .expect("No number after Tile ")
            .parse::<usize>()
            .expect("Couldn't unwrap tile index");
        let north_string = &lines.lines().nth(1).expect("No first line");
        let south_string = &lines.lines().nth(10).expect("No tenth line");
        let east_string = &lines
            .lines()
            .filter(|line| !line.contains("Tile"))
            .map(|line| line.chars().nth(9).expect("No last character on line?"))
            .collect::<String>();
        let west_string = &lines
            .lines()
            .filter(|line| !line.contains("Tile"))
            .map(|line| line.chars().next().expect("No last character on line?"))
            .collect::<String>();

        let (north, r_north) = dir_and_r_dir_values(north_string);
        let (r_south, south) = dir_and_r_dir_values(south_string); // Note south side going clockwise is opposite to the north side
        let (east, r_east) = dir_and_r_dir_values(east_string);
        let (r_west, west) = dir_and_r_dir_values(west_string); // Note west side going clockwise is opposite to the east side

        let main_tile = lines
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()[2..10]
            .iter()
            .map(|line| line[1..9].to_string())
            .collect::<Vec<String>>();

        Tile {
            index,
            north,
            east,
            south,
            west,
            r_north,
            r_east,
            r_south,
            r_west,
            main_tile,
        }
    }

    fn get_edge(&self, edge: Direction, rot: &Rotation) -> usize {
        match (edge, rot) {
            (Direction::North, Rotation::Original) => self.north,
            (Direction::North, Rotation::Original90) => self.west,
            (Direction::North, Rotation::Original180) => self.south,
            (Direction::North, Rotation::Original270) => self.east,
            (Direction::North, Rotation::Reflect) => self.r_north,
            (Direction::North, Rotation::Reflect90) => self.r_west,
            (Direction::North, Rotation::Reflect180) => self.r_south,
            (Direction::North, Rotation::Reflect270) => self.r_east,
            (Direction::East, Rotation::Original) => self.east,
            (Direction::East, Rotation::Original90) => self.north,
            (Direction::East, Rotation::Original180) => self.west,
            (Direction::East, Rotation::Original270) => self.south,
            (Direction::East, Rotation::Reflect) => self.r_west,
            (Direction::East, Rotation::Reflect90) => self.r_south,
            (Direction::East, Rotation::Reflect180) => self.r_east,
            (Direction::East, Rotation::Reflect270) => self.r_north,
            (Direction::South, Rotation::Original) => self.south,
            (Direction::South, Rotation::Original90) => self.east,
            (Direction::South, Rotation::Original180) => self.north,
            (Direction::South, Rotation::Original270) => self.west,
            (Direction::South, Rotation::Reflect) => self.r_south,
            (Direction::South, Rotation::Reflect90) => self.r_east,
            (Direction::South, Rotation::Reflect180) => self.r_north,
            (Direction::South, Rotation::Reflect270) => self.r_west,
            (Direction::West, Rotation::Original) => self.west,
            (Direction::West, Rotation::Original90) => self.south,
            (Direction::West, Rotation::Original180) => self.east,
            (Direction::West, Rotation::Original270) => self.north,
            (Direction::West, Rotation::Reflect) => self.r_east,
            (Direction::West, Rotation::Reflect90) => self.r_north,
            (Direction::West, Rotation::Reflect180) => self.r_west,
            (Direction::West, Rotation::Reflect270) => self.r_south,
        }
    }

    fn get_flipped_edge(&self, edge: Direction, rot: &Rotation) -> usize {
        match (edge, rot) {
            (Direction::North, Rotation::Original) => self.r_north,
            (Direction::North, Rotation::Original90) => self.r_west,
            (Direction::North, Rotation::Original180) => self.r_south,
            (Direction::North, Rotation::Original270) => self.r_east,
            (Direction::North, Rotation::Reflect) => self.north,
            (Direction::North, Rotation::Reflect90) => self.west,
            (Direction::North, Rotation::Reflect180) => self.south,
            (Direction::North, Rotation::Reflect270) => self.east,
            (Direction::East, Rotation::Original) => self.r_east,
            (Direction::East, Rotation::Original90) => self.r_north,
            (Direction::East, Rotation::Original180) => self.r_west,
            (Direction::East, Rotation::Original270) => self.r_south,
            (Direction::East, Rotation::Reflect) => self.west,
            (Direction::East, Rotation::Reflect90) => self.south,
            (Direction::East, Rotation::Reflect180) => self.east,
            (Direction::East, Rotation::Reflect270) => self.north,
            (Direction::South, Rotation::Original) => self.r_south,
            (Direction::South, Rotation::Original90) => self.r_east,
            (Direction::South, Rotation::Original180) => self.r_north,
            (Direction::South, Rotation::Original270) => self.r_west,
            (Direction::South, Rotation::Reflect) => self.south,
            (Direction::South, Rotation::Reflect90) => self.east,
            (Direction::South, Rotation::Reflect180) => self.north,
            (Direction::South, Rotation::Reflect270) => self.west,
            (Direction::West, Rotation::Original) => self.r_west,
            (Direction::West, Rotation::Original90) => self.r_south,
            (Direction::West, Rotation::Original180) => self.r_east,
            (Direction::West, Rotation::Original270) => self.r_north,
            (Direction::West, Rotation::Reflect) => self.east,
            (Direction::West, Rotation::Reflect90) => self.north,
            (Direction::West, Rotation::Reflect180) => self.west,
            (Direction::West, Rotation::Reflect270) => self.south,
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(IntoEnumIterator, PartialEq, Debug)]
enum Rotation {
    Original,
    Original90, // Rotate clockwise
    Original180,
    Original270,
    Reflect, // Horizontal reflection after any rotation
    Reflect90,
    Reflect180,
    Reflect270,
}

fn dir_and_r_dir_values(dir_string: &str) -> (usize, usize) {
    let binary_string = dir_string.replace(".", "0").replace("#", "1");
    (
        isize::from_str_radix(&binary_string, 2).unwrap() as usize,
        isize::from_str_radix(&binary_string.chars().rev().collect::<String>(), 2).unwrap()
            as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::{count_seamonsters, Rotation, Tile};
    use enum_iterator::IntoEnumIterator;

    #[test]
    fn parse_tile() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";
        let tile = Tile::new(input);
        assert_eq!(tile.index, 2311);
        assert_eq!(tile.north, 210);
        assert_eq!(tile.east, 89);
        assert_eq!(tile.south, 924);
        assert_eq!(tile.west, 318);
        assert_eq!(tile.r_north, 300);
        assert_eq!(tile.r_east, 616);
        assert_eq!(tile.r_south, 231);
        assert_eq!(tile.r_west, 498);
    }

    #[test]
    fn count_sample_seamonsters() {
        let input: Vec<Vec<char>> = vec![
            vec![
                '.', '#', '.', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#',
                '#', '.', '.', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '#', '#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '#', '.',
                '.', '#', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '.', '#',
                '#', '#', '#', '#', '#', '.', '.', '.',
            ],
            vec![
                '#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#',
                '#', '#', '#', '.', '#', '.', '.', '#',
            ],
            vec![
                '#', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '#',
                '#', '.', '.', '.', '#', '.', '#', '#',
            ],
            vec![
                '.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '.',
                '.', '#', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#', '.', '.', '#',
                '.', '#', '.', '#', '#', '#', '.', '.',
            ],
            vec![
                '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.',
                '.', '#', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '#', '.', '.', '#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '.',
                '#', '.', '#', '#', '.', '.', '.', '.',
            ],
            vec![
                '#', '.', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '.', '#',
                '.', '#', '.', '#', '#', '#', '.', '.',
            ],
            vec![
                '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#',
                '#', '#', '.', '#', '.', '.', '#', '#',
            ],
            vec![
                '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.', '#', '#',
                '#', '#', '#', '#', '#', '#', '.', '#',
            ],
            vec![
                '#', '#', '.', '.', '#', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#',
                '.', '#', '.', '#', '.', '#', '.', '.',
            ],
            vec![
                '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '#', '.',
                '.', '#', '#', '#', '.', '#', '#', '#',
            ],
            vec![
                '.', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.',
                '.', '#', '#', '#', '.', '#', '#', '.',
            ],
            vec![
                '#', '#', '#', '.', '#', '.', '.', '.', '#', '.', '.', '#', '.', '#', '#', '.',
                '#', '#', '#', '#', '#', '#', '.', '.',
            ],
            vec![
                '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '.', '#', '#', '.', '#',
                '.', '.', '#', '.', '#', '#', '.', '.',
            ],
            vec![
                '.', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '.', '.', '.', '#', '#',
                '#', '.', '#', '.', '.', '#', '.', '#',
            ],
            vec![
                '.', '.', '#', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '.',
                '#', '#', '#', '#', '.', '#', '#', '#',
            ],
            vec![
                '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.',
                '#', '#', '#', '.', '#', '#', '#', '.',
            ],
            vec![
                '#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#',
                '#', '#', '.', '.', '.', '.', '#', '#',
            ],
            vec![
                '#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '.',
                '#', '#', '#', '#', '.', '.', '.', '#',
            ],
            vec![
                '.', '#', '.', '#', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.',
                '#', '#', '#', '#', '.', '#', '#', '.',
            ],
            vec![
                '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '.', '#', '.',
                '.', '.', '#', '.', '.', '#', '#', '#',
            ],
        ];
        assert_eq!(
            Rotation::into_enum_iter()
                .map(|rot| count_seamonsters(&input, rot))
                .sum::<u64>(),
            2
        );
    }
}
