use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use once_cell::sync::OnceCell;
use regex::Regex;

pub fn day16(input_lines: &str) -> (String, String) {
    let cavern_set = input_lines
        .lines()
        .map(|line| {
            let cavern = line.parse::<Cavern>().unwrap();
            (cavern.name.clone(), cavern)
        })
        .collect::<HashMap<String, Cavern>>();

    let distance_map: HashMap<String, HashMap<String, usize>> = cavern_set
        .iter()
        .filter_map(|(cavern_name, cavern)| {
            if (cavern.flow_rate > 0) || cavern_name == "AA" {
                let destination_map = dijkstra_caverns(cavern_name, &cavern_set);
                Some((cavern_name.clone(), destination_map))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let mut human_only_route_set = Vec::from([Route {
        caverns_visited: HashSet::from(["AA".to_string()]),
        pressure_released: 0,
        time_remaining: 30,
        can_move: true,
        current_location: "AA".to_string(),
    }]);

    while human_only_route_set.iter().any(|route| route.can_move) {
        let new_route_set = human_only_route_set
            .iter()
            .flat_map(|route| extend_route(route, &distance_map, &cavern_set))
            .collect::<Vec<Route>>();

        let current_max = new_route_set
            .iter()
            .map(|route| route.pressure_released)
            .max()
            .unwrap();

        human_only_route_set = new_route_set
            .into_iter()
            .filter(|route| route.can_move || route.pressure_released >= current_max)
            .collect::<Vec<Route>>();
    }

    assert_eq!(human_only_route_set.len(), 1);

    let answer1 = human_only_route_set.first().unwrap().pressure_released;

    let mut both_route_set = Vec::from([Route {
        caverns_visited: HashSet::from(["AA".to_string()]),
        pressure_released: 0,
        time_remaining: 26,
        can_move: true,
        current_location: "AA".to_string(),
    }]);

    // Have the human explore as much as possible
    let mut stopped_routes = Vec::new();
    while both_route_set.iter().any(|route| route.can_move) {
        let new_route_set = both_route_set
            .iter()
            .flat_map(|route| extend_route(route, &distance_map, &cavern_set))
            .collect::<Vec<Route>>();

        let mut newly_stopped_routes = new_route_set
            .clone()
            .into_iter()
            .filter(|route| !route.can_move)
            .collect::<Vec<_>>();
        stopped_routes.append(&mut newly_stopped_routes);
        both_route_set = new_route_set
            .into_iter()
            .filter(|route| route.can_move)
            .collect::<Vec<_>>();
    }

    assert!(both_route_set.is_empty());
    // both_route_set.append(&mut stopped_routes);
    // assert!(!both_route_set.is_empty());
    // assert!(stopped_routes.is_empty());
    let mut answer2 = 0;

    // Depth first search the human routes
    stopped_routes.iter().for_each(|human_route| {
        // Have the elephant explore
        both_route_set = Vec::from([Route {
            caverns_visited: human_route.caverns_visited.clone(),
            pressure_released: human_route.pressure_released,
            time_remaining: 26,
            can_move: true,
            current_location: "AA".to_string(),
        }]);

        while both_route_set.iter().any(|route| route.can_move) {
            let new_route_set = both_route_set
                .iter()
                .flat_map(|route| extend_route(route, &distance_map, &cavern_set))
                .collect::<Vec<Route>>();

            let current_max = new_route_set
                .iter()
                .map(|route| route.pressure_released)
                .max()
                .unwrap();

            both_route_set = new_route_set
                .into_iter()
                .filter(|route| route.can_move || route.pressure_released >= current_max)
                .collect::<Vec<Route>>();
        }
        answer2 = usize::max(both_route_set.first().unwrap().pressure_released, answer2);
        // println!("Candidate answer: {}", answer2);
    });

    // assert_eq!(both_route_set.len(), 1);

    // let answer2 = both_route_set.iter().next().unwrap().pressure_released;
    // let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn extend_route(
    route: &Route,
    distance_map: &HashMap<String, HashMap<String, usize>>,
    caverns: &HashMap<String, Cavern>,
) -> Vec<Route> {
    // Have one route where we just decide to stop moving (important for Part 2, to allow the elephant to have options)
    let mut new_routes = Vec::from([Route {
        caverns_visited: route.caverns_visited.clone(),
        pressure_released: route.pressure_released,
        time_remaining: route.time_remaining,
        can_move: false,
        current_location: route.current_location.clone(),
    }]);

    let mut extended_routes = distance_map
        .get(&route.current_location)
        .unwrap()
        .iter()
        .filter_map(|(dest_cavern, travel_time)| {
            if route.time_remaining <= travel_time + 1
                || route.caverns_visited.contains(dest_cavern)
            {
                None
            } else {
                // Possible new destination!
                let time_remaining = route.time_remaining - (travel_time + 1);
                let mut caverns_visited = route.caverns_visited.clone();
                assert!(caverns_visited.insert(dest_cavern.clone()));
                let cavern = caverns.get(dest_cavern).unwrap();

                let can_move = distance_map.get(dest_cavern).unwrap().iter().any(
                    |(next_dest, next_travel)| {
                        !caverns_visited.contains(next_dest) && time_remaining > next_travel + 1
                    },
                );

                Some(Route {
                    caverns_visited,
                    pressure_released: route.pressure_released + time_remaining * cavern.flow_rate,
                    time_remaining,
                    can_move,
                    current_location: dest_cavern.to_owned(),
                })
            }
        })
        .collect::<Vec<Route>>();
    new_routes.append(&mut extended_routes);
    new_routes
}

fn dijkstra_caverns(
    cavern_name: &str,
    cavern_set: &HashMap<String, Cavern>,
) -> HashMap<String, usize> {
    let mut explored_caverns = HashSet::from([cavern_name.to_string()]);
    let mut cavern_distances = HashMap::new();
    let mut candidate_caverns = cavern_set
        .get(cavern_name)
        .unwrap()
        .direct_connections
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    let mut walked_distance = 0;
    while !candidate_caverns.is_empty() {
        walked_distance += 1;
        let mut next_candidate_caverns = HashSet::new();
        candidate_caverns.iter().for_each(|cavern_name| {
            let cavern = cavern_set.get(cavern_name).unwrap();
            assert!(explored_caverns.insert(cavern_name.to_string()));
            if cavern.flow_rate > 0 {
                cavern_distances.insert(cavern_name.to_string(), walked_distance);
            }
            cavern.direct_connections.iter().for_each(|adjacency| {
                if !explored_caverns.contains(adjacency) && !candidate_caverns.contains(adjacency) {
                    next_candidate_caverns.insert(adjacency.clone());
                }
            });
        });
        candidate_caverns = next_candidate_caverns;
    }
    cavern_distances
}

struct Cavern {
    name: String,
    flow_rate: usize,
    direct_connections: Vec<String>,
}

static RE: OnceCell<Regex> = OnceCell::new();

impl FromStr for Cavern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RE.get_or_init(|| {
            Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z|,|\s]+)")
                .unwrap()
        })
        .captures(s)
        .map(|cap| {
            Self {
                name: cap[1].to_string(),
                flow_rate: cap[2].parse::<usize>().expect("Couldn't parse flow rate"),
                direct_connections: cap[3].split(", ").map(|s| s.to_string()).collect::<Vec<String>>(),
            }
        })
        .ok_or_else(|| "Parse error".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Route {
    caverns_visited: HashSet<String>,
    pressure_released: usize,
    time_remaining: usize,
    can_move: bool,
    current_location: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_day16_part1_case1() {
    //     assert_eq!(day16("").0, "0".to_string())
    // }

    // #[test]
    // fn check_day16_part2_case1() {
    //     assert_eq!(day16("").1, "0".to_string())
    // }

    #[test]
    fn check_day16_both_case1() {
        assert_eq!(
            day16(
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            ),
            ("1651".to_string(), "1707".to_string())
        )
    }
}
