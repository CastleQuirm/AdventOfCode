// Potential improvements:
//

use std::{collections::HashSet, str::FromStr};

use grid::coord::Coord3;
use itertools::Itertools;

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    let coords = input_lines[0]
        .iter()
        .map(|line| Coord3::from_str(line).expect("Bad coord"))
        .collect::<Vec<Coord3>>();
    let cables = coords
        .iter()
        .enumerate()
        .combinations(2)
        .map(|combi| Cabling {
            square_dist: combi[0].1.eucl_dist_squared(combi[1].1),
            first: combi[0].0,
            second: combi[1].0,
        })
        .sorted()
        .collect::<Vec<Cabling>>();

    // Create a set of routers to track state.
    let mut routers = input_lines[0]
        .iter()
        .enumerate()
        .map(|(index, _)| Router {
            index,
            conn_type: ConnectionType::Unconnected,
        })
        .collect::<Vec<_>>();

    let part1_conns = if routers.len() == 20 {
        10 // test code
    } else {
        1000 // real input
    };

    // Perform all the connections for part 1.
    cables[0..part1_conns].iter().for_each(|cable| {
        cable.apply_cable(&mut routers);
    });

    // Get the set of network sizes.
    let mut network_sizes = routers
        .iter()
        .filter_map(|r| r.powered_count())
        .collect::<Vec<usize>>();
    network_sizes.sort();
    network_sizes.reverse();
    let answer1 = network_sizes[0..3].iter().product::<usize>();

    // And part2...
    let mut further_cables = cables[part1_conns..].iter();
    let mut cand_cable = cables[part1_conns];
    while routers[0].powered_count() != Some(routers.len()) {
        let next_cable = further_cables.next().expect("Must still have a cable");
        next_cable.apply_cable(&mut routers);
        cand_cable = *next_cable;
    }

    let answer2 = coords[cand_cable.first].x * coords[cand_cable.second].x;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct Cabling {
    square_dist: i64,
    first: usize,
    second: usize,
}

impl Cabling {
    fn apply_cable(&self, routers: &mut [Router]) {
        routers[self.first]
            .clone()
            .connect(&routers[self.second].clone(), routers);
    }
}

#[derive(Clone, Debug)]
struct Router {
    index: usize,
    conn_type: ConnectionType,
}

impl Router {
    fn connect(&self, other: &Router, array: &mut [Router]) {
        // Find the lowest source and label the objects to act on them.
        let (new_source, secondary) = match self.source().cmp(&other.source()) {
            std::cmp::Ordering::Less => (self.source(), other),
            std::cmp::Ordering::Equal => return, // Nothing needs to happen!
            std::cmp::Ordering::Greater => (other.source(), self),
        };

        // Work out what the higher's contents are and redirect them first.
        let merged_routers = match &secondary.conn_type {
            ConnectionType::Unconnected => {
                array[secondary.index].conn_type = ConnectionType::Powered { source: new_source };
                HashSet::from([secondary.index])
            }
            ConnectionType::Source { .. } => secondary.source_becomes_powered(new_source, array),
            ConnectionType::Powered { source } => array[*source]
                .clone()
                .source_becomes_powered(new_source, array),
        };

        // New source needs updating to (a) make sure its a Source and (b) has all the contents of the other
        let updating_source = array.get_mut(new_source).expect("Bad index");
        match &updating_source.conn_type {
            ConnectionType::Unconnected => {
                updating_source.conn_type = ConnectionType::Source {
                    powered: merged_routers,
                }
            }
            ConnectionType::Source {
                powered: already_powered,
            } => {
                updating_source.conn_type = ConnectionType::Source {
                    powered: merged_routers
                        .union(already_powered)
                        .cloned()
                        .collect::<HashSet<_>>(),
                };
            }
            ConnectionType::Powered { .. } => panic!("This shouldn't have happened"),
        }
    }

    fn source_becomes_powered(&self, new_source: usize, array: &mut [Router]) -> HashSet<usize> {
        match &self.conn_type {
            ConnectionType::Source { powered } => {
                powered.iter().for_each(|downstream_ix| {
                    let updated_router = array.get_mut(*downstream_ix).expect("bad router index");
                    assert!(matches!(
                        updated_router.conn_type,
                        ConnectionType::Powered { source: _ }
                    ));
                    updated_router.conn_type = ConnectionType::Powered { source: new_source };
                });
                array
                    .get_mut(self.index)
                    .expect("bad router index")
                    .conn_type = ConnectionType::Powered { source: new_source };
                let mut new_contents = powered.clone();
                new_contents.insert(self.index);
                new_contents
            }
            _ => {
                panic!("Bad caller")
            }
        }
    }

    fn source(&self) -> usize {
        match self.conn_type {
            ConnectionType::Powered { source } => source,
            _ => self.index,
        }
    }

    fn powered_count(&self) -> Option<usize> {
        match &self.conn_type {
            ConnectionType::Source { powered } => Some(powered.len() + 1),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
enum ConnectionType {
    Unconnected,
    Source { powered: HashSet<usize> },
    Powered { source: usize },
}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689", // INPUT STRING
            "40",    // PART 1 RESULT
            "25272", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day08(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
