// Potential improvements:
//

use grid::directions::CompassDirection::{self, East};
use grid::directions::Rotation::{Left, Right, Straight};
use grid::{coord::Coord2, Grid};
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};

pub fn day17(input_lines: &[Vec<String>]) -> (String, String) {
    let mut city_costs = Grid::<u64>::from_digits(&input_lines[0]);
    // add a really large valued border so that it'll appear in candidate directions but
    // should never get used before we find the end result (we could also filter it out on getting
    // a cost of this size or above to enter the potential list, or really clean this up by making
    // the Grid cover an enum of type Block { HeatLoss: u64 } / Edge, and only allowing Block type)
    city_costs.add_border(&999999999999);

    let target_coord = Coord2::from_len_pair((input_lines[0][0].len(), input_lines[0].len()))
        .expect("Couldn't create a target coord");

    let answer1 = calculate_answer(&city_costs, &target_coord, false);
    let answer2 = calculate_answer(&city_costs, &target_coord, true);
    (format!("{}", answer1), format!("{}", answer2))
}

fn calculate_answer(city_costs: &Grid<u64>, target_coord: &Coord2, ultra: bool) -> u64 {
    // Djikstra!
    let mut confirmed_routes: HashMap<Node, u64> = HashMap::new();
    let mut candidate_pile: HashMap<u64, HashSet<Node>> = HashMap::from([(
        0,
        HashSet::from([Node {
            loc: Coord2::from((1, 1)),
            // TECHNICALLY this field should be an Option so that this here can be None
            // but this is the only node that ever needs that and we can just hack it
            dir: East,
            steps_since_turn: 0,
        }]),
    )]);

    let mut dist_moved = 0;

    while !candidate_pile.is_empty() {
        if let Some(new_candidates) = &candidate_pile.get(&dist_moved).cloned() {
            // Consider moving each node into the confirmed routes. Note that we don't filter
            // nodes for duplication on entry to the candidate pile so we must do that here.
            // We could do something to filter out if we've already got a cheaper entry to the
            // same node in the same direction with fewer steps, but we don't really need to -
            // its branches will end up pruned soon enough.
            for node in new_candidates {
                if let Vacant(e) = confirmed_routes.entry(*node) {
                    // If this node is the destination, we're done.
                    if node.loc == *target_coord {
                        return dist_moved;
                    }

                    // Otherwise, add this node to the confirmed routes and get the new candidate
                    // nodes.
                    e.insert(dist_moved);

                    // Add the new candidates from this Node.
                    let insert_new_node = |candidate_pile: &mut HashMap<u64, HashSet<Node>>,
                                           rot| {
                        let new_dir = node.dir.rotate(rot);
                        let new_loc = node.loc.compass_sum(&new_dir);
                        let new_cost = dist_moved + city_costs.get(&new_loc);
                        let new_node = Node {
                            loc: new_loc,
                            dir: new_dir,
                            steps_since_turn: if *rot == Straight {
                                node.steps_since_turn + 1
                            } else {
                                1
                            },
                        };
                        candidate_pile.entry(new_cost).or_default().insert(new_node);
                    };

                    // Which are the correct candidates depends on whether these are ultra
                    // crucibles or not.
                    if ultra {
                        // Ultra crucibles can ONLY go straight unless they've moved at least 4,
                        // AND ONLY turn if they've moved 10.  Note that 0 is special cased to
                        // allow turning because that's just used for the starting node where the
                        // direction shouldn't have been fixed yet.
                        if node.steps_since_turn == 0 || node.steps_since_turn >= 4 {
                            insert_new_node(&mut candidate_pile, &Left);
                            insert_new_node(&mut candidate_pile, &Right);
                        }
                        if node.steps_since_turn < 10 {
                            insert_new_node(&mut candidate_pile, &Straight);
                        }
                    } else {
                        // Standard crucibles (part 1) can always turn but can't go more than
                        // 3 in a straight line.
                        insert_new_node(&mut candidate_pile, &Left);
                        insert_new_node(&mut candidate_pile, &Right);
                        if node.steps_since_turn < 3 {
                            insert_new_node(&mut candidate_pile, &Straight);
                        }
                    }
                }
            }
        }
        candidate_pile.remove(&dist_moved);
        dist_moved += 1;
    }
    panic!()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Node {
    loc: Coord2,
    dir: CompassDirection,
    steps_since_turn: u64,
}

#[cfg(test)]
mod tests {
    use super::day17;
    use crate::utils::load_input;

    #[test]
    fn check_day17_case01() {
        full_test(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533", // INPUT STRING
            "102", // PART 1 RESULT
            "94",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day17(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
