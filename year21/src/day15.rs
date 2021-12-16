use std::collections::HashMap;

// Potential improvements:
// A lot of awkwardly repeated code again, both repeating from other days (I should really make a Coordinate class, and map reader),
// and internally (the direction loop).
// Feels like there might be a nicer way to do some of it? (The Djikstra implementation is probably fine, but the general handling is messy)

pub fn day15(input_lines: &[String]) -> (u64, u64) {
    // Read in Map
    let part_1_risk_map: Vec<Vec<u64>> = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<u64>()
                        .expect("Couldn't parse character")
                })
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    // Build the Part 2 map.
    let part_1_size = part_1_risk_map.len(); // Use the fact it's a square
    let mut part_2_risk_map: Vec<Vec<u64>> = vec![vec![0; part_1_size * 5]; part_1_size * 5];
    for (i, row) in part_2_risk_map.iter_mut().enumerate().take(part_1_size * 5) {
        for (j, val) in row.iter_mut().enumerate().take(part_1_size * 5) {
            let original_map_i = i % part_1_size;
            let original_map_j = j % part_1_size;
            let increment = (i / part_1_size + j / part_1_size) as u64;
            let result = (part_1_risk_map[original_map_i][original_map_j] + increment) % 9;
            let result = if result != 0 { result } else { 9 };
            *val = result;
        }
    }

    (
        calculate_answer(&part_1_risk_map),
        calculate_answer(&part_2_risk_map),
    )
}

fn calculate_answer(risk_map: &[Vec<u64>]) -> u64 {
    let square_size = risk_map.len(); // The puzzle says it's a square cavern.

    // Implement Djikstra (maintain list of candidate next nodes consisting of coords and candidae cost, pick lowest cost, make fixed)
    let mut djikstra_map: Vec<Vec<Option<u64>>> = vec![vec![None; square_size]; square_size];
    let mut candidate_hops = CandidateHeap::new();

    while djikstra_map[square_size - 1][square_size - 1].is_none() {
        // Pull a cheapest hop
        let (next, cost) = candidate_hops.take_next();
        // If we've already done this cell, skip it
        if djikstra_map[next.i][next.j].is_some() {
            continue;
        }
        // Fill into the Djikstra map
        djikstra_map[next.i][next.j] = Some(cost);
        // Determine the costs of anywhere we haven't gone yet
        candidate_hops.add_all_candidates(risk_map, &djikstra_map, &next, cost, square_size);
    }

    // Take value of bottom right corner.
    djikstra_map[square_size - 1][square_size - 1].expect("Didn't get an answer")
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    i: usize,
    j: usize,
}

struct CandidateHeap {
    by_size: HashMap<u64, Vec<Coordinate>>,
}
impl CandidateHeap {
    fn new() -> Self {
        let mut by_size = HashMap::new();
        by_size.insert(0, vec![Coordinate { i: 0, j: 0 }]);
        Self { by_size }
    }
    fn take_next(&mut self) -> (Coordinate, u64) {
        let cheapest_next = *self.by_size.keys().min().expect("no cheapest next hop"); // Don't call this on an empty list!
        let possible_hops = self
            .by_size
            .get_mut(&cheapest_next)
            .expect("We just had this key");
        let chosen_hop = possible_hops.pop().expect("Wasn't a coordinate for this");
        if possible_hops.is_empty() {
            self.by_size.remove(&cheapest_next);
        }

        (chosen_hop, cheapest_next)
    }
    fn add_all_candidates(
        &mut self,
        risk_map: &[Vec<u64>],
        djikstra_map: &[Vec<Option<u64>>],
        next: &Coordinate,
        cost: u64,
        square_size: usize,
    ) {
        if next.i > 0 && djikstra_map[next.i - 1][next.j].is_none() {
            self.add_candidate(
                Coordinate {
                    i: next.i - 1,
                    j: next.j,
                },
                cost + risk_map[next.i - 1][next.j],
            );
        }
        if next.i < square_size - 1 && djikstra_map[next.i + 1][next.j].is_none() {
            self.add_candidate(
                Coordinate {
                    i: next.i + 1,
                    j: next.j,
                },
                cost + risk_map[next.i + 1][next.j],
            );
        }
        if next.j > 0 && djikstra_map[next.i][next.j - 1].is_none() {
            self.add_candidate(
                Coordinate {
                    i: next.i,
                    j: next.j - 1,
                },
                cost + risk_map[next.i][next.j - 1],
            );
        }
        if next.j < square_size - 1 && djikstra_map[next.i][next.j + 1].is_none() {
            self.add_candidate(
                Coordinate {
                    i: next.i,
                    j: next.j + 1,
                },
                cost + risk_map[next.i][next.j + 1],
            );
        }
    }
    fn add_candidate(&mut self, coord: Coordinate, cost: u64) {
        let list_for_cost = self.by_size.entry(cost).or_insert_with(Vec::new);
        list_for_cost.push(coord)
    }
}

#[cfg(test)]
mod tests {
    use super::day15;

    #[test]
    fn check_day15() {
        let input_lines = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day15(&input_lines), (40, 315));
    }
}
