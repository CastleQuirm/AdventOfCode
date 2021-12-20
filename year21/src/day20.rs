// Potential improvements:
// 1. It feels inefficient that when enhancing we need to read each existing cell 9 times (for impact on every neighbor)
// 2. We could do clever stuff like for each row down, the lookup number is the lookup for the cell above % 64 * 8 + the new line's value
// 3. enhance_multiple() needing to do one enhance then afterwards do the rest seems silly, but avoids an unnecessary clone - is there anything
//    better we could do?

pub fn day20(input_lines: &[String]) -> (u64, u64) {
    // Read first line as the enhancement algorithm
    let enhancement_algorithm = input_lines[0]
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("unrecognised enhancement element"),
        })
        .collect::<Vec<u32>>();
    assert_eq!(enhancement_algorithm.len(), 512);

    // read the rest off as the starting map.
    let starting_map = Map::new(&input_lines[2..]);

    // Enhance twice
    let twice_enhanced = starting_map.enhance_multiple(&enhancement_algorithm, 2);

    // Count the lit up cells.
    let part1 = twice_enhanced.count_lit();

    // Enhance 48 more times and count lit cells.
    let part2 = twice_enhanced
        .enhance_multiple(&enhancement_algorithm, 48)
        .count_lit();

    (part1, part2)
}

struct Map {
    image: Vec<Vec<u32>>,
    infinite_value: u32,
}

impl Map {
    fn new(lines: &[String]) -> Self {
        let image = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => 1,
                        '.' => 0,
                        _ => panic!("unrecognised map element"),
                    })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        Self {
            image,
            infinite_value: 0,
        }
    }

    fn enhance_multiple(&self, enhancement_algorithm: &[u32], factor: usize) -> Self {
        let mut map = self.enhance(enhancement_algorithm);
        (1..factor).for_each(|_| map = map.enhance(enhancement_algorithm));
        map
    }

    fn enhance(&self, enhancement_algorithm: &[u32]) -> Self {
        let new_infinite_value = match self.infinite_value {
            0 => enhancement_algorithm[0],
            1 => enhancement_algorithm[511],
            _ => panic!("Unrecognised infinite value in old map"),
        };

        let new_map_size = self.image.len() + 2;
        assert_eq!(new_map_size, self.image[0].len() + 2);

        let new_image = (0..new_map_size)
            .map(|i| {
                (0..new_map_size)
                    .map(|j| {
                        // Get the lookup index
                        let decimal_value = (0..3)
                            .flat_map(|di| {
                                (0..3).map(move |dj| {
                                    if i + di < 2
                                        || j + dj < 2
                                        || i + di >= new_map_size
                                        || j + dj >= new_map_size
                                    {
                                        self.infinite_value
                                    } else {
                                        self.image[i + di - 2][j + dj - 2]
                                    }
                                })
                            })
                            .fold(0, |acc, x| acc * 2 + x)
                            as usize;

                        // Look up that index
                        enhancement_algorithm[decimal_value]
                    })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        Self {
            image: new_image,
            infinite_value: new_infinite_value,
        }
    }

    fn count_lit(&self) -> u64 {
        assert_eq!(self.infinite_value, 0); // If we're calling this on a map that has its infinite value lit, the result would be infinite.
        self.image
            .iter()
            .map(|line| line.iter().sum::<u32>() as u64)
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::day20;

    #[test]
    fn check_day20() {
        let input_lines = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day20(&input_lines), (35, 3351));
    }
}
