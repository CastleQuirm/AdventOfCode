use std::collections::HashMap;

// Potential improvements:
// 1. There could be a bit more overlap between parts 1 and 2 in terms of structs, even though the actual work is quite different.
// 2. Part 1 play_round() could do something a little neater but it's so simple I'm not sure it's worth it.

pub fn day21(input_lines: &[String]) -> (u64, u64) {
    if cfg!(debug_assertions) {
        println!("P1 | P2 | Player 1 Win prob | Total calculated win prob");
        println!("-------------------------------------------------------");
        (1..11).for_each(|i| {
            (1..11).for_each(|j| {
                let mut universe_count = HashMap::new();
                universe_count.insert(
                    GameState {
                        players: [Player { space: i, score: 0 }, Player { space: j, score: 0 }],
                    },
                    1,
                );
                let mut quantum_game = QuantumGame {
                    universe_count,
                    in_progress_universes: 1,
                    player_wins: [0, 0],
                    weighted_wins: [0_f64, 0_f64],
                };
                quantum_game.play_to_completion();
                println!(
                    "{:02} | {:02} | {} | {}",
                    i,
                    j,
                    quantum_game.weighted_wins[0],
                    quantum_game.weighted_wins.iter().sum::<f64>()
                );
            })
        });
    }

    aoc_answers(input_lines)
}

fn aoc_answers(input_lines: &[String]) -> (u64, u64) {
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
    in_progress_universes: u64,
    player_wins: [u64; 2],
    weighted_wins: [f64; 2],
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
            in_progress_universes: 1,
            player_wins: [0, 0],
            weighted_wins: [0_f64, 0_f64],
        }
    }

    fn play_to_completion(&mut self) {
        while self.in_progress_universes > 0 {
            self.play_turn(0);
            self.play_turn(1);
        }
    }

    fn play_turn(&mut self, player: usize) {
        let mut new_universes: HashMap<GameState, u64> = HashMap::new();

        self.universe_count
            .iter()
            .for_each(|(old_state, old_universe_count)| {
                let universe_probability = if cfg!(debug_assertions) {
                    *old_universe_count as f64 / self.in_progress_universes as f64
                } else {
                    0 as f64
                };
                self.in_progress_universes -= old_universe_count;
                [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
                    .iter()
                    .for_each(|(roll, cases)| {
                        let mut new_state = old_state.clone();
                        new_state.players[player].move_piece(*roll);
                        let new_universe_count = cases * old_universe_count;

                        if new_state.players[player].score >= 21 {
                            self.player_wins[player] += new_universe_count;
                            if cfg!(debug_assertions) {
                                self.weighted_wins[player] += (*cases as f64)
                                    * (1_f64 - self.weighted_wins.iter().sum::<f64>())
                                    * universe_probability
                                    / 27_f64;
                            }
                        } else {
                            let other_count = new_universes.entry(new_state).or_insert(0);
                            *other_count += new_universe_count;
                            self.in_progress_universes += new_universe_count;
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
    use super::aoc_answers;

    #[test]
    fn check_day21() {
        let input_lines = "Player 1 starting position: 4
Player 2 starting position: 8"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(aoc_answers(&input_lines), (739785, 444356092776315));
    }
}
