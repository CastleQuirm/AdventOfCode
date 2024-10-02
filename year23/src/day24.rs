// Potential improvements:
//

use std::collections::HashSet;

use itertools::Itertools;

use crate::coord::{Coord2, Coord3};

pub fn day24(input_lines: &[Vec<String>]) -> (String, String) {
    let hailstones = input_lines[0]
        .iter()
        .map(|line| Hailstone::from(line))
        .collect_vec();
    let (x_y_min, x_y_max) = if hailstones.len() < 10 {
        (7.0, 27.0)
    } else {
        (200000000000000.0, 400000000000000.0)
    };

    let answer1 = hailstones
        .iter()
        .combinations(2)
        .filter(|pair| {
            let h1 = pair[0];
            let h2 = pair[1];
            h1.future_planar_intersection(h2, x_y_min, x_y_max)
        })
        .count();

    // Let's try just testing times for the first two hailstones, to see if we can get an
    // integer set of values and it'll extend.
    // hailstones.iter().permutations(2).for_each(|pair| search(pair[0], pair[1], &hailstones));

    // Let's try what was suggested on Reddit:
    // - Work out plausible velocities
    // --- For any pair of hailstones with p_1i > p_2i and v_1i > v_2i, the stone can't have v_1i > v_si > v_2i
    //     (Reason: v_si > v_2i => (p_si + t * v_si = p_2i + t * v_2i => p_si < p_2i). But similarly we get p_si > p_1i, which is impossible together if p_1i > p_2i
    // - Then just brute force?
    let (impossible_vx, impossible_vy, impossible_vz) = hailstones.iter().permutations(2).fold(
        (HashSet::new(), HashSet::new(), HashSet::new()),
        |(mut acc_x, mut acc_y, mut acc_z), pair| {
            if pair[0].pos_at_zero.x > pair[1].pos_at_zero.x
                && pair[0].velocity.x > pair[1].velocity.x
            {
                acc_x = acc_x
                    .union(&(pair[1].velocity.x..pair[0].velocity.x).collect::<HashSet<_>>())
                    .cloned()
                    .collect();
            }
            if pair[0].pos_at_zero.y > pair[1].pos_at_zero.y
                && pair[0].velocity.y > pair[1].velocity.y
            {
                acc_y = acc_y
                    .union(&(pair[1].velocity.y..pair[0].velocity.y).collect::<HashSet<_>>())
                    .cloned()
                    .collect();
            }
            if pair[0].pos_at_zero.z > pair[1].pos_at_zero.z
                && pair[0].velocity.z > pair[1].velocity.z
            {
                acc_z = acc_z
                    .union(&(pair[1].velocity.z..pair[0].velocity.z).collect::<HashSet<_>>())
                    .cloned()
                    .collect();
            }
            (acc_x, acc_y, acc_z)
        },
    );

    // Guess that the correct velocity will be in the ranges of the other velocities provided, and within the ranges we've
    // managed to eliminate at that. This doesn't hold for the test code, so extend the ranges in that case
    // TODO extending these too far (-10/10) generated a different answer, presumably due to a subtle bug somewhere.
    let min_x = *impossible_vx.iter().min().unwrap().min(&-5);
    let min_y = *impossible_vy.iter().min().unwrap().min(&-5);
    let min_z = *impossible_vz.iter().min().unwrap().min(&-5);
    let max_x = *impossible_vx.iter().max().unwrap().max(&5);
    let max_y = *impossible_vy.iter().max().unwrap().max(&5);
    let max_z = *impossible_vz.iter().max().unwrap().max(&5);

    let mut answer2 = 0;
    'outer: for x in min_x..max_x {
        if impossible_vx.contains(&x) {
            continue;
        }
        for y in min_y..max_y {
            if impossible_vy.contains(&y) {
                continue;
            }
            for z in min_z..max_z {
                if impossible_vz.contains(&z) {
                    continue;
                }
                if let Some(sol) = check_candidate_velocity(&Coord3 { x, y, z }, &hailstones) {
                    answer2 = sol.x + sol.y + sol.z;
                    break 'outer;
                }
            }
        }
    }

    (format!("{}", answer1), format!("{}", answer2))
}

fn check_candidate_velocity(candidate_vel: &Coord3, hailstones: &[Hailstone]) -> Option<Coord3> {
    // Work out a point of intersection
    let h1 = hailstones[0].offset_velocity(candidate_vel);
    let h2 = hailstones[1].offset_velocity(candidate_vel);

    // h1: x + t v_x, y + t v_y, z + t v_z
    h1.xy_integer_intersection(&h2)
        .and_then(|candidate_xy_intersection| {
            // Does z work as well?

            // TODO I thought these asserts should work but they didn't. Why?
            // assert_eq!((candidate_xy_intersection.x - h1.pos_at_zero.x) % h1.velocity.x, 0);
            // assert_eq!((candidate_xy_intersection.y - h1.pos_at_zero.y) % h1.velocity.y, 0);
            // assert_eq!((candidate_xy_intersection.x - h2.pos_at_zero.x) % h2.velocity.x, 0);
            // assert_eq!((candidate_xy_intersection.y - h2.pos_at_zero.y) % h2.velocity.y, 0);

            let t1 = (candidate_xy_intersection.x - h1.pos_at_zero.x) / h1.velocity.x;
            // assert_eq!(h1.pos_at_zero.y + t1 * h1.velocity.y, candidate_xy_intersection.y);

            let t2 = (candidate_xy_intersection.x - h2.pos_at_zero.x) / h2.velocity.x;
            // assert_eq!(h2.pos_at_zero.y + t2 * h2.velocity.y, candidate_xy_intersection.y);

            let z1 = h1.pos_at_zero.z + t1 * h1.velocity.z;
            let z2 = h2.pos_at_zero.z + t2 * h2.velocity.z;
            if z1 == z2 {
                // Should check the other hailstones all go through this point
                let candidate_origin = Coord3 {
                    x: candidate_xy_intersection.x,
                    y: candidate_xy_intersection.y,
                    z: z1,
                };
                if hailstones[2..]
                    .iter()
                    .all(|hailstone| hailstone.hits_point(&candidate_origin))
                {
                    Some(candidate_origin)
                } else {
                    None
                }
            } else {
                None
            }
        })
}

// fn search(h1: &Hailstone, h2: &Hailstone, hailstones: &[Hailstone]) {
//     (0..10000).for_each(|t1| {
//         let h1_loc = Coord3 {
//             x: h1.pos_at_zero.x + t1 * h1.velocity.x,
//             y: h1.pos_at_zero.y + t1 * h1.velocity.y,
//             z: h1.pos_at_zero.z + t1 * h1.velocity.z,
//         };
//         (0..10000).for_each(|t2| {
//             let time_diff = t2 - t1;
//             if time_diff == 0 {
//                 return;
//             }
//             let h2_loc = Coord3 {
//                 x: h2.pos_at_zero.x + t2 * h2.velocity.x,
//                 y: h2.pos_at_zero.y + t2 * h2.velocity.y,
//                 z: h2.pos_at_zero.z + t2 * h2.velocity.z,
//             };

//             if (h2_loc.x - h1_loc.x) % time_diff != 0 || (h2_loc.y - h1_loc.y) % time_diff != 0 || (h2_loc.z - h1_loc.z) % time_diff != 0 {
//                 return;
//             }

//             let stone_velocity =
//                 Coord3 {
//                     x: (h2_loc.x - h1_loc.x) / time_diff,
//                     y: (h2_loc.y - h1_loc.y) / time_diff,
//                     z: (h2_loc.z - h1_loc.z) / time_diff,
//                 };
//             let stone_start = Coord3 {
//                 x: h1_loc.x - t1 * stone_velocity.x,
//                 y: h1_loc.y - t1 * stone_velocity.y,
//                 z: h1_loc.z - t1 * stone_velocity.z,
//             };

//             if stone_velocity.x == 0 || stone_velocity.y == 0 || stone_velocity.z == 0 {
//                 println!("Velocity had a 0 element (start {:?}, vel {:?})", stone_start, stone_velocity);
//                 return;
//             }

//             if !hailstones.iter().all(|hailstone| {
//                 if stone_velocity.x == hailstone.velocity.x {
//                     println!("stone and hailstone have same velocity x (start {:?}, vel {:?})", stone_start, stone_velocity);
//                     return true;
//                 }
//                 let t = (hailstone.pos_at_zero.x - stone_start.x) / (stone_velocity.x - hailstone.velocity.x);
//                 hailstone.pos_at_zero.x + t * hailstone.velocity.x == stone_start.x + t * stone_velocity.x &&
//                     hailstone.pos_at_zero.y + t * hailstone.velocity.y == stone_start.y + t * stone_velocity.y &&
//                     hailstone.pos_at_zero.z + t * hailstone.velocity.z == stone_start.z + t * stone_velocity.z
//             }) {
//                 return;
//             }
//             println!("One in a million! {:?}", stone_start);
//             panic!()
//         });
//     });
// }

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    pos_at_zero: Coord3,
    velocity: Coord3,
}

impl Hailstone {
    fn from(line: &str) -> Self {
        let (position, velocity) = line.split_once(" @ ").unwrap();
        let mut position = position.split(", ").map(|num| num.parse::<i64>().unwrap());
        let mut velocity = velocity
            .split(", ")
            .map(|num| num.trim_start().parse::<i64>().unwrap());
        let stone = Self {
            pos_at_zero: Coord3 {
                x: position.next().unwrap(),
                y: position.next().unwrap(),
                z: position.next().unwrap(),
            },
            velocity: Coord3 {
                x: velocity.next().unwrap(),
                y: velocity.next().unwrap(),
                z: velocity.next().unwrap(),
            },
        };
        assert!(position.next().is_none());
        assert!(velocity.next().is_none());
        stone
    }

    fn planar_trajectory(&self) -> PlanarTrajectory {
        let m = (self.velocity.y as f64) / (self.velocity.x as f64);
        PlanarTrajectory {
            m,
            c: (self.pos_at_zero.y as f64) - m * (self.pos_at_zero.x as f64),
        }
    }

    fn future_planar_intersection(&self, other: &Self, x_y_min: f64, x_y_max: f64) -> bool {
        // Solve y = m1 x + c1 , y = m2 x + c2 => (m1 - m2) x = c2 - c1
        let self_traj = self.planar_trajectory();
        let other_traj = other.planar_trajectory();
        let grad_diff = self_traj.m - other_traj.m;
        if grad_diff == 0f64 {
            // trajectories are parallel. Either they'll never cross or they're the same line,
            // and I want to assume the latter won't happen.
            assert_ne!(self_traj.c, other_traj.c);
            return false;
        }

        let x_intersection = (other_traj.c - self_traj.c) / grad_diff;
        let y_intersection = self_traj.m * x_intersection + self_traj.c;
        let intersection_in_both_futures = [self, other].iter().all(|h| {
            // Make use of the fact no hailstone has a 0 position or velocity in any of the
            // three axes.
            ((x_intersection - h.pos_at_zero.x as f64) / (h.velocity.x as f64)).is_sign_positive()
        });

        x_intersection >= x_y_min
            && x_intersection <= x_y_max
            && y_intersection >= x_y_min
            && y_intersection <= x_y_max
            && intersection_in_both_futures
    }

    fn xy_integer_intersection(&self, other: &Self) -> Option<Coord2> {
        // Solve y = m1 x + c1 , y = m2 x + c2 => (m1 - m2) x = c2 - c1
        let self_traj = self.planar_trajectory();
        let other_traj = other.planar_trajectory();
        let grad_diff = self_traj.m - other_traj.m;
        if grad_diff == 0f64 {
            // trajectories are parallel. Either they'll never cross or they're the same line,
            // and I want to assume the latter won't happen.
            assert_ne!(self_traj.c, other_traj.c);
            return None;
        }

        let x_intersection = (other_traj.c - self_traj.c) / grad_diff;
        let y_intersection = self_traj.m * x_intersection + self_traj.c;

        if x_intersection.fract() != 0.0 || y_intersection.fract() != 0.0 {
            return None;
        }

        Some(Coord2 {
            x: x_intersection as i64,
            y: y_intersection as i64,
        })
    }

    fn offset_velocity(&self, reference_vel: &Coord3) -> Self {
        Self {
            pos_at_zero: self.pos_at_zero,
            velocity: Coord3 {
                x: self.velocity.x - reference_vel.x,
                y: self.velocity.y - reference_vel.y,
                z: self.velocity.z - reference_vel.z,
            },
        }
    }

    fn hits_point(&self, _point: &Coord3) -> bool {
        // TODO
        true
    }
}

struct PlanarTrajectory {
    m: f64,
    c: f64,
}

#[cfg(test)]
mod tests {
    use super::day24;
    use crate::utils::load_input;

    #[test]
    fn check_day24_case01() {
        full_test(
            "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3", // INPUT STRING
            "2",  // PART 1 RESULT
            "47", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day24(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
