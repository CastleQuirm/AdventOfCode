// Potential improvements:
//

use std::collections::HashSet;

use crate::{coord::Coord2, grid::Grid};

pub fn day21(input_lines: &[Vec<String>]) -> (String, String) {
    // Create a garden. We don't add a border since in part 1 we can't reach the edge and in part 2
    // we'll want it unwalled. Given the fixed numbers in the question, I'm confident everyone's grid
    // will be 131 * 131 with the start point in the dead center. (I'm not going to do a test for this
    // puzzle since the size and step counts in the sample are annoyingly different.)
    let mut maze = Grid::<MazeSpace>::from_input(&input_lines[0]);
    assert_eq!(maze.grid.len(), 131);
    assert_eq!(maze.grid[0].len(), 131);

    // Record the start point and set it as a garden for exploration going forwards.
    let start_coord = maze
        .find_single_element(&MazeSpace::Start)
        .expect("Not a single start");
    assert_eq!(start_coord, Coord2::from((65, 65)));
    maze.set_cell(&start_coord, &MazeSpace::Garden);

    // Walk 64 steps (notably keeping us within one garden's bounds)
    let answer1 = explore(&maze, &start_coord, 64);

    // Part 2
    // Time for some arithmetic based on some observations about the map, specifically...:
    // - The map is a square of 131 * 131 (asserted on above)
    // - The start point is (65, 65) using 0 indexing from a corner (asserted on above)
    for i in 0..131 {
        // - Each edge and center row and column is a clear line, meaning we can always optimise a path
        //   to get to a given version of the grid by following those and relying on Manhattan distance
        assert_eq!(maze.get(&Coord2::from((0, i))), MazeSpace::Garden);
        assert_eq!(maze.get(&Coord2::from((65, i))), MazeSpace::Garden);
        assert_eq!(maze.get(&Coord2::from((130, i))), MazeSpace::Garden);
        assert_eq!(maze.get(&Coord2::from((i, 0))), MazeSpace::Garden);
        assert_eq!(maze.get(&Coord2::from((i, 65))), MazeSpace::Garden);
        assert_eq!(maze.get(&Coord2::from((i, 130))), MazeSpace::Garden);
        // - The diagonals between the midpoints of each side is a clear line with a gap on either side,
        //   allowing for a clear walk along them
        assert_eq!(
            maze.get(&Coord2::from(((65 - i).abs(), i))),
            MazeSpace::Garden
        );
        assert_eq!(
            maze.get(&Coord2::from(((65 - i - 1).abs(), i))),
            MazeSpace::Garden
        );
        assert_eq!(
            maze.get(&Coord2::from(((65 - i + 1).abs(), i))),
            MazeSpace::Garden
        );
        assert_eq!(
            maze.get(&Coord2::from((130 - (65 - i).abs(), i))),
            MazeSpace::Garden
        );
        assert_eq!(
            maze.get(&Coord2::from((130 - (65 - i - 1).abs(), i))),
            MazeSpace::Garden
        );
        assert_eq!(
            maze.get(&Coord2::from((130 - (65 - i + 1).abs(), i))),
            MazeSpace::Garden
        );
    }
    // - The number of steps we want to take overall is an even multiple of the grid size plus half the grid size again
    //   (this is already a given from the assumption that the grid is 131 * 131, asserted on earlier, since the total
    //   number of steps is a fixed part of the question)
    let part2_step_count = 26501365;
    assert_eq!(part2_step_count % maze.grid.len(), maze.grid.len() / 2);
    let grid_walks = part2_step_count / maze.grid.len();
    assert_eq!(grid_walks % 2, 0);
    // - The above assertions mean the basic grid is split into eight 'corners' - the four outside corners and four inside
    //   ones surrounding the centre. I *think* the process I'm using will rely on being able to reach anywhere within a corner
    //   in a number of steps not significantly larger than manhattan distance from an appropriate corner/centre-edge/centre
    //   but (a) I'm not certain what 'significant' is here or (b) if this is a meaningful assumption.  Probably good to run
    //   with though.

    // Find some numbers to do arithmetic with.
    // How many spaces can we get to within the grid with an odd or even number of steps from the starting point, regardless of how many?

    // Create a walled version of the garden and work this out - we need to try walking because there are small inaccesible spots!
    let mut walled_garden = maze.clone();
    walled_garden.add_border(&MazeSpace::Rock);
    // all_even is the number of spaces we can reach with an even number of steps from a midpoint (which we can reach from an odd number
    // of grids away). The number of steps available is always one and a half grids' length (131 + 65), and we need to update the
    // start_coord because of the wall.
    let all_even = explore(&walled_garden, &start_coord.plus(1, 1), 196);
    // This assert confirms that we can't reach anywhere else with more steps.  We could also just increase the steps count until we hit
    // a max, and assert that number of steps is <= 196, but that's more work and I'm going to hope this is good enough.
    assert_eq!(
        all_even,
        explore(&walled_garden, &start_coord.plus(1, 1), 198)
    );

    // all_odd is the same, but for odd steps away.  This time we'll safely cover two and a half grid's worth (2 * 131 + 65) = 327.
    // This would be a notably bigger saving by doing a max here, but eh.
    let all_odd = explore(&walled_garden, &start_coord.plus(1, 1), 327);
    assert_eq!(
        all_odd,
        explore(&walled_garden, &start_coord.plus(1, 1), 329)
    );

    // We also want to know the number of places in each of the four corners of the diamond
    let corner_count = explore(&walled_garden, &Coord2::from((66, 131)), 130)
        + explore(&walled_garden, &Coord2::from((66, 1)), 130)
        + explore(&walled_garden, &Coord2::from((1, 66)), 130)
        + explore(&walled_garden, &Coord2::from((131, 66)), 130);

    // And the big sections
    let big_sections = explore(&walled_garden, &Coord2::from((131, 131)), 195)
        + explore(&walled_garden, &Coord2::from((1, 131)), 195)
        + explore(&walled_garden, &Coord2::from((131, 1)), 195)
        + explore(&walled_garden, &Coord2::from((1, 1)), 195);

    // And the small sections
    let small_sections = explore(&walled_garden, &Coord2::from((131, 131)), 64)
        + explore(&walled_garden, &Coord2::from((1, 131)), 64)
        + explore(&walled_garden, &Coord2::from((131, 1)), 64)
        + explore(&walled_garden, &Coord2::from((1, 1)), 64);

    // Add together with the suitable multipliers.
    let answer2 = corner_count
        + (grid_walks - 1).pow(2) * all_odd
        + grid_walks.pow(2) * all_even
        + (grid_walks - 1) * big_sections
        + grid_walks * small_sections;
    (format!("{}", answer1), format!("{}", answer2))
}

fn explore(maze: &Grid<MazeSpace>, start_coord: &Coord2, steps: u64) -> usize {
    let mut superposition = HashSet::from([*start_coord]);
    (0..steps).for_each(|_| {
        superposition = superposition
            .iter()
            .flat_map(|position| {
                position
                    .orthoganally_adjacent()
                    .into_iter()
                    .filter(|candidate| maze.get(candidate) != MazeSpace::Rock)
                    .collect::<HashSet<Coord2>>()
            })
            .collect::<HashSet<Coord2>>();
    });
    superposition.len()
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum MazeSpace {
    Garden,
    Rock,
    Start,
}

impl From<char> for MazeSpace {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Garden,
            'S' => Self::Start,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day21;
    use crate::utils::load_input;

    #[test]
    fn check_day21_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day21(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
