// Potential improvements:
//

pub fn day23(input_lines: &[String]) -> (u64, u64) {
    let part1 = OverallState::new(input_lines)
        .experiment()
        .expect("No end result");
    (part1, 0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OverallState {
    a_tunnel: [Option<char>; 2],
    b_tunnel: [Option<char>; 2],
    c_tunnel: [Option<char>; 2],
    d_tunnel: [Option<char>; 2],
    left_tunnel: [Option<char>; 2],
    right_tunnel: [Option<char>; 2],
    ab_gap: Option<char>,
    bc_gap: Option<char>,
    cd_gap: Option<char>,
    // arthopod_locs: [usize; 8],
    // TODO probably add lookup state for the Arthopods themselves
    energy_spent: u64,
}
impl OverallState {
    fn new(input_lines: &[String]) -> Self {
        Self {
            a_tunnel: [input_lines[2].chars().nth(3), input_lines[3].chars().nth(3)],
            b_tunnel: [input_lines[2].chars().nth(5), input_lines[3].chars().nth(5)],
            c_tunnel: [input_lines[2].chars().nth(7), input_lines[3].chars().nth(7)],
            d_tunnel: [input_lines[2].chars().nth(9), input_lines[3].chars().nth(9)],
            left_tunnel: [None; 2],
            right_tunnel: [None; 2],
            ab_gap: None,
            bc_gap: None,
            cd_gap: None,
            energy_spent: 0,
        }
    }

    fn experiment(&self) -> Option<u64> {
        // try every possible move
        //   for each: if it reaches a finished state, provide the energy spent; else recursive call
        // return the min (noting it'll be "None" if there were no possible moves).
        self.next_turns()
            .iter()
            .flat_map(|possible_states| {
                if possible_states.is_finished() {
                    Some(possible_states.energy_spent)
                } else {
                    possible_states.experiment()
                }
            })
            .min()
    }

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

        // Add cases for each of the possible moves from the corridors.
        self.try_move_from_left(&mut possible_states, 1);
        self.try_move_from_left(&mut possible_states, 0);
        self.try_move_from_right(&mut possible_states, 1);
        self.try_move_from_right(&mut possible_states, 0);

        possible_states
    }

    fn try_move_from_left(&self, possible_states: &mut Vec<Self>, pos: usize) {
        if let Some(mover) = self.left_tunnel[pos] {
            let (col_ix, target) = self.matching_tunnel(mover);
            if (pos == 0 || self.left_tunnel[0].is_none())
                && target[0].is_none()
                && (target[1].is_none() || target[1] == Some(mover))
            {
                let mut new_state = self.clone();
                new_state.left_tunnel[pos] = None;
                let pos = pos as u64;
                let cost_per_move = cost_per_move(mover);
                // If we just cost all the moves to the first entry point, we can then just add 1111 at the very end.
                new_state.energy_spent += cost_per_move * (2 + pos + col_ix * 2);

                // As long as the junctions are clear, we're good.
                // We'll just stick the value on the outside cell, and rely on the hallway being empty to judge if it's done.
                match mover {
                    'A' => new_state.a_tunnel[0] = Some('A'),
                    'B' if self.ab_gap.is_none() => new_state.b_tunnel[0] = Some('B'),
                    'C' if self.ab_gap.is_none() && self.bc_gap.is_none() => {
                        new_state.c_tunnel[0] = Some('C')
                    }
                    'D' if self.ab_gap.is_none()
                        && self.bc_gap.is_none()
                        && self.cd_gap.is_none() =>
                    {
                        new_state.d_tunnel[0] = Some('D')
                    }
                    _ => return,
                }
                possible_states.push(new_state);
            }
        }
    }

    fn try_move_from_right(&self, possible_states: &mut Vec<Self>, pos: usize) {
        if let Some(mover) = self.right_tunnel[pos] {
            let (col_ix, target) = self.matching_tunnel(mover);
            if (pos == 0 || self.right_tunnel[0].is_none())
                && target[0].is_none()
                && (target[1].is_none() || target[1] == Some(mover))
            {
                let mut new_state = self.clone();
                new_state.right_tunnel[pos] = None;
                let pos = pos as u64;
                let cost_per_move = cost_per_move(mover);
                // If we just cost all the moves to the first entry point, we can then just add 1111 at the very end.
                new_state.energy_spent += cost_per_move * (2 + pos + (3 - col_ix) * 2);

                // As long as the junctions are clear, we're good.
                // We'll just stick the value on the outside cell, and rely on the hallway being empty to judge if it's done.
                match mover {
                    'A' if self.ab_gap.is_none()
                        && self.bc_gap.is_none()
                        && self.cd_gap.is_none() =>
                    {
                        new_state.a_tunnel[0] = Some('A')
                    }
                    'B' if self.bc_gap.is_none() && self.cd_gap.is_none() => {
                        new_state.b_tunnel[0] = Some('B')
                    }
                    'C' if self.cd_gap.is_none() => new_state.c_tunnel[0] = Some('C'),
                    'D' => new_state.d_tunnel[0] = Some('D'),
                    _ => return,
                }
                possible_states.push(new_state);
            }
        }
    }

    fn is_finished(&self) -> bool {
        // Assert on the corridor being empty - this is true at the very start, but then not again until it ends.
        self.left_tunnel == [None, None]
            && self.right_tunnel == [None, None]
            && self.ab_gap.is_none()
            && self.bc_gap.is_none()
            && self.cd_gap.is_none()
    }

    fn matching_tunnel(&self, char: char) -> (u64, &[Option<char>; 2]) {
        match char {
            'A' => (0, &self.a_tunnel),
            'B' => (1, &self.b_tunnel),
            'C' => (2, &self.c_tunnel),
            'D' => (3, &self.d_tunnel),
            _ => panic!("Unrecognised for matching tunnel"),
        }
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
                a_tunnel: [Some('B'), Some('A')],
                b_tunnel: [Some('C'), Some('D')],
                c_tunnel: [Some('B'), Some('C')],
                d_tunnel: [Some('D'), Some('A')],
                left_tunnel: [None; 2],
                right_tunnel: [None; 2],
                ab_gap: None,
                bc_gap: None,
                cd_gap: None,
                energy_spent: 0
            }
        )
    }
}
