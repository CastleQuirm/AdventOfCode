use std::collections::{HashMap, HashSet};

// Potential improvements:
// 1. A bunch of the naming isn't great
// 2. The CavernSystem struct isn't really necessary - I added it for Part 2 before I realised I couldn't track whether a small repeat was a available globally, then couldn't be bothered to remove it.
// 3. I suspect there's more string and vec copying around than needed - but not certain of anywhere we could trim it down.

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let cave_system = CavernSystem::new(input_lines);

    let start_cavern = cave_system
        .map
        .get("start")
        .expect("Couldn't find the start cavern!");
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("start".to_string());
    count_routes_to_end(&cave_system, start_cavern, &visited, true)
}

// Count the number of possible routes to reach the end from a given cave, based on the small caves we've already
// visited in our path and whether we're still allowed to repeat one of them, by checking every connection out of
// the current cave with recursive calls to this function and summing the results.
fn count_routes_to_end(
    cave_system: &CavernSystem,
    from: &Cavern,
    visited_small: &HashSet<String>,
    small_repeat_available: bool,
) -> (u64, u64) {
    from.connected
        .iter()
        .map(|potential_next| {
            let next_cavern = cave_system
                    .map
                    .get(potential_next)
                    .expect("Couldn't find cavern");
            match next_cavern.size {
                // We've got a complete path. Return a 1 for part2 so we can add this to our sum as we unwind; whether we
                // return a 0 or 1 for part 1 depends on whether we've revisited a small cave on this path (which we can
                // tell by virtue of whether we still would be allowed to visit a small cave).
                CavernSize::End if small_repeat_available => (1, 1),
                CavernSize::End if !small_repeat_available => (0, 1),
                // Paths can't go back through start. Return a 0 to bail on this potential path.
                CavernSize::Start => (0, 0),
                // If the next cavern is small, we've already visited it and we can't re-visit a small cavern at this point,
                // return a 0 to bail on this potential path.
                CavernSize::Small if !small_repeat_available && visited_small.contains(potential_next) => (0, 0),
                // In other cases, continue exploring with recursion (after producing a new updated visited list and
                // updating our small_repeat_available status if applicable).
                _ =>  {
                    let mut new_visited = visited_small.clone();
                    let mut new_small_repeat_available = small_repeat_available;
                    // Note lazy evaluation here means only Small caverns will be inserted in the visited path, which
                    // is a nice optimisation since only those ones need to be there (but also, the code doesn't rely
                    // on this fact, so it's only an optimisation).
                    if next_cavern.size == CavernSize::Small
                        && !new_visited.insert(potential_next.to_string())
                    {
                        new_small_repeat_available = false;
                    }
                    count_routes_to_end(
                        cave_system,
                        next_cavern,
                        &new_visited,
                        new_small_repeat_available,
                    )
                }
            }
        })
        .fold((0, 0), |total, acc| (total.0 + acc.0, total.1 + acc.1))
}

struct CavernSystem {
    map: HashMap<String, Cavern>,
}

impl CavernSystem {
    fn new(input_lines: &[String]) -> Self {
        let mut map: HashMap<String, Cavern> = HashMap::new();

        input_lines.iter().for_each(|line| {
            let caverns = line.split('-').collect::<Vec<&str>>();
            let (cavern_1_name, cavern_2_name) = (caverns[0], caverns[1]);
            CavernSystem::add_cavern_route_to_map(&mut map, cavern_1_name, cavern_2_name);
            CavernSystem::add_cavern_route_to_map(&mut map, cavern_2_name, cavern_1_name);
        });

        CavernSystem { map }
    }

    fn add_cavern_route_to_map(cave_system: &mut HashMap<String, Cavern>, from: &str, dest: &str) {
        match cave_system.get_mut(from) {
            Some(cavern) => cavern.connected.push(dest.to_string()),
            None => {
                let mut new_cavern = Cavern::new(from);
                new_cavern.connected.push(dest.to_string());
                cave_system.insert(from.to_string(), new_cavern);
            }
        }
    }
}

#[derive(Debug)]
struct Cavern {
    connected: Vec<String>,
    size: CavernSize,
}

impl Cavern {
    fn new(name: &str) -> Self {
        let size = if name.to_lowercase() == name {
            match name {
                "start" => CavernSize::Start,
                "end" => CavernSize::End,
                _ => CavernSize::Small
            }
        } else {
            CavernSize::Large
        };

        Cavern {
            connected: Vec::new(),
            size,
        }
    }
}

#[derive(Debug, PartialEq)]
enum CavernSize {
    Large,
    Small,
    Start,
    End,
}

#[cfg(test)]
mod tests {
    use super::day12;

    #[test]
    fn check_day12_simple() {
        let input_lines = "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day12(&input_lines), (10, 36));
    }

    #[test]
    fn check_day12_medium() {
        let input_lines = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day12(&input_lines), (19, 103));
    }

    #[test]
    fn check_day12_complex() {
        let input_lines = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day12(&input_lines), (226, 3509));
    }
}
