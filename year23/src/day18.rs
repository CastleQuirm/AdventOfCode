// Potential improvements:
//

use grid::coord::Coord2;
use grid::directions::Direction;
use grid::directions::Rotation::{Left, Right};

pub fn day18(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = determine_area(
        input_lines[0]
            .iter()
            .map(|instruction| Trench::part1_read(instruction))
            .collect::<Vec<Trench>>(),
    );
    let answer2 = determine_area(
        input_lines[0]
            .iter()
            .map(|instruction| Trench::part2_read(instruction))
            .collect::<Vec<Trench>>(),
    );
    (format!("{}", answer1), format!("{}", answer2))
}

fn determine_area(trenches: Vec<Trench>) -> i64 {
    // IMPORTANT ASSUMPTION: TRENCHES NEVER OVERLAY EACH OTHER. THIS COULD BE WRONG. (It's not for Part 1)
    // We also assume that every consecutive instruction is a 90 degree turn i.e. no doubling-back
    // or continuing straight on in the same colour. This is asserted on below.

    // Start by working out which side is the inside. Do this by counting the angles we turn.
    let mut trenches_with_loop = trenches.clone();
    trenches_with_loop.push(trenches[0].clone());
    let (left_turns, right_turns) =
        trenches_with_loop
            .windows(2)
            .fold(
                (0, 0),
                |(left_acc, right_acc), trench_pair| match trench_pair[0]
                    .dir
                    .count_rotation(&trench_pair[1].dir)
                {
                    Left => (left_acc + 1, right_acc),
                    Right => (left_acc, right_acc + 1),
                    _ => panic!("Wasn't expecting trenches to do anything other than turn!"),
                },
            );

    // This should take us a full loop in one direction or the other.
    let total_turns: i64 = left_turns - right_turns;
    assert_eq!(total_turns.abs(), 4);

    // We now want a set of vertices for the blocks. We'll say each square we remove is *centred*
    // on the integer vertices i.e. our first vertex is (0, 0) but the square as a whole would have
    // vertices (-0.5, -0.5) to (0.5, 0.5).  We'll then add in the missing sections later.
    let mut vertices: Vec<Coord2> = Vec::new();
    let mut current_vertex = Coord2::from((0, 0));
    for trench in &trenches {
        vertices.push(current_vertex);
        let movement = match trench.dir {
            Direction::Left => Coord2::from((-trench.len, 0)),
            Direction::Right => Coord2::from((trench.len, 0)),
            Direction::Up => Coord2::from((0, trench.len)),
            Direction::Down => Coord2::from((0, -trench.len)),
        };
        current_vertex = current_vertex.sum(&movement);
    }
    // The trench must close the loop.  This is also an important last element for shoelace.
    assert_eq!(current_vertex, Coord2::from((0, 0)));
    vertices.push(current_vertex);

    // Work out the area of the enclosed space using shoelace theorem. The sum of the determinants
    // gives twice the area, but rather than halving we'll actually doubly for now to make sure we
    // keep to integers in the following step. (Hence the result here is 4 * area, apart from the
    // bits we're missing.)
    let quadruple_inner_shoelace = 2 * vertices
        .windows(2)
        .map(|vertex_pair| {
            vertex_pair[0].x * vertex_pair[1].y - vertex_pair[0].y * vertex_pair[1].x
        })
        .sum::<i64>()
        .abs();

    // Add the extra area in the outside of the border: it's an extra half-square for every non-corner space,
    // three-quarter square for outside corner, and one-quarter square for every inside corner. Multiply all
    // of these by 4 to work with integers.
    let quadruple_side_bonus = 2 * trenches.iter().map(|trench| trench.len - 1).sum::<i64>();
    let quadruple_corner_bonus = if total_turns.is_negative() {
        // We've turned more Right than Left
        3 * right_turns + left_turns
    } else {
        // We've turned more Left than Right
        right_turns + 3 * left_turns
    };
    let quadruple_area = quadruple_inner_shoelace + quadruple_side_bonus + quadruple_corner_bonus;
    assert_eq!(quadruple_area % 4, 0);
    quadruple_area / 4
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Trench {
    dir: Direction,
    len: i64,
}

impl Trench {
    fn part1_read(instruction: &str) -> Self {
        let mut sections = instruction.split_whitespace();
        Self {
            dir: sections
                .next()
                .expect("No text at all")
                .parse::<Direction>()
                .expect("Wasn't a direction"),
            len: sections
                .next()
                .expect("No length")
                .parse::<i64>()
                .expect("Couldn't parse the length"),
        }
    }

    fn part2_read(instruction: &str) -> Self {
        let mut relevant_text = instruction
            .split_once(" (#")
            .and_then(|(_, hex)| hex.strip_suffix(')'))
            .expect("No colour format")
            .to_owned();
        assert_eq!(relevant_text.len(), 6);
        let colour_char = relevant_text.pop().unwrap();
        let dir = match colour_char {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Unexpected direction!"),
        };
        assert_eq!(relevant_text.len(), 5);
        let len =
            i64::from_str_radix(&relevant_text, 16).expect("Couldn't read length as hexadecimal");
        Self { dir, len }
    }
}

impl From<&String> for Trench {
    fn from(value: &String) -> Self {
        let mut sections = value.split_whitespace();
        Self {
            dir: sections
                .next()
                .expect("No text at all")
                .parse::<Direction>()
                .expect("Wasn't a direction"),
            len: sections
                .next()
                .expect("No length")
                .parse::<i64>()
                .expect("Couldn't parse the length"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day18;
    use crate::utils::load_input;

    #[test]
    fn check_day18_case01() {
        full_test(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)", // INPUT STRING
            "62",           // PART 1 RESULT
            "952408144115", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day18(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
