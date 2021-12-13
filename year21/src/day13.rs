use std::{cmp::Ordering, collections::HashSet};

// Potential improvements:
// Eh, it all feels a bit hacky.
// In particular it'd be nice to commonise fold_up and fold_left.

pub fn day13(input_lines: &[String]) -> (u64, u64) {
    let mut dots: HashSet<Coordinate> = HashSet::new();
    let mut instructions: Vec<Fold> = Vec::new();

    input_lines.iter().for_each(|line| {
        if line.contains(',') {
            let coords = line
                .split(',')
                .map(|str| str.parse::<u64>().expect("Couldn't parse"))
                .collect::<Vec<u64>>();
            dots.insert(Coordinate {
                i: coords[0],
                j: coords[1],
            });
        } else if line.contains('=') {
            let axis = if line.contains('x') {
                Axis::X
            } else if line.contains('y') {
                Axis::Y
            } else {
                panic!("Where do we fold?")
            };
            let value = line.split('=').collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .expect("Couldn't parse");
            instructions.push(Fold { axis, value });
        }
    });

    let part1_answer = fold_paper(&dots, &instructions[0]).len() as u64;

    for fold in instructions {
        dots = fold_paper(&dots, &fold);
    }

    let max_x = dots.iter().map(|dot| dot.i).max().expect("No max x?");
    let max_y = dots.iter().map(|dot| dot.j).max().expect("No max y?");

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if dots.contains(&Coordinate { i: x, j: y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    (part1_answer, 0)
}

fn fold_paper(dots: &HashSet<Coordinate>, instruction: &Fold) -> HashSet<Coordinate> {
    match instruction.axis {
        Axis::X => fold_left(dots, instruction.value),
        Axis::Y => fold_up(dots, instruction.value),
    }
}

fn fold_left(dots: &HashSet<Coordinate>, value: u64) -> HashSet<Coordinate> {
    let mut new_dots: HashSet<Coordinate> = HashSet::new();

    for dot in dots {
        match dot.i.cmp(&value) {
            Ordering::Less => new_dots.insert(*dot),
            Ordering::Greater => new_dots.insert(Coordinate {
                i: 2 * value - dot.i,
                j: dot.j,
            }),
            Ordering::Equal => panic!("Wasn't supposed to be a dot on the line!"),
        };
    }

    new_dots
}

fn fold_up(dots: &HashSet<Coordinate>, value: u64) -> HashSet<Coordinate> {
    let mut new_dots: HashSet<Coordinate> = HashSet::new();

    for dot in dots {
        match dot.j.cmp(&value) {
            Ordering::Less => new_dots.insert(*dot),
            Ordering::Greater => new_dots.insert(Coordinate {
                i: dot.i,
                j: 2 * value - dot.j,
            }),
            Ordering::Equal => panic!("Wasn't supposed to be a dot on the line!"),
        };
    }

    new_dots
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    i: u64,
    j: u64,
}

struct Fold {
    axis: Axis,
    value: u64,
}

enum Axis {
    X,
    Y,
}

#[cfg(test)]
mod tests {
    use super::day13;

    #[test]
    fn check_day13() {
        let input_lines = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day13(&input_lines), (17, 0));
    }
}
