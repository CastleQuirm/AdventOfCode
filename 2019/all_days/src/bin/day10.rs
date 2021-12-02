// Assumption: the answer to Part 2 is reliant on the answer to Part 1 being over 200.
// Maintainability: very inconsistent use of terminology.  Should improve by consolidating on two concepts: Asteroid (defined by usize co-ords) and Moves (defined by i64 delta co-ords)
// Maintainability: Threw in an infinite loop at the end that definitely will break...so long as there aren't any bugs.
// Efficiency: We perform the same O(n^2) operation twice so that I don't have to bother holding it.
// Uses a crate!  Gcd - greatest common denominator - to simplify our move vecotrs and then manipulate to give LCMs for ordering.

// Possible enhancement - use the compare-angles for the initial find_visible, use a hash-map to combine on angle (what's the key?), use somehow?

use std::fs;
use std::collections::HashMap;
use gcd::Gcd;
use std::cmp::Ordering;

fn main() {
    let string = fs::read_to_string("input/day10.txt").expect("Failed to read file");

    let row_count = string.lines().count();
    let column_count = string.lines().next().expect("").len();

    let mut asteroid_map: HashMap<(usize, usize), Asteroid> = HashMap::new();

    (0..row_count)
        .for_each(|row_ix| (0..column_count)
            .for_each(|col_ix|
                if string.lines().collect::<Vec<&str>>()[row_ix].chars().nth(col_ix).expect("") == '#' {
                    asteroid_map.insert((col_ix, row_ix), Asteroid {
                        x_coord: col_ix,
                        y_coord: row_ix,
                    } );
                }
            )
        );

    let max_visible_asteroids = asteroid_map
        .values()
        .map(|asteroid| find_visible(&asteroid, &asteroid_map).len())
        .max()
        .expect("");
    println!("Part 1 Answer: {}", max_visible_asteroids);
    let nth_destroyed = 200;
    if max_visible_asteroids < nth_destroyed {
        panic!("Part 2 processing will break - fewer than {} visible", nth_destroyed);
    }

    // Bit inefficient having to do find_visible on every asteroid again...
    let central_asteroid: Vec<&Asteroid> = asteroid_map
        .values()
        .filter(|asteroid| find_visible(asteroid, &asteroid_map).len() == max_visible_asteroids)
        .collect();
    if central_asteroid.len() != 1 {
        panic!("Expecting one unique location for the central asteroid");
    }
    let central_asteroid: &Asteroid = central_asteroid.first().expect("");

    // And to be super inefficient, we're going to call find_visible on this
    // asteroid a third time!
    // Apply sort function to this vector
    let mut sorted_targets = find_visible(central_asteroid, &asteroid_map);
    sorted_targets.sort_unstable_by(|a, b| compare_angles(a, b));

    // Take 200th element
    assert!(sorted_targets.len() == max_visible_asteroids);
    let desired_target_vec = sorted_targets[nth_destroyed - 1];

    // Find the first asteroid in that direction
    for n in 1.. {
        let desired_x = (central_asteroid.x_coord as i64 + n * desired_target_vec.0) as usize;
        let desired_y = (central_asteroid.y_coord as i64 + n * desired_target_vec.1) as usize;
        match asteroid_map.get(&(desired_x, desired_y)) {
            Some(_) => {
                println!("Part 2 Answer: {}", desired_x * 100 + desired_y);
                break;
            }
            None => continue,
        }
    }
}

struct Asteroid {
    x_coord: usize,
    y_coord: usize,
}

fn find_visible(asteroid: &Asteroid, asteroid_map: &HashMap<(usize, usize), Asteroid> ) -> Vec<(i64, i64)> {
    let mut look_vectors: Vec<(i64, i64)> = Vec::new();
    asteroid_map
        .keys()
        .map(|&key| (key.0 as i64 - asteroid.x_coord as i64, key.1 as i64 - asteroid.y_coord as i64))
        .map(|vec| gcd_coord(vec))
        .for_each(|vec| if !look_vectors.contains(&vec) && vec != (0, 0) { look_vectors.push(vec); });
    look_vectors
}

fn gcd_coord(coord: (i64, i64)) -> (i64, i64) {
    if coord == (0, 0) {
        (0, 0)
    } else {
        let gcd = (coord.0.abs() as u64).gcd(coord.1.abs() as u64) as i64;
        (coord.0 / gcd, coord.1 / gcd)
    }
}

fn compare_angles(a: &(i64, i64), b: &(i64, i64)) -> Ordering {
    assert!(a.0 != 0 || a.1 != 0);
    assert!(b.0 != 0 || b.1 != 0);
    if a.0 == 0 && a.1  < 0 {
        if b.0 == 0 && b.1 < 0 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    } else if a.0 == 0 {
        assert!(a.1 > 0);
        if b.0 == 0 && b.1 < 0 {
            Ordering::Greater
        } else {
            b.0.cmp(&a.0)
        }
    } else if a.0 > 0 {
        if b.0 == 0 && b.1 < 0 {
            Ordering::Greater
        } else if b.0 <= 0 {
            Ordering::Less
        } else {
            (a.1 * b.0).cmp(&(b.1 * a.0))
        }
    } else {
        assert!(a.0 < 0);
        if b.0 >= 0 {
            Ordering::Greater
        } else {
            (a.1 * b.0).cmp(&(b.1 * a.0))
        }
    }
}
