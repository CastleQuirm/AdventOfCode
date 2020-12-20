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
    println!("{} Tiles to place", tileset.len());

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
    println!("Total hashses in map: {}", total_hashes_in_map);

    // TODO
    let num_seamonsters = 0;
    let part2_answer = total_hashes_in_map - 15 * num_seamonsters;

    (part1_answer, part2_answer)
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
            .collect::<Vec<String>>()[2..9]
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

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(IntoEnumIterator)]
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
    use super::Tile;

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
}
