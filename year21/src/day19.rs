use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use itertools::Itertools;

// Potential improvements:
// 1. Well, it's very slow (over 4 seconds). There might be some improvements in memory management but mostly I feel it's the
//    iterative searching that needs improvement.
// 2. During development, I wondered about having a set of manhattan distances between the probes per node and looking for overlaps,
//    but need to be careful about duplicate such distances, distances between pairs that aren't both in each patch, and other issues.
// 3. ALSO wow that rotation set. Surely there must be something nicer.

pub fn day19(input_lines: &[String]) -> (u64, u64) {
    // Parse the input
    let mut scanner_set: Vec<Scanner> = Vec::new();
    let mut scanner = Scanner::new();

    for line in input_lines {
        if line.is_empty() {
            scanner_set.push(scanner);
            scanner = Scanner::new();
        } else if !line.contains("scanner") {
            scanner.relative_probes.insert(Coord::parse(line));
        }
    }
    assert_ne!(scanner.relative_probes.len(), 0);
    scanner_set.push(scanner);

    // Mark the first scanner as 0,0,0
    scanner_set[0].absolute_loc = Some(Coord { x: 0, y: 0, z: 0 });
    scanner_set[0].absolute_probes = scanner_set[0].relative_probes.clone();

    let mut completed_scanners: Vec<Scanner> = Vec::new();
    let mut stitched_scanners = vec![scanner_set[0].clone()];
    let mut unstitched_scanners = scanner_set[1..].to_vec();

    while !stitched_scanners.is_empty() {
        let target_scanner = stitched_scanners
            .pop()
            .expect("No available targets to stitch to");
        for other in &mut unstitched_scanners {
            target_scanner.stitch_if_poss(other);
        }
        completed_scanners.push(target_scanner);

        stitched_scanners.append(
            &mut unstitched_scanners
                .iter()
                .filter_map(|scanner| {
                    if scanner.absolute_loc.is_some() {
                        Some(scanner.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Scanner>>(),
        );

        unstitched_scanners = unstitched_scanners
            .iter()
            .filter_map(|scanner| {
                if scanner.absolute_loc.is_none() {
                    Some(scanner.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Scanner>>();
    }

    let part1 = completed_scanners
        .iter()
        .map(|scanner| scanner.absolute_probes.clone())
        .reduce(|probes, additional_probes| {
            probes
                .union(&additional_probes)
                .cloned()
                .collect::<HashSet<Coord>>()
        })
        .expect("Didn't have any scanners?")
        .len() as u64;

    let part2 = completed_scanners
        .iter()
        .combinations(2)
        .map(|pair| {
            pair[0]
                .absolute_loc
                .expect("What?")
                .manhattan_dist(&pair[1].absolute_loc.expect("What squared?"))
        })
        .max()
        .expect("No max?") as u64;

    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Scanner {
    relative_probes: HashSet<Coord>,
    absolute_loc: Option<Coord>,
    absolute_probes: HashSet<Coord>,
}
impl Scanner {
    fn new() -> Self {
        Self {
            relative_probes: HashSet::new(),
            absolute_loc: None,
            absolute_probes: HashSet::new(),
        }
    }

    fn stitch_if_poss(&self, other: &mut Self) -> bool {
        // Pick a probe on one scanner -> pick a probe on the second scanner -> pick one of the twenty-four orientations for the second scanner -> derive second scanner's relative location
        for (index, self_probe) in self.absolute_probes.iter().enumerate() {
            if index + 12 > self.absolute_probes.len() { return false; }
            for other_probe in &other.relative_probes {
                // Work out where the other scanner could be (up to 24 options!) if self is at 0,0,0 and self_probe '=' other_probe.
                let candidate_other_scanner_locs = other_probe
                    .rotations()
                    .iter()
                    .enumerate()
                    .map(|(index, rotated)| (index, *self_probe - *rotated))
                    .collect::<Vec<(usize, Coord)>>();

                // For each of these options, work out the set of coordinates of the second set relative to self's scanner.
                for (rot_ix, candidate_location) in candidate_other_scanner_locs {
                    let others_probes_relative_to_self = other
                        .relative_probes
                        .iter()
                        .map(|probe| probe.rotations()[rot_ix] + candidate_location)
                        .collect::<HashSet<Coord>>();
                    let overlaps = others_probes_relative_to_self
                        .intersection(&self.absolute_probes)
                        .count();
                    assert_ne!(overlaps, 0);
                    if overlaps >= 12 {
                        // We've got a match! Lock it in and drop this search
                        assert_eq!(other.absolute_loc, None);
                        other.absolute_loc = Some(candidate_location);
                        assert!(other.absolute_probes.is_empty());
                        other.absolute_probes = others_probes_relative_to_self;
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}
impl Coord {
    fn parse(line: &str) -> Self {
        let coordinates = line
            .split(',')
            .map(|num| num.parse::<i32>().expect("Couldn't parse"))
            .collect::<Vec<i32>>();
        assert_eq!(coordinates.len(), 3);
        Self {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        }
    }
    fn manhattan_dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
    fn rotations(&self) -> [Self; 24] {
        [
            Self {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            Self {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            Self {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            Self {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            Self {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            Self {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            Self {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            Self {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            Self {
                x: self.z,
                y: -self.y,
                z: self.x,
            },
            Self {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            Self {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
            Self {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
            Self {
                x: self.y,
                y: -self.x,
                z: self.z,
            },
            Self {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            Self {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },
            Self {
                x: self.z,
                y: self.x,
                z: self.y,
            },
            Self {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },
            Self {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            Self {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
            Self {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            Self {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },
            Self {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },
            Self {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
        ]
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day19;

    #[test]
    fn check_day19() {
        let input_lines = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day19(&input_lines), (79, 3621));
    }
}
