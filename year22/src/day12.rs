use std::collections::HashMap;

use crate::{coord::Coord2, directions::Direction};

pub fn day12(input_lines: &str) -> (String, String) {
    let raw_map = input_lines
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_spot = find_char_coords(&raw_map, &'S');
    let end_spot = find_char_coords(&raw_map, &'E');
    println!(
        "Start at ({},{}), end at ({},{})",
        start_spot.x, start_spot.y, end_spot.x, end_spot.y
    );

    let mut dijkstra_explored_map = HashMap::from([(
        start_spot,
        ExploredSpace {
            distance: 0,
            _direction_reached: None,
        },
    )]);

    let mut newly_added = dijkstra_explored_map.clone();

    // while dijkstra_explored_map.len() < raw_map.len() * raw_map[0].len() {
    while !dijkstra_explored_map.contains_key(&end_spot) {
        let mut newly_newly_added = HashMap::new();
        newly_added.iter().for_each(|(source, current_info)| {
            let current_height = raw_map[source.y as usize][source.x as usize];
            let current_height = if current_height == 'S' {
                'a'
            } else {
                current_height
            };
            [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .iter()
            .for_each(|direction| {
                let candidate_coords = source.sum(&Coord2::movement(direction));
                if candidate_coords.y >= 0
                    && candidate_coords.y < raw_map.len() as i64
                    && candidate_coords.x >= 0
                    && candidate_coords.x < raw_map[candidate_coords.y as usize].len() as i64
                    && !dijkstra_explored_map.contains_key(&candidate_coords)
                {
                    // New location we can consider moving to!
                    let candidate_height =
                        raw_map[candidate_coords.y as usize][candidate_coords.x as usize];
                    let candidate_height = if candidate_height == 'E' {
                        'z'
                    } else {
                        candidate_height
                    };

                    // Rely on ASCII ordering!
                    if candidate_height as usize <= current_height as usize + 1 {
                        // Move here!
                        println!(
                            "Moving from ({},{}) to ({},{}), total distance {}",
                            source.x,
                            source.y,
                            candidate_coords.x,
                            candidate_coords.y,
                            current_info.distance + 1
                        );
                        dijkstra_explored_map.insert(
                            candidate_coords,
                            ExploredSpace {
                                distance: current_info.distance + 1,
                                _direction_reached: Some(*direction),
                            },
                        );
                        newly_newly_added.insert(
                            candidate_coords,
                            ExploredSpace {
                                distance: current_info.distance + 1,
                                _direction_reached: Some(*direction),
                            },
                        );
                    }
                }
            })
        });

        // Replace the newly added list
        newly_added = newly_newly_added;
    }
    // println!("OVERALL MAP ({} entries)", dijkstra_explored_map.len());
    // for location in &dijkstra_explored_map {
    //     println!("({},{}) - distance {}", location.0.x, location.0.y, location.1.distance);
    // }

    let answer1 = dijkstra_explored_map
        .get(&end_spot)
        .expect("No dest found")
        .distance;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn find_char_coords(raw_map: &[Vec<char>], target: &char) -> Coord2 {
    let target_y = raw_map
        .iter()
        .position(|line| line.contains(target))
        .expect("No start found");
    let target_x = raw_map[target_y].iter().position(|c| c == target).unwrap();
    Coord2 {
        x: target_x as i64,
        y: target_y as i64,
    }
}

#[derive(Clone)]
struct ExploredSpace {
    // location: Coord2,
    distance: usize,
    _direction_reached: Option<Direction>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_day12_part1_case1() {
    //     assert_eq!(day12("").0, "0".to_string())
    // }

    // #[test]
    // fn check_day12_part2_case1() {
    //     assert_eq!(day12("").1, "0".to_string())
    // }

    #[test]
    fn check_day12_both_case1() {
        assert_eq!(
            day12(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            ),
            ("31".to_string(), "0".to_string())
        )
    }
}
