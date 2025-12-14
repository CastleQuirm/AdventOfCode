// Potential improvements:
//

use std::collections::{HashMap, HashSet, VecDeque};

pub fn day11(input_lines: &[Vec<String>]) -> (String, String) {
    let server_set = input_lines[0]
        .iter()
        .map(|line| {
            let (name, connections) = line.split_once(": ").unwrap();
            (
                name.to_owned(),
                Server {
                    networks: connections
                        .split_ascii_whitespace()
                        .map(|x| x.to_owned())
                        .collect::<HashSet<String>>(),
                },
            )
        })
        .collect::<HashMap<String, Server>>();

    let answer1 = count_routes(&server_set, "you", "out");
    let answer2 = count_routes(&server_set, "svr", "fft")
        * count_routes(&server_set, "fft", "dac")
        * count_routes(&server_set, "dac", "out")
        + count_routes(&server_set, "svr", "dac")
            * count_routes(&server_set, "dac", "fft")
            * count_routes(&server_set, "fft", "out");
    (format!("{}", answer1), format!("{}", answer2))
}

fn count_routes(server_set: &HashMap<String, Server>, start: &str, end: &str) -> u64 {
    let mut routes_to = server_set
        .keys()
        .map(|k| (k.to_owned(), 0))
        .collect::<HashMap<String, u64>>();
    routes_to.entry(start.to_string()).and_modify(|e| *e += 1);
    routes_to.insert(end.to_string(), 0);

    let mut active_counts = server_set
        .keys()
        .map(|k| (k.to_owned(), 0))
        .collect::<HashMap<String, u64>>();
    active_counts
        .entry(start.to_string())
        .and_modify(|e| *e += 1);
    active_counts.insert(end.to_string(), 0);

    let mut active_routes = VecDeque::from([start]);
    while let Some(orig) = active_routes.pop_front() {
        let count = *active_counts.get(orig).unwrap();
        if let Some(server) = server_set.get(orig) {
            server.networks.iter().for_each(|dest| {
                routes_to.entry(dest.to_owned()).and_modify(|e| *e += count);
                if active_counts.get(dest) == Some(&0) {
                    active_routes.push_back(dest);
                }
                active_counts
                    .entry(dest.clone())
                    .and_modify(|e| *e += count);
            });
        }
        active_counts.entry(orig.to_owned()).and_modify(|e| *e = 0);
    }
    *routes_to.get(end).unwrap()
}

struct Server {
    networks: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::day11;
    use crate::utils::load_input;

    #[test]
    fn check_day11_case01() {
        full_test(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out", // INPUT STRING
            "5", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day11_case02() {
        full_test(
            "you: out
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out", // INPUT STRING
            "1", // PART 1 RESULT
            "2", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day11(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
