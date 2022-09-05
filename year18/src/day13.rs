// Potential improvements:
//

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet}, fmt::Display,
};

use itertools::Itertools;

use crate::utils::{
    Coord,
    Direction::{self, East, North, South, West},
};

pub fn day13(input_lines: &[Vec<String>]) -> (String, String) {
    // Read the input
    let (map, mut carts) = read_input(&input_lines[0]);

    // Progress ticks until there's a crash
    run_track(&map, &mut carts)
}

fn read_input(input: &[String]) -> (HashMap<Coord, TrackType>, HashMap<Coord, Cart>) {
    let mut map = HashMap::new();
    let mut carts = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        let y = y.try_into().unwrap();
        for (x, space) in line.chars().enumerate() {
            let x = x.try_into().unwrap();
            let coord = Coord { x, y };
            match space {
                '|' => map.insert(coord, TrackType::Vertical),
                '-' => map.insert(coord, TrackType::Horizontal),
                '+' => map.insert(coord, TrackType::Crossroads),
                '/' => {
                    let track = match map.get(&Coord { x, y: y - 1 }) {
                        Some(track) if track.connects_south() => TrackType::CornerNW,
                        _ => TrackType::CornerSE,
                    };
                    map.insert(coord, track)
                }
                '\\' => {
                    let track = match map.get(&Coord { x, y: y - 1 }) {
                        Some(track) if track.connects_south() => TrackType::CornerNE,
                        _ => TrackType::CornerSW,
                    };
                    map.insert(coord, track)
                }
                '^' => {
                    carts.insert(coord, Cart::new(coord, North));
                    map.insert(coord, TrackType::Vertical)
                }
                'v' => {
                    carts.insert(coord, Cart::new(coord, South));
                    map.insert(coord, TrackType::Vertical)
                }
                '>' => {
                    carts.insert(coord, Cart::new(coord, East));
                    map.insert(coord, TrackType::Horizontal)
                }
                '<' => {
                    carts.insert(coord, Cart::new(coord, West));
                    map.insert(coord, TrackType::Horizontal)
                }
                _ => None,
            };
        }
    }

    (map, carts)
}

fn run_track(map: &HashMap<Coord, TrackType>, carts: &mut HashMap<Coord, Cart>) -> (String, String) {
    let mut first_crash = None;

    while carts.len() > 1 {
        tick(map, carts, &mut first_crash);
    }
    let first_crash = first_crash.expect("Nothing crashed?");

    (
        first_crash.to_string(),
        carts.keys().next().map(|c| c.to_string()).unwrap_or("No carts remained".to_owned()),
    )
}

fn tick(
    map: &HashMap<Coord, TrackType>,
    carts: &mut HashMap<Coord, Cart>,
    first_crash: &mut Option<Coord>,
) {
    let mut new_carts: HashMap<Coord, Cart> = HashMap::new();
    let mut ignore_carts = HashSet::new();

    // Order the carts by position and process one at a time.
    for cart_loc in carts.keys().sorted() {
        if ignore_carts.contains(cart_loc) {
            continue;
        }
        let moved_cart = carts.get(cart_loc).unwrap().advance(map);
        let new_position = moved_cart.position;

        // Check if the cart has crashed with any of the as-yet-unmoved carts. This applies if it's moved to a 'bigger' coordinate and
        // the original set of carts contained something there.
        if new_position > *cart_loc && carts.contains_key(&new_position) {
            if first_crash.is_none() {
                *first_crash = Some(new_position);
            }
            // We'll drop the new cart simply by not adding it to the new element. Track the as-yet-unmoved cart to not bother moving it.
            ignore_carts.insert(new_position);
            continue;
        }

        // Check if the cart has crashed with any of the already moved carts.  This is done by trying to insert it into the set of new carts
        // (which we want to do anyway!) and returning the error if there's a conflict.
        let result = new_carts.insert(new_position, moved_cart);
        if result.is_some() {
            if first_crash.is_none() {
                *first_crash = Some(new_position);
            }
            // Simply drop both carts from the new set.
            new_carts.remove(&new_position);
        }
    }

    *carts = new_carts.clone();
}

#[derive(Debug, PartialEq, Eq)]
enum TrackType {
    Horizontal,
    Vertical,
    Crossroads,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
}

impl TrackType {
    fn connects_south(&self) -> bool {
        *self == Self::Vertical
            || *self == Self::Crossroads
            || *self == Self::CornerSE
            || *self == Self::CornerSW
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NextCrossroads {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    position: Coord,
    next_crossroads: NextCrossroads,
    direction: Direction,
}

impl Cart {
    fn new(position: Coord, direction: Direction) -> Self {
        Self {
            position,
            next_crossroads: NextCrossroads::Left,
            direction,
        }
    }

    fn advance(&self, map: &HashMap<Coord, TrackType>) -> Self {
        let new_position = self.position.sum(&self.direction.as_move_delta());
        let new_track = map.get(&new_position).expect("Came off the track!");
        let (new_next_crossroads, new_direction) = match new_track {
            TrackType::Horizontal | TrackType::Vertical => (self.next_crossroads, self.direction),
            TrackType::Crossroads => match self.next_crossroads {
                NextCrossroads::Left => (NextCrossroads::Straight, self.direction.turn_left()),
                NextCrossroads::Straight => (NextCrossroads::Right, self.direction),
                NextCrossroads::Right => (NextCrossroads::Left, self.direction.turn_right()),
            },
            TrackType::CornerNE => match self.direction {
                North | East => unreachable!(),
                South => (self.next_crossroads, East),
                West => (self.next_crossroads, North),
            },
            TrackType::CornerNW => match self.direction {
                North | West => unreachable!(),
                South => (self.next_crossroads, West),
                East => (self.next_crossroads, North),
            },
            TrackType::CornerSE => match self.direction {
                South | East => unreachable!(),
                North => (self.next_crossroads, East),
                West => (self.next_crossroads, South),
            },
            TrackType::CornerSW => match self.direction {
                South | West => unreachable!(),
                North => (self.next_crossroads, West),
                East => (self.next_crossroads, South),
            },
        };
        Self {
            position: new_position,
            next_crossroads: new_next_crossroads,
            direction: new_direction,
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_cmp = self.y.cmp(&other.y);

        if y_cmp == Ordering::Equal {
            self.x.cmp(&other.x)
        } else {
            y_cmp
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

impl Direction {
    fn as_move_delta(&self) -> Coord {
        match *self {
            North => Coord { x: 0, y: -1 },
            East => Coord { x: 1, y: 0 },
            South => Coord { x: 0, y: 1 },
            West => Coord { x: -1, y: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day13;
    use crate::utils::load_input;

    #[test]
    fn check_day13_case01() {
        full_test(
            "/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ", // INPUT STRING
            "7,3", // PART 1 RESULT
            "No carts remained",   // PART 2 RESULT
        )
    }

    #[test]
    fn check_day13_case02() {
        full_test(
            "/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/", // INPUT STRING
            "2,0", // PART 1 RESULT
            "6,4", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day13(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
