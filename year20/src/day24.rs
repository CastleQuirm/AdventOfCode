// Possible improvements
// 1) Commonise with the other Game of Life days.  Hex grid is just the same as a square grid with adjacency added along one diagnoal; could use either method for both grid types.
// 2) Get traits with required derives working.  union_sets should be over <T> but it needs to know PartialEq is available for T.
// 3) Work out why I needed union_sets as a function at all!  It should have worked with the direct application to the HashSets, but got a compiler error.

use std::collections::{HashMap, HashSet};

pub fn day24(input_lines: &[String]) -> (u64, u64) {
    let mut black_cells: HashSet<(isize, isize)> = HashSet::new();

    input_lines.iter().for_each(|line| {
        let dest_coords = dest_coords(line);
        if black_cells.contains(&dest_coords) {
            black_cells.remove(&dest_coords);
        } else {
            black_cells.insert(dest_coords);
        }
    });
    let part1_answer = black_cells.len() as u64;

    (1..101).for_each(|_| {
        black_cells = iterate(&black_cells);
    });

    (part1_answer, black_cells.len() as u64)
}

fn dest_coords(line: &str) -> (isize, isize) {
    let (mut x, mut y) = (0isize, 0isize);
    let mut i = 0usize;
    while i < line.len() {
        match line.chars().nth(i).unwrap() {
            'e' => x += 1,
            'w' => x -= 1,
            'n' => {
                y -= 1;
                i += 1;
                if line.chars().nth(i).unwrap() == 'e' {
                    x += 1;
                }
            }
            's' => {
                y += 1;
                i += 1;
                if line.chars().nth(i).unwrap() == 'w' {
                    x -= 1;
                }
            }
            _ => unreachable!(),
        }
        i += 1;
    }
    (x, y)
}

fn iterate(black_cells: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    // Build up a map of all black-cell adjacencies
    let mut adjacent_cells: HashMap<(isize, isize), usize> = HashMap::new();
    let adjacencies: [(isize, isize); 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

    black_cells.iter().for_each(|&cell| {
        adjacencies.iter().for_each(|&delta| {
            let prev_adj = *adjacent_cells
                .get(&(cell.0 + delta.0, cell.1 + delta.1))
                .unwrap_or(&0);
            adjacent_cells.insert((cell.0 + delta.0, cell.1 + delta.1), prev_adj + 1);
        })
    });

    // Adjacent Cells now contains every co-ordinate with one or more black cells adjacent.
    // Need white cells that have turned black: the adjacent_cells.keys().difference(black_cells).filter(|cell| *adjacent_cells.get(cell).unwrap_or(&0) == 2)
    let white_to_black = adjacent_cells
        .keys()
        .copied()
        .collect::<HashSet<(isize, isize)>>()
        .difference(black_cells)
        .filter(|cell| adjacent_cells.get(cell).unwrap_or(&0) == &2)
        .copied()
        .collect::<HashSet<(isize, isize)>>();

    // Need black cells that remain black: exactly 1 or 2 black cells adjacent.
    let black_to_black = black_cells
        .iter()
        .filter(|cell| {
            *adjacent_cells.get(cell).unwrap_or(&0) == 1
                || *adjacent_cells.get(cell).unwrap_or(&0) == 2
        })
        .copied()
        .collect::<HashSet<(isize, isize)>>();

    union_sets(white_to_black, black_to_black)
}

fn union_sets(a: HashSet<(isize, isize)>, b: HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    a.union(&b).cloned().collect::<HashSet<(isize, isize)>>()
}

#[cfg(test)]
mod tests {
    use super::day24;

    #[test]
    fn day24_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day24(&input), (10, 2208));
    }
}
