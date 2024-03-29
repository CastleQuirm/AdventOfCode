use std::collections::HashMap;

// Potential improvements:
// 1. The Map struct isn't really necessary!
// 2. The mark_spot() function for walking the line updating every point could be nicer:
//  2a. The walk feels like I should be able to do something with an iterator (without defining the whole line up front)
//  2b. Getting the value from the HashMap then overwriting it feels clunky, but not sure there's a better option.
// 3. I wonder if there's any optimisation by not counting a point again even more once it's been hit twice perpendicularly.

pub fn day05(input_lines: &[String]) -> (u64, u64) {
    let mut map = Map::new();
    input_lines.iter().for_each(|line| map.fill_line(line));

    let actual_danger = map.seabed.values().filter(|&danger| danger.all >= 2);
    let part2 = actual_danger.clone().count() as u64;
    let initial_danger = actual_danger.filter(|&danger| danger.perpendicular >= 2);
    let part1 = initial_danger.count() as u64;

    (part1, part2)
}

struct Map {
    seabed: HashMap<Coordinate, DangerCount>,
}

impl Map {
    fn new() -> Self {
        Map {
            seabed: HashMap::new(),
        }
    }

    fn fill_line(&mut self, line: &str) {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        let start = Coordinate::new(coords.first().expect("No first coordinate"));
        let end = Coordinate::new(coords.last().expect("No last coordinate"));
        let diagonal = start.x != end.x && start.y != end.y;

        let direction_x = (end.x - start.x).signum();
        let direction_y = (end.y - start.y).signum();

        let mut next_spot = start;
        while next_spot != end {
            self.mark_spot(&next_spot, diagonal);
            next_spot = Coordinate {
                x: next_spot.x + direction_x,
                y: next_spot.y + direction_y,
            };
        }
        self.mark_spot(&next_spot, diagonal);
    }

    fn mark_spot(&mut self, spot: &Coordinate, diagonal: bool) {
        let current_danger = self.seabed.entry(*spot).or_insert(DangerCount {
            perpendicular: 0,
            all: 0,
        });
        current_danger.all += 1;
        if !diagonal {
            current_danger.perpendicular += 1;
        }
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(text: &str) -> Self {
        let values = text
            .split(',')
            .map(|value| value.parse::<i32>().expect("Couldn't parse coordinate"))
            .collect::<Vec<i32>>();
        Self {
            x: *values.first().expect("No first value"),
            y: *values.last().expect("No second value"),
        }
    }
}

struct DangerCount {
    perpendicular: u64,
    all: u64,
}

#[cfg(test)]
mod tests {
    use super::day05;

    #[test]
    fn check_day05() {
        let input_lines = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day05(&input_lines), (5, 12));
    }
}

// Note: the below was my first (two) attempts at solving the problem, preserved for posterity. As the first point notes: it's VERY slow.
// It's also bugged: the first attempt worked and solved the puzzle, then I tried to improve it and didn't notice I was getting subtly
// different answers for Part 2, so there's an issue somewhere in the attempts to be clever.
// I rewrote using a completely different approach above after deciding the below was always going to be too painful, and the above is about
// 3 orders of magnitude faster (~20ms in release, instead of ~36s). It's also got some nicer code in places e.g. the directions to
// change in when determining all the points along a line (point 2 below).

// Potential improvements:
// 1. THIS IS SLOW. It takes ~36 seconds (SECONDS, NOT MILLI!) to complete.  IN RELEASE MODE
//    - 1a. Obviously wasteful: creating line_segment_collection twice by over-enthusiastic filtering.  Do the filter for part1 and reuse the collection.
//    - 1b. Obviously wasteful: rerunning all the pairwise intersections in Part 1 for Part 2.  These are all still valid!  We could run that just once.
//    That said: Part 1 on its own took ~4 seconds, so best we can gain by removing duplicate work is that.  32 seconds of this is still baked in to Part 2.
//    Much bigger efficiencies will require doing something much more clever/neat than pairwise HashSet intersections.
//    E.g. storing line segments just as their end-points, and writing a SMART (& FAST) function that calculates the intersection.
// 2. Probably obsoleted by the above, but the 9-case handling of LineSeg::new() working out the contained points is ugly and almost certainly reducable
//    (or at least more commonisable)

// UPDATE:
// I tried reimplementing LineSeg and - more importantly - the intersection() function for two line segs, to make things faster.
// Firstly, there's some logic for handling the points cases - but those wouldn't have been long in the first place.
// Secondly, there's an attempt to make the intersection of two lines a bit nicer by just looking at the shorter line and checking every point
// on it - but if the "contains_point" function is no faster than an intersection check where one list is a single element, that doesn't improve things much either!
// And surprise... the time is barely changed (still over 30 seconds).
// ACTUAL IMPROVEMENT would require implementing something genuine for intersection of two non-point lines.  Problem is, there's a lot of options to cover!

// pub fn day05(input_lines: &[String]) -> (u64, u64) {
//     // parse all the line segments
//     let line_segment_collection = input_lines
//         .iter()
//         .map(|line| LineSeg::new(line))
//         .collect::<Vec<LineSeg>>();

//     // for every pair run .intersection() on one of the pair using the other,
//     // then union them all and count the total.
//     let part2 = count_intersections(&line_segment_collection);

//     // For part 1, do the same but without diagonals.  Done after part 2 because the filter does funny things if we don't use into_iter(),
//     // but into_iter() consumes the vec.
//     let part1 = count_intersections(
//         &line_segment_collection
//             .into_iter()
//             .filter(|seg| !seg.diagonal())
//             .collect(),
//     );

//     (part1, part2)
// }

// fn count_intersections(segments: &Vec<LineSeg>) -> u64 {
//     segments
//         .iter()
//         .combinations(2)
//         .map(|pair| pair[0].intersection(pair[1]))
//         .reduce(|overlaps, acc| {
//             overlaps
//                 .union(&acc)
//                 .cloned()
//                 .collect::<HashSet<Coordinate>>()
//         })
//         .expect("Didn't have any combinations to reduce")
//         .len() as u64
// }

// #[derive(Clone)]
// struct LineSeg {
//     start: Coordinate,
//     end: Coordinate,
//     contained: HashSet<Coordinate>,
// }

// impl LineSeg {
//     fn new(line: &str) -> Self {
//         let coords = line.split(" -> ").collect::<Vec<&str>>();
//         let start = Coordinate::new(coords.first().expect("No first coordinate"));
//         let end = Coordinate::new(coords.last().expect("No last coordinate"));
//         let contained = points_contained(&start, &end);

//         Self {
//             start,
//             end,
//             contained,
//         }
//     }

//     fn intersection(&self, other: &Self) -> HashSet<Coordinate> {
//         let mut result_set = HashSet::new();

//         // Check if there's an intersection based on one or both of the lines being a point
//         match (self.point(), other.point()) {
//             (true, true) => {
//                 if self.start == other.start {
//                     assert!(result_set.insert(self.start))
//                 }
//             }
//             (true, false) => {
//                 if other.contains_point(&self.start) {
//                     assert!(result_set.insert(self.start))
//                 }
//             }
//             (false, true) => {
//                 if self.contains_point(&other.start) {
//                     assert!(result_set.insert(other.start))
//                 }
//             }
//             (false, false) => {
//                 if self.len() < other.len() {
//                     for point in &self.contained {
//                         if other.contains_point(point) {
//                             assert!(result_set.insert(point.clone()))
//                         }
//                     }
//                 } else {
//                     for point in &other.contained {
//                         if self.contains_point(point) {
//                             assert!(result_set.insert(point.clone()))
//                         }
//                     }
//                 }
//             }
//         }

//         result_set
//     }

//     fn diagonal(&self) -> bool {
//         self.start.x != self.end.x && self.start.y != self.end.y
//     }

//     fn point(&self) -> bool {
//         self.start == self.end
//     }

//     fn len(&self) -> usize {
//         self.contained.len()
//     }

//     fn contains_point(&self, point: &Coordinate) -> bool {
//         let point_delta_x = point.x as i32 - self.start.x as i32;
//         let point_delta_y = point.y as i32 - self.start.y as i32;
//         let line_delta_x = self.end.x as i32 - self.start.x as i32;
//         let line_delta_y = self.end.y as i32 - self.start.y as i32;

//         match (point_delta_x, point_delta_y) {
//             (0, 0) => true,
//             (0, d) => {
//                 line_delta_x == 0 && ((line_delta_y > d && d > 0) || (line_delta_y < d && d < 0))
//             }
//             (d, 0) => {
//                 line_delta_y == 0 && ((line_delta_x > d && d > 0) || (line_delta_x < d && d < 0))
//             }
//             (dx, dy) => {
//                 dx.abs() == dy.abs()
//                     && dx.signum() == line_delta_x.signum()
//                     && dy.signum() == line_delta_y.signum()
//                     && line_delta_x.abs() > dx.abs()
//             }
//         }
//     }
// }

// fn points_contained(start: &Coordinate, end: &Coordinate) -> HashSet<Coordinate> {
//     let mut contained: HashSet<Coordinate> = [*start].iter().cloned().collect();
//     match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
//         (Ordering::Equal, Ordering::Equal) => (), // Single point line, don't need to add anything else.
//         (Ordering::Equal, Ordering::Less) => {
//             for y in start.y..end.y {
//                 assert!(contained.insert(Coordinate {
//                     x: start.x,
//                     y: y + 1
//                 })) // add 1 to y - we've already added start, and we need to include end.
//             }
//         }
//         (Ordering::Equal, Ordering::Greater) => {
//             for y in end.y..start.y {
//                 assert!(contained.insert(Coordinate { x: start.x, y }))
//                 // we've already got start, so OK to finish early.
//             }
//         }
//         (Ordering::Less, Ordering::Equal) => {
//             for x in start.x..end.x {
//                 assert!(contained.insert(Coordinate {
//                     x: x + 1,
//                     y: start.y
//                 })) // add 1 to x - we've already added start, and we need to include end.
//             }
//         }
//         (Ordering::Greater, Ordering::Equal) => {
//             for x in end.x..start.x {
//                 assert!(contained.insert(Coordinate { x, y: start.y })) // we've already got start, so OK to finish early.
//             }
//         }
//         (Ordering::Less, Ordering::Less) => {
//             for i in 1..(end.x - start.x + 1) {
//                 assert!(contained.insert(Coordinate {
//                     x: start.x + i,
//                     y: start.y + i
//                 }))
//             }
//         }
//         (Ordering::Less, Ordering::Greater) => {
//             for i in 1..(end.x - start.x + 1) {
//                 assert!(contained.insert(Coordinate {
//                     x: start.x + i,
//                     y: start.y - i
//                 }))
//             }
//         }
//         (Ordering::Greater, Ordering::Less) => {
//             for i in 1..(start.x - end.x + 1) {
//                 assert!(contained.insert(Coordinate {
//                     x: start.x - i,
//                     y: start.y + i
//                 }))
//             }
//         }
//         (Ordering::Greater, Ordering::Greater) => {
//             for i in 1..(start.x - end.x + 1) {
//                 assert!(contained.insert(Coordinate {
//                     x: start.x - i,
//                     y: start.y - i
//                 }))
//             }
//         }
//     }
//     contained
// }

// // struct LineSeg {
// //     diagonal: bool,
// //     contains: HashSet<Coordinate>,
// // }

// // impl LineSeg {
// //     fn new(line: &str) -> Self {
// //         let coords = line.split(" -> ").collect::<Vec<&str>>();

// //         let start = Coordinate::new(coords.first().expect("No first coordinate"));
// //         let end = Coordinate::new(coords.last().expect("No last coordinate"));

// //         let diagonal = start.x != end.x && start.y != end.y;

// //         LineSeg { diagonal, contains: points_contained(&start, &end) }
// //     }
// //     fn intersection(&self, other: &Self) -> HashSet<Coordinate> {
// //         self.contains
// //             .intersection(&other.contains)
// //             .cloned()
// //             .collect()
// //     }
// //     fn diagonal(&self) -> bool {
// //         self.diagonal
// //     }
// // }

// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// struct Coordinate {
//     x: u64,
//     y: u64,
// }
// impl Coordinate {
//     fn new(text: &str) -> Self {
//         let values = text
//             .split(',')
//             .map(|value| value.parse::<u64>().expect("Couldn't parse coordinate"))
//             .collect::<Vec<u64>>();
//         Coordinate {
//             x: *values.first().expect("No first value"),
//             y: *values.last().expect("No second value"),
//         }
//     }
// }
