use std::collections::{HashMap, HashSet};

// Potential improvements:
//

pub fn day23(input_lines: &[String]) -> (u64, u64) {
    // let first_state = OverallState::new(input_lines);
    // let possible_states = first_state.next_turns();


    let part1 = calculate_answer(input_lines);


    // let part1 = OverallState::new(input_lines)
    //     .experiment()
    //     .expect("No end result");



    (part1, 0)
}

fn calculate_answer(input_lines: &[String]) -> u64 {
    let mut candidate_heap = AllStates::new(input_lines);
    loop {
        // Pull a cheapest hop
        let (next, cost) = candidate_heap.take_next();
        
        if !candidate_heap.fully_added.insert(next.clone()) {
            continue;
        };
        println!("Considering option of cost {}", cost);

        // If we're done, this is the cheapest route, return it.
        if next.is_finished() { return cost; }

        // Generate the possible next states
        let possible_states = next.next_turns();

        // Fill each of these into the candidate heaps.
        candidate_heap.add_candidates(possible_states);
    }
}

struct AllStates {
    candidates_by_cost: HashMap<u64, Vec<OverallState>>,
    fully_added: HashSet<OverallState>,
}
impl AllStates {
    fn new(input_lines: &[String]) -> Self {
        let mut candidates_by_cost = HashMap::new();
        candidates_by_cost.insert(0, vec![OverallState::new(input_lines)]);
        Self {
            candidates_by_cost,
            fully_added: HashSet::new()
        }
    }
    fn take_next(&mut self) -> (OverallState, u64) {
        let cheapest_next = *self.candidates_by_cost.keys().min().expect("no cheapest next hop"); // Don't call this on an empty list!
        let possible_states = self
            .candidates_by_cost
            .get_mut(&cheapest_next)
            .expect("We just had this key");
        let chosen_state = possible_states.pop().expect("Wasn't a state for this");
        if possible_states.is_empty() {
            self.candidates_by_cost.remove(&cheapest_next);
        }

        (chosen_state, cheapest_next)
    }
    fn add_candidates(&mut self, possible_states: Vec<OverallState>) {
        possible_states.into_iter().for_each(|state| {
            let list_for_cost = self.candidates_by_cost.entry(state.energy_spent).or_insert_with(Vec::new);
            list_for_cost.push(state)
        })
    }
}

#[derive(Clone, Debug, Hash)]
struct OverallState {
    // The rooms are a vec that we push to and pop from - i.e. the last in the vec is the closest to the corridor.
    // We don't care about details of how many things are in each (it's fully deducible).
    rooms: [Vec<char>; 4],
    corridor: [Option<char>; 7],
    energy_spent: u64,
}
impl OverallState {
    fn new(input_lines: &[String]) -> Self {
        Self {
            rooms: [
                vec![
                    input_lines[3].chars().nth(3).expect("No char?"),
                    input_lines[2].chars().nth(3).expect("No char?"),
                ],
                vec![
                    input_lines[3].chars().nth(5).expect("No char?"),
                    input_lines[2].chars().nth(5).expect("No char?"),
                ],
                vec![
                    input_lines[3].chars().nth(7).expect("No char?"),
                    input_lines[2].chars().nth(7).expect("No char?"),
                ],
                vec![
                    input_lines[3].chars().nth(9).expect("No char?"),
                    input_lines[2].chars().nth(9).expect("No char?"),
                ],
            ],
            corridor: [None; 7],
            energy_spent: 0,
        }
    }

    // fn experiment(&self) -> Option<u64> {
    //     // try every possible move
    //     //   for each: if it reaches a finished state, provide the energy spent; else recursive call
    //     // return the min (noting it'll be "None" if there were no possible moves).
    //     self.next_turns()
    //         .iter()
    //         .flat_map(|possible_states| {
    //             if possible_states.is_finished() {
    //                 Some(possible_states.energy_spent)
    //             } else {
    //                 println!("Just once");
    //                 panic!();
    //                 possible_states.experiment()
    //             }
    //         })
    //         .min()
    // }

    fn next_turns(&self) -> Vec<Self> {
        // For each possible arthopod that could move
        //   For each possible place it could move to
        //     Create a new OverallState with that change and the appropriate energy spent

        // An arthopod in a letter column that matches its letter cannot move
        // An arthopod in a gap or a left/right column can only move to its letter column, and can only do so if
        //   - it can reach it
        //   - the first space in the column is None
        //   - the second space in the column is either None or Some('matching letter')
        // An arthopod in a letter column that doesn't match can move:
        //   - only if it is in the first space of the column OR the first space of the column is empty
        //   - could move to its letter column if points for second option apply (it can reach it, column is either [None, None] or [None, Some('match')])
        //   - could move to a space in the side-columns or one of the mid-column gaps if it can reach it and they're empty.

        let mut possible_states: Vec<Self> = Vec::new();

        // Add cases for each of the possible moves from the corridors to rooms.
        // For each filled space in the corridor:
        //   - Is the room in a good state?
        //   - Is the room reachable?
        //   - If both are yes: what's the cost?
        self.corridor
            .iter()
            .enumerate()
            .filter(|(corridor_index, corridor_space)| {
                corridor_space.is_some()
                    && self.can_enter_own_room(corridor_space.unwrap())
                    && self.can_reach_own_room(*corridor_index)
            })
            .for_each(|(corridor_index, arthopod)| {
                let arthopod = arthopod.unwrap();
                let mut new_state = self.clone();
                new_state.corridor[corridor_index] = None;
                new_state.add_arthopod_to_own_room(arthopod);
                new_state.energy_spent += self.cost_from_corridor_to_room(corridor_index);
                possible_states.push(new_state);
                // println!("Consider moving arthopod {} from corridor {}", arthopod, corridor_index);
            });

        // Add cases for each of the possible moves from the rooms to corridors.
        //   - ONLY IF WE HAVEN'T GOT ANY WAY OF GETTING AN ARTHOPOD HOME
        //   - Is any arthopod in it not home?
        //   - What are the set of corridor spaces it can reach?
        if possible_states.is_empty() {
            (0..4)
                .filter(|&room_index| {
                    self.rooms[room_index]
                        .iter()
                        .any(|&arthopod| matching_room(arthopod) != room_index)
                })
                .for_each(|room_index| {
                    (0_usize..7)
                        .filter(|&corridor_index| {
                            self.room_can_reach_corrior(room_index, corridor_index)
                        })
                        .for_each(|corridor_index| {
                            let mut new_state = self.clone();
                            let arthopod = new_state.rooms[room_index].pop().unwrap();
                            new_state.corridor[corridor_index] = Some(arthopod);
                            new_state.energy_spent +=
                                self.cost_from_room_to_corridor(room_index, corridor_index);
                            possible_states.push(new_state);
                            // println!("Consider moving arthopod {} from room {} to corridor {}", arthopod, room_index, corridor_index);
                        })
                });
            }

        possible_states
    }

    fn is_finished(&self) -> bool {
        // Check if the rooms are full and correct.
        self.rooms
            == [
                vec!['A', 'A'],
                vec!['B', 'B'],
                vec!['C', 'C'],
                vec!['D', 'D'],
            ]
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
        assert!(self.rooms[matching_room(arthopod)].len() < 2);
        let move_count_in_room = (1 - self.rooms[matching_room(arthopod)].len()) as u64;

        cost_per_move(arthopod) * (move_count_in_room + move_count_to_room)
    }

    fn cost_from_room_to_corridor(&self, room_index: usize, corridor_index: usize) -> u64 {
        let arthopod = self.rooms[room_index].last().unwrap();
        let move_count_from_room = moves_corridor_to_room_entrance(corridor_index, room_index);
        assert!(!self.rooms[room_index].is_empty());
        assert!(self.rooms[room_index].len() < 3);
        let move_count_in_room = (2 - self.rooms[room_index].len()) as u64;

        cost_per_move(*arthopod) * (move_count_in_room + move_count_from_room)
    }

    fn cost_from_room_to_room(&self, start_room_index: usize, arthopod: char) -> u64 {
        cost_per_move(arthopod) * moves_room_entrance_to_room_entrance(start_room_index, matching_room(arthopod))
    }

    fn remaining_distance_heuristic(&self) -> u64 {
        self.corridor.iter().enumerate().map(|(corridor_space, entry)| {
            if entry.is_some() {
                self.cost_from_corridor_to_room(corridor_space)
            } else { 0 }
        }).sum::<u64>() +
        self.rooms.iter().enumerate().map(|(room_index, room)| {
            room.iter().map(|&arthopod| self.cost_from_room_to_room(room_index, arthopod)).sum::<u64>()
        }).sum::<u64>()
    }
}
impl PartialEq for OverallState {
    fn eq(&self, other: &OverallState) -> bool {
        self.rooms == other.rooms && self.corridor == other.corridor
    }
}
impl Eq for OverallState {}

fn matching_room(char: char) -> usize {
    match char {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Unrecognised for matching tunnel"),
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
    use super::OverallState;

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
        assert_eq!(day23(&input_lines), (12521, 0));
    }

    #[test]
    fn check_day23_initial_state() {
        let input_lines = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(
            OverallState::new(&input_lines),
            OverallState {
                rooms: [
                    vec!['A', 'B'],
                    vec!['D', 'C'],
                    vec!['C', 'B'],
                    vec!['A', 'D']
                ],
                corridor: [None; 7],
                energy_spent: 0
            }
        )
    }
}
