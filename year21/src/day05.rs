use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

// Potential improvements:
// 1. THIS IS SLOW. It takes ~36 seconds (SECONDS, NOT MILLI!) to complete.  IN RELEASE MODE
//    - 1a. Obviously wasteful: creating line_segment_collection twice by over-enthusiastic filtering.  Do the filter for part1 and reuse the collection.
//    - 1b. Obviously wasteful: rerunning all the pairwise intersections in Part 1 for Part 2.  These are all still valid!  We could run that just once.
//    That said: Part 1 on its own took ~4 seconds, so best we can gain by removing duplicate work is that.  32 seconds of this is still baked in to Part 2.
//    Much bigger efficiencies will require doing something much more clever/neat than pairwise HashSet intersections.
//    E.g. storing line segments just as their end-points, and writing a SMART (& FAST) function that calculates the intersection.
// 2. Probably obsoleted by the above, but the 9-case handling of LineSeg::new() working out the contained points is ugly and almost certainly reducable
//    (or at least more commonisable)

pub fn day05(input_lines: &[String]) -> (u64, u64) {
    // parse all the line segments
    let line_segment_collection = input_lines
        .iter()
        .map(|line| LineSeg::new(line))
        .filter(|seg| !seg.diagonal)
        .collect::<Vec<LineSeg>>();

    // for every pair run .intersection() on one of the pair using the other,
    // then union them all and count the total.
    let part1 = line_segment_collection
        .iter()
        .combinations(2)
        .map(|pair| pair[0].intersection(pair[1]))
        .reduce(|overlaps, acc| {
            overlaps
                .union(&acc)
                .cloned()
                .collect::<HashSet<Coordinate>>()
        })
        .expect("Didn't have any combinations to reduce")
        .len() as u64;

    // parse all the line segments
    let line_segment_collection = input_lines
        .iter()
        .map(|line| LineSeg::new(line))
        .collect::<Vec<LineSeg>>();

    // for every pair run .intersection() on one of the pair using the other,
    // then union them all and count the total.
    let part2 = line_segment_collection
        .iter()
        .combinations(2)
        .map(|pair| pair[0].intersection(pair[1]))
        .reduce(|overlaps, acc| {
            overlaps
                .union(&acc)
                .cloned()
                .collect::<HashSet<Coordinate>>()
        })
        .expect("Didn't have any combinations to reduce")
        .len() as u64;

    (part1, part2)
}

struct LineSeg {
    diagonal: bool,
    contains: HashSet<Coordinate>,
}

impl LineSeg {
    fn new(line: &str) -> Self {
        let coords = line.split(" -> ").collect::<Vec<&str>>();

        let start = Coordinate::new(coords.first().expect("No first coordinate"));
        let end = Coordinate::new(coords.last().expect("No last coordinate"));

        let diagonal = start.x != end.x && start.y != end.y;
        let mut contains: HashSet<Coordinate> = [start].iter().cloned().collect();
        match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
            (Ordering::Equal, Ordering::Equal) => (), // Single point line, don't need to add anything else.
            (Ordering::Equal, Ordering::Less) => {
                for y in start.y..end.y {
                    assert!(contains.insert(Coordinate {
                        x: start.x,
                        y: y + 1
                    })) // add 1 to y - we've already added start, and we need to include end.
                }
            }
            (Ordering::Equal, Ordering::Greater) => {
                for y in end.y..start.y {
                    assert!(contains.insert(Coordinate { x: start.x, y }))
                    // we've already got start, so OK to finish early.
                }
            }
            (Ordering::Less, Ordering::Equal) => {
                for x in start.x..end.x {
                    assert!(contains.insert(Coordinate {
                        x: x + 1,
                        y: start.y
                    })) // add 1 to x - we've already added start, and we need to include end.
                }
            }
            (Ordering::Greater, Ordering::Equal) => {
                for x in end.x..start.x {
                    assert!(contains.insert(Coordinate { x, y: start.y })) // we've already got start, so OK to finish early.
                }
            }
            (Ordering::Less, Ordering::Less) => {
                for i in 1..(end.x - start.x + 1) {
                    assert!(contains.insert(Coordinate {
                        x: start.x + i,
                        y: start.y + i
                    }))
                }
            }
            (Ordering::Less, Ordering::Greater) => {
                for i in 1..(end.x - start.x + 1) {
                    assert!(contains.insert(Coordinate {
                        x: start.x + i,
                        y: start.y - i
                    }))
                }
            }
            (Ordering::Greater, Ordering::Less) => {
                for i in 1..(start.x - end.x + 1) {
                    assert!(contains.insert(Coordinate {
                        x: start.x - i,
                        y: start.y + i
                    }))
                }
            }
            (Ordering::Greater, Ordering::Greater) => {
                for i in 1..(start.x - end.x + 1) {
                    assert!(contains.insert(Coordinate {
                        x: start.x - i,
                        y: start.y - i
                    }))
                }
            }
        }

        LineSeg { diagonal, contains }
    }
    fn intersection(&self, other: &Self) -> HashSet<Coordinate> {
        self.contains
            .intersection(&other.contains)
            .cloned()
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: u64,
    y: u64,
}
impl Coordinate {
    fn new(text: &str) -> Self {
        let values = text
            .split(',')
            .map(|value| value.parse::<u64>().expect("Couldn't parse coordinate"))
            .collect::<Vec<u64>>();
        Coordinate {
            x: *values.first().expect("No first value"),
            y: *values.last().expect("No second value"),
        }
    }
}
