use std::collections::{HashMap, HashSet};

// Potential improvements:
// 1. A bunch of the naming isn't great
// 2. The CavernSystem struct isn't really necessary - I added it for Part 2 before I realised I couldn't track whether a small repeat was a available globally, then couldn't be bothered to remove it.
// 3. I suspect there's more string and vec copying around than needed - but not certain of anywhere we could trim it down.
// 4. count_routes_to_end() should really have some more commenting!

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let cave_system = CavernSystem::new(input_lines);

    let start_cavern = cave_system
        .map
        .get("start")
        .expect("Couldn't find the start cavern!");
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("start".to_string());
    let part1 = count_routes_to_end(&cave_system, start_cavern, &visited, false);
    let part2 = count_routes_to_end(&cave_system, start_cavern, &visited, true);

    (part1, part2)
}

fn count_routes_to_end(
    cave_system: &CavernSystem,
    from: &Cavern,
    visited_small: &HashSet<String>,
    small_repeat_available: bool,
) -> u64 {
    from.connected
        .iter()
        .filter_map(|potential_next| {
            if potential_next == "end" {
                Some(1)
            } else if potential_next == "start" // Need to make sure start is never revisited even if we have a small repeat available
                || (!small_repeat_available && visited_small.contains(potential_next))
            {
                None
            } else {
                let next_cavern = cave_system
                    .map
                    .get(potential_next)
                    .expect("Couldn't find cavern");
                let mut new_visited = visited_small.clone();
                let mut new_small_repeat_available = small_repeat_available;
                // Note rely on lazy evaluation here to not insert Large caverns into the visited path
                if next_cavern.size == CavernSize::Small
                    && !new_visited.insert(potential_next.to_string())
                {
                    new_small_repeat_available = false;
                }
                Some(count_routes_to_end(
                    cave_system,
                    next_cavern,
                    &new_visited,
                    new_small_repeat_available,
                ))
            }
        })
        .sum()
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
            CavernSize::Small
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
