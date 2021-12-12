use std::collections::{HashMap, HashSet};

// Potential improvements:
//

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let cave_system = CavernSystem::new(input_lines);

    let start_cavern = cave_system.map.get("start").expect("Couldn't find the start cavern!");
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("start".to_string());
    let part1_answer = count_routes_to_end(&cave_system, start_cavern, &visited);

    (part1_answer, 0)
}

fn count_routes_to_end(cave_system: &CavernSystem, from: &Cavern, visited_small: &HashSet<String>) -> u64 {
    from.connected.iter().filter_map(|potential_next| {
        if potential_next == "end" { Some(1) }
        else if visited_small.contains(potential_next) { None }
        else {
            let next_cavern = cave_system.map.get(potential_next).expect("Couldn't find cavern");
            let mut new_visited = visited_small.clone();
            if next_cavern.size == CavernSize::Small { new_visited.insert(potential_next.to_string()); }
            Some(count_routes_to_end(cave_system, next_cavern, &new_visited))
        }
    }).sum()
}

struct CavernSystem {
    map: HashMap<String, Cavern>,
    small_repeat_available: bool
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
    
        CavernSystem{
            map,
            small_repeat_available: true
        }
    }
    
    fn add_cavern_route_to_map(cave_system: &mut HashMap<String, Cavern>, from: &str, dest: &str) {
        match cave_system.get_mut(from) {
            Some(cavern) => cavern.connected.push(dest.to_string()),
            None => {
                let mut new_cavern = Cavern::new(from);
                new_cavern.connected.push(dest.to_string());
                cave_system.insert(from.to_string(), new_cavern);
            },
        }
    }
}

#[derive(Debug)]
struct Cavern {
    // name: String,
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
            // name: name.to_string(),
            connected: Vec::new(),
            size
        }
    }
}

#[derive(Debug, PartialEq)]
enum CavernSize {
    Large,
    Small
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
