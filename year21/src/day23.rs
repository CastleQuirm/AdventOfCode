use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

// Potential improvements:
// ...there's probably some (function names, comments, neater checks, would be nice to track and print out info about the best solution)
// but it's taken a long time to solve this well and it now runs both parts in under 100ms so...pretty happy with it as it stands now!

pub fn day23(input_lines: &[String]) -> (u64, u64) {
    let part1 = calculate_astar_search_answer(input_lines);

    let mut extended_map = input_lines
        .iter()
        .take(3)
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    extended_map.push("  #D#C#B#A#".to_string());
    extended_map.push("  #D#B#A#C#".to_string());
    let mut remaining_map = input_lines
        .iter()
        .skip(3)
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    extended_map.append(&mut remaining_map);
    let part2 = calculate_astar_search_answer(&extended_map);

    (
        part1.expect("Couldn't find a route"),
        part2.expect("Couldn't find a route in the extended map"),
    )
}

fn calculate_astar_search_answer(input_lines: &[String]) -> Option<u64> {
    // Open Set is the f-score heap - implement manually via a map of Fscores to the set of states
    let mut open_set: BinaryHeap<Reverse<OverallState>> = BinaryHeap::new();
    open_set.push(Reverse(OverallState::new_from_input(input_lines)));
    let mut closed_set: HashSet<LocationState> = HashSet::new();

    while !open_set.is_empty() {
        let next_state = open_set.pop().unwrap().0;

        if closed_set.contains(&next_state.layout) {
            continue;
        } else if next_state.h_cost == 0 {
            assert!(next_state.layout.is_finished());
            return Some(next_state.g_cost);
        }

        let new_candidates = next_state.next_states();
        new_candidates
            .iter()
            .for_each(|candidate| open_set.push(Reverse(candidate.clone())));
        closed_set.insert(next_state.layout);
    }

    // Couldn't find a path
    None
}

#[derive(Clone, Debug)]
struct OverallState {
    layout: LocationState,
    g_cost: u64,
    h_cost: u64,
}
impl OverallState {
    fn new_from_input(input_lines: &[String]) -> Self {
        let layout = LocationState::new(input_lines);
        let h_cost = layout.remaining_distance_heuristic();
        Self {
            layout,
            g_cost: 0,
            h_cost,
        }
    }

    fn next_states(&self) -> Vec<OverallState> {
        let mut possible_states: Vec<Self> = Vec::new();

        // Add cases for each of the possible moves from the corridors to rooms.
        // For each filled space in the corridor:
        //   - Is the room in a good state?
        //   - Is the room reachable?
        //   - If both are yes: what's the cost?
        self.layout
            .corridor
            .iter()
            .enumerate()
            .filter(|(corridor_index, corridor_space)| {
                corridor_space.is_some()
                    && self.layout.can_enter_own_room(corridor_space.unwrap())
                    && self.layout.can_reach_own_room(*corridor_index)
            })
            .for_each(|(corridor_index, arthopod)| {
                let arthopod = arthopod.unwrap();
                let mut new_state = self.clone();
                new_state.layout.corridor[corridor_index] = None;
                new_state.layout.add_arthopod_to_own_room(arthopod);
                new_state.g_cost += self.layout.cost_from_corridor_to_room(corridor_index);
                new_state.h_cost = new_state.layout.remaining_distance_heuristic();
                possible_states.push(new_state);
            });

        // Add cases for each of the possible moves from the rooms to corridors.
        //   - ONLY IF WE HAVEN'T GOT ANY WAY OF GETTING AN ARTHOPOD HOME
        //   - Is any arthopod in it not home?
        //   - What are the set of corridor spaces it can reach AND ARE CURRENTLY EMPTY?
        if possible_states.is_empty() {
            (0..4)
                .filter(|&room_index| {
                    self.layout.rooms[room_index]
                        .iter()
                        .any(|&arthopod| matching_room(arthopod) != room_index)
                })
                .for_each(|room_index| {
                    (0_usize..7)
                        .filter(|&corridor_index| {
                            self.layout
                                .room_can_reach_corrior(room_index, corridor_index)
                                && self.layout.corridor[corridor_index].is_none()
                        })
                        .for_each(|corridor_index| {
                            let mut new_state = self.clone();
                            let arthopod = new_state.layout.rooms[room_index].pop().unwrap();
                            new_state.layout.corridor[corridor_index] = Some(arthopod);
                            new_state.g_cost += self
                                .layout
                                .cost_from_room_to_corridor(room_index, corridor_index);
                            new_state.h_cost = new_state.layout.remaining_distance_heuristic();
                            possible_states.push(new_state);
                        })
                });
        }

        possible_states
    }

    fn f_cost(&self) -> u64 {
        self.g_cost + self.h_cost
    }
}

impl PartialEq for OverallState {
    fn eq(&self, other: &OverallState) -> bool {
        self.f_cost() == other.f_cost()
    }
}
impl Eq for OverallState {}
impl PartialOrd for OverallState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OverallState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_cost().cmp(&other.f_cost())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct LocationState {
    // The rooms are a vec that we push to and pop from - i.e. the last in the vec is the closest to the corridor.
    // We don't care about details of how many things are in each (it's fully deducible).
    rooms: [Vec<char>; 4],
    corridor: [Option<char>; 7],
    filled_room_size: usize,
}
impl LocationState {
    fn new(input_lines: &[String]) -> Self {
        Self {
            rooms: [
                (2..input_lines.len() - 1)
                    .rev()
                    .map(|row| input_lines[row].chars().nth(3).expect("No char?"))
                    .collect::<Vec<char>>(),
                (2..input_lines.len() - 1)
                    .rev()
                    .map(|row| input_lines[row].chars().nth(5).expect("No char?"))
                    .collect::<Vec<char>>(),
                (2..input_lines.len() - 1)
                    .rev()
                    .map(|row| input_lines[row].chars().nth(7).expect("No char?"))
                    .collect::<Vec<char>>(),
                (2..input_lines.len() - 1)
                    .rev()
                    .map(|row| input_lines[row].chars().nth(9).expect("No char?"))
                    .collect::<Vec<char>>(),
            ],
            corridor: [None; 7],
            filled_room_size: input_lines.len() - 3,
        }
    }

    fn is_finished(&self) -> bool {
        // Check if the rooms are full and correct.
        ['A', 'B', 'C', 'D'].iter().all(|&arthopod| {
            let room_index = matching_room(arthopod);
            self.rooms[room_index].len() == self.filled_room_size
                && self.rooms[room_index]
                    .iter()
                    .all(|&in_room| in_room == arthopod)
        })
    }

    fn can_enter_own_room(&self, arthopod: char) -> bool {
        let room = &self.rooms[matching_room(arthopod)];
        room.iter().all(|&inhabitant| inhabitant == arthopod)
    }

    fn can_reach_own_room(&self, corridor_index: usize) -> bool {
        let arthopod = self.corridor[corridor_index].unwrap();
        corridor_indices_between_corridor_and_room(corridor_index, matching_room(arthopod))
            .iter()
            .all(|&index| self.corridor[index].is_none())
    }

    fn room_can_reach_corrior(&self, room_index: usize, corridor_index: usize) -> bool {
        corridor_indices_between_corridor_and_room(corridor_index, room_index)
            .iter()
            .all(|&corridor_space| self.corridor[corridor_space].is_none())
    }

    fn add_arthopod_to_own_room(&mut self, arthopod: char) {
        self.rooms[matching_room(arthopod)].push(arthopod);
    }

    fn cost_from_corridor_to_room(&self, corridor_index: usize) -> u64 {
        let arthopod = self.corridor[corridor_index].unwrap();
        let move_count_to_room =
            moves_corridor_to_room_entrance(corridor_index, matching_room(arthopod));
        assert!(self.rooms[matching_room(arthopod)].len() < self.filled_room_size);
        let move_count_in_room =
            (self.filled_room_size - 1 - self.rooms[matching_room(arthopod)].len()) as u64;

        cost_per_move(arthopod) * (move_count_in_room + move_count_to_room)
    }

    fn cost_from_room_to_corridor(&self, room_index: usize, corridor_index: usize) -> u64 {
        let arthopod = self.rooms[room_index].last().unwrap();
        let move_count_from_room = moves_corridor_to_room_entrance(corridor_index, room_index);
        assert!(!self.rooms[room_index].is_empty());
        assert!(self.rooms[room_index].len() <= self.filled_room_size);
        let move_count_in_room = (self.filled_room_size - self.rooms[room_index].len()) as u64;

        cost_per_move(*arthopod) * (move_count_in_room + move_count_from_room)
    }

    fn cost_from_room_to_room(&self, start_room_index: usize, arthopod: char) -> u64 {
        cost_per_move(arthopod)
            * moves_room_entrance_to_room_entrance(start_room_index, matching_room(arthopod))
    }

    fn remaining_distance_heuristic(&self) -> u64 {
        self.corridor.iter().enumerate().map(|(corridor_space, entry)| {
            if let Some(arthopod) = entry {
                moves_corridor_to_room_entrance(corridor_space, matching_room(*arthopod)) * cost_per_move(*arthopod)
            } else { 0 }
        }).sum::<u64>() +
        self.rooms.iter().enumerate().map(|(room_index, room)| {
            // cost of getting everything into the right room (entrance to entrance)
            room.iter().map(|&arthopod| self.cost_from_room_to_room(room_index, arthopod)).sum::<u64>() +
            // cost of getting everything in the room to the entrance and/or spread out from the entrance
            self.cost_remaining_in_room(room_index)
        }).sum::<u64>()
    }

    fn cost_remaining_in_room(&self, room_index: usize) -> u64 {
        let arthopod_for_room = matching_arthopod(room_index);
        if self.rooms[room_index]
            .iter()
            .all(|&inhabitant| inhabitant == arthopod_for_room)
        {
            // The room is empty or already only contains the correct arthopods.
            assert!(self.rooms[room_index].len() <= self.filled_room_size);
            let spaces_to_fill = (self.filled_room_size - self.rooms[room_index].len()) as u64;
            // Avoid underflow on calculating triangle numbers...
            if spaces_to_fill > 0 {
                cost_per_move(arthopod_for_room) * spaces_to_fill * (spaces_to_fill - 1) / 2
            } else {
                0
            }
        } else {
            // Cost of moving everything in the room to the entrance
            let empty_room = self.rooms[room_index]
                .iter()
                .enumerate()
                .map(|(depth, arthopod)| {
                    (self.filled_room_size - 1 - depth) as u64 * cost_per_move(*arthopod)
                })
                .sum::<u64>();

            // Cost of just filling this room correctly
            let spaces_to_fill = self.filled_room_size as u64;
            assert!(spaces_to_fill > 0);
            let end_fill =
                cost_per_move(arthopod_for_room) * spaces_to_fill * (spaces_to_fill - 1) / 2;

            // Sum the two
            empty_room + end_fill
        }
    }
}

fn matching_room(char: char) -> usize {
    match char {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Unrecognised for matching room"),
    }
}

fn matching_arthopod(room: usize) -> char {
    match room {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => panic!("Unrecognised for matching arthopod"),
    }
}

fn cost_per_move(mover: char) -> u64 {
    match mover {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Unrecognised mover"),
    }
}

fn moves_corridor_to_room_entrance(corridor_index: usize, room_index: usize) -> u64 {
    match (corridor_index, room_index) {
        (0, 0) => 3,
        (1, 0) => 2,
        (2, 0) => 2,
        (3, 0) => 4,
        (4, 0) => 6,
        (5, 0) => 8,
        (6, 0) => 9,
        (0, 1) => 5,
        (1, 1) => 4,
        (2, 1) => 2,
        (3, 1) => 2,
        (4, 1) => 4,
        (5, 1) => 6,
        (6, 1) => 7,
        (0, 2) => 7,
        (1, 2) => 6,
        (2, 2) => 4,
        (3, 2) => 2,
        (4, 2) => 2,
        (5, 2) => 4,
        (6, 2) => 5,
        (0, 3) => 9,
        (1, 3) => 8,
        (2, 3) => 6,
        (3, 3) => 4,
        (4, 3) => 2,
        (5, 3) => 2,
        (6, 3) => 3,
        _ => panic!(),
    }
}

fn corridor_indices_between_corridor_and_room(
    corridor_index: usize,
    room_index: usize,
) -> Vec<usize> {
    match (corridor_index, room_index) {
        (0, 0) => vec![1],
        (1, 0) => vec![],
        (2, 0) => vec![],
        (3, 0) => vec![2],
        (4, 0) => vec![2, 3],
        (5, 0) => vec![2, 3, 4],
        (6, 0) => vec![2, 3, 4, 5],
        (0, 1) => vec![1, 2],
        (1, 1) => vec![2],
        (2, 1) => vec![],
        (3, 1) => vec![],
        (4, 1) => vec![3],
        (5, 1) => vec![3, 4],
        (6, 1) => vec![3, 4, 5],
        (0, 2) => vec![1, 2, 3],
        (1, 2) => vec![2, 3],
        (2, 2) => vec![3],
        (3, 2) => vec![],
        (4, 2) => vec![],
        (5, 2) => vec![4],
        (6, 2) => vec![4, 5],
        (0, 3) => vec![1, 2, 3, 4],
        (1, 3) => vec![2, 3, 4],
        (2, 3) => vec![3, 4],
        (3, 3) => vec![4],
        (4, 3) => vec![],
        (5, 3) => vec![],
        (6, 3) => vec![5],
        _ => panic!(),
    }
}

fn moves_room_entrance_to_room_entrance(start_room: usize, end_room: usize) -> u64 {
    match (start_room, end_room) {
        (0, 0) => 0,
        (0, 1) => 4,
        (0, 2) => 6,
        (0, 3) => 8,
        (1, 0) => 4,
        (1, 1) => 0,
        (1, 2) => 4,
        (1, 3) => 6,
        (2, 0) => 6,
        (2, 1) => 4,
        (2, 2) => 0,
        (2, 3) => 4,
        (3, 0) => 8,
        (3, 1) => 6,
        (3, 2) => 4,
        (3, 3) => 0,
        _ => panic!("Unrecognised room in or out"),
    }
}

#[cfg(test)]
mod tests {
    use super::day23;
    use super::LocationState;

    #[test]
    fn check_day23() {
        let input_lines = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day23(&input_lines), (12521, 44169));
    }

    #[test]
    fn check_day23_input() {
        let input_lines = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(
            LocationState::new(&input_lines),
            LocationState {
                rooms: [
                    vec!['A', 'B'],
                    vec!['D', 'C'],
                    vec!['C', 'B'],
                    vec!['A', 'D']
                ],
                corridor: [None; 7],
                filled_room_size: 2
            }
        )
    }
}
