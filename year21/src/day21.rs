use std::collections::HashMap;

// Potential improvements:
//

pub fn day21(input_lines: &[String]) -> (u64, u64) {
    let mut game = Game::new(input_lines);
    let mut part1 = None;
    while part1.is_none() {
        part1 = game.play_round();
    }

    let mut quantum_game = QuantumGame::new(input_lines);
    quantum_game.play_to_completion();

    (part1.unwrap(), quantum_game.max_winning_universes())
}

struct Game {
    players: Vec<Player>,
    number_of_rolls: u64,
}
impl Game {
    fn new(input_lines: &[String]) -> Self {
        Self {
            players: vec![Player::new(&input_lines[0]), Player::new(&input_lines[1])],
            number_of_rolls: 0,
        }
    }
    fn play_round(&mut self) -> Option<u64> {
        self.play_turn(0);
        if self.players[0].score >= 1000 {
            return Some(self.players[1].score * self.number_of_rolls);
        }
        self.play_turn(1);
        if self.players[1].score >= 1000 {
            return Some(self.players[0].score * self.number_of_rolls);
        }
        None
    }
    fn play_turn(&mut self, player: usize) {
        let player = &mut self.players[player];
        // Single roll is "Add one to the number of rolls, %100, move that number of spaces"
        // Three of these is "Add two to the number of rolls, multiply by 3, move that many spaces %10, add one more to the number of rolls"
        player.move_piece(((self.number_of_rolls + 2) * 3) % 10);
        self.number_of_rolls += 3;
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Player {
    space: u64,
    score: u64,
}
impl Player {
    fn new(line: &str) -> Self {
        Self {
            space: line
                .split(": ")
                .last()
                .expect("No string")
                .parse::<u64>()
                .expect("No parse"),
            score: 0,
        }
    }
}

impl Player {
    fn move_piece(&mut self, spaces_moved: u64) {
        self.space = (self.space + spaces_moved) % 10;
        if self.space == 0 {
            self.space = 10
        }
        self.score += self.space;
    }
}

struct QuantumGame {
    universe_count: HashMap<GameState, u64>,
    player_wins: [u64; 2],
}

impl QuantumGame {
    fn new(input_lines: &[String]) -> Self {
        let mut universe_count = HashMap::new();
        universe_count.insert(
            GameState {
                players: [Player::new(&input_lines[0]), Player::new(&input_lines[1])],
            },
            1,
        );
        Self {
            universe_count,
            player_wins: [0, 0],
        }
    }

    fn play_to_completion(&mut self) {
        while self.universe_count.values().sum::<u64>() > 0 {
            self.play_turn(0);
            self.play_turn(1);
            // Player 1 takes a turn
            // For each entry in self.universe_count: map the game state to the seven possible new keys
            // For any where the player score is >= 21, add 1/3/6/7/6/3/1 * old_count to self.player1_wins
            // For all others, entry(new_game_state).or_insert(0) += 1/3/6/7/6/3/1 * old_count

            // Player 2 takes a turn
        }
    }

    fn play_turn(&mut self, player: usize) {
        let mut new_universes: HashMap<GameState, u64> = HashMap::new();
        self.universe_count
            .iter()
            .for_each(|(old_state, old_universe_count)| {
                [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
                    .iter()
                    .for_each(|(roll, cases)| {
                        let mut new_state = old_state.clone();
                        new_state.players[player].move_piece(*roll);
                        let new_universe_count = cases * old_universe_count;

                        if new_state.players[player].score >= 21 {
                            self.player_wins[player] += new_universe_count;
                        } else {
                            let other_count = new_universes.entry(new_state).or_insert(0);
                            *other_count += new_universe_count;
                        }
                    })
            });
        self.universe_count = new_universes;
    }

    fn max_winning_universes(&self) -> u64 {
        self.player_wins[0].max(self.player_wins[1])
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct GameState {
    players: [Player; 2],
}

#[cfg(test)]
mod tests {
    use super::day21;

    #[test]
    fn check_day21() {
        let input_lines = "Player 1 starting position: 4
Player 2 starting position: 8"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day21(&input_lines), (739785, 444356092776315));
    }
}
