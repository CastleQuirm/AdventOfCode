// Potential improvements:
// 1. Parsing code is a bit messy (not least the last line of "oh, we needed to push a card because we were doing it on the new lines and there isn't a blank at the end")
// 2. `check_win()`'s array of lines to look up is a bit sad; it'd be nice to do something programmatic there (but not actually worth it)
// 3. BingoHall probably isn't worth it as a struct! (and if we are keeping it, maybe put the number line into it?)
// 4. There's a lot of muts; it'd be nicer to have fewer.

pub fn day04(input_lines: &[String]) -> (u64, u64) {
    let mut number_line: Vec<u64> = Vec::new();

    // read Bingo cards
    let mut cards: Vec<BingoCard> = Vec::new();

    let mut new_card: Vec<Vec<u64>> = Vec::new();
    for line in input_lines {
        match line.len() {
            0 => {
                match new_card.len() {
                    0 => (),
                    5 => cards.push(BingoCard::new(new_card)),
                    _ => panic!("Bingo card wasn't expected length"),
                }
                new_card = Vec::new();
            }
            14 => {
                let new_line: Vec<u64> = line
                    .split(' ')
                    .filter(|&x| !x.is_empty())
                    .map(|x| {
                        x.parse::<u64>()
                            .expect("Couldn't parse a number for a bingo card")
                    })
                    .collect::<Vec<u64>>();
                new_card.push(new_line);
            }
            _ => {
                number_line = line
                    .split(',')
                    .map(|x| {
                        x.parse::<u64>()
                            .expect("Couldn't parse a number on the first line")
                    })
                    .collect::<Vec<u64>>()
            }
        }
    }
    cards.push(BingoCard::new(new_card));

    let mut hall = BingoHall { cards };

    let mut scoresheets = hall.play_full_game(number_line);
    scoresheets.sort_by(|a, b| a.turn_finished.cmp(&b.turn_finished));

    (
        scoresheets.first().expect("No first sheet finished?").score,
        scoresheets.last().expect("No last sheet finished?").score,
    )
}

struct BingoHall {
    cards: Vec<BingoCard>,
}

impl BingoHall {
    fn play_full_game(&mut self, number_line: Vec<u64>) -> Vec<ScoreCard> {
        let mut scoresheet: Vec<ScoreCard> = Vec::new();
        'outer: for card in &mut self.cards {
            for (turn, draw) in number_line.iter().enumerate() {
                card.mark_number(draw);
                if card.check_win() {
                    scoresheet.push(ScoreCard {
                        turn_finished: turn,
                        score: draw * card.calculate_base_score(),
                    });
                    continue 'outer;
                }
            }
        }
        scoresheet
    }

    // Code written for part 1 where we play all card simultaneously - but we'd need to
    // start dropping elements from set of cards as they complete.
    // fn play_game(&mut self, number_line: Vec<u64>) -> u64 {
    //     for draw in number_line {
    //         let result = self.call_number(&draw);
    //         if let Some(winner) = result {
    //             return winner.calculate_base_score() * draw;
    //         }
    //     }
    //     0
    // }
    // fn call_number(&mut self, number: &u64) -> Option<&BingoCard> {
    //     for card in &mut self.cards {
    //         card.mark_number(number);
    //         if card.check_win() {
    //             return Some(card);
    //         }
    //     }
    //     None
    // }
}

#[derive(Copy, Clone, Debug)]
struct BingoCard {
    cells: [(u64, bool); 25],
}

impl BingoCard {
    fn new(input_values: Vec<Vec<u64>>) -> Self {
        let mut cells = [(0_u64, false); 25];
        for i in 0..input_values.len() {
            for j in 0..input_values[i].len() {
                cells[i * 5 + j].0 = input_values[i][j];
            }
        }
        Self { cells }
    }
    fn mark_number(&mut self, call: &u64) {
        for i in 0..25 {
            match self.cells[i] {
                (val, true) if val == *call => {
                    panic!("We already marked this number on this card!")
                }
                (val, false) if val == *call => {
                    self.cells[i] = (*call, true);
                    return;
                }
                _ => (),
            }
        }
    }
    fn check_win(&self) -> bool {
        let all_lines: [[usize; 5]; 10] = [
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
        ];
        //  [0, 6, 12, 18, 24],
        //  [20, 16, 12, 8, 4]]; // Diagonals don't count!
        for line in all_lines {
            if self.check_line(line) {
                return true;
            }
        }
        false
    }
    fn check_line(&self, line: [usize; 5]) -> bool {
        for index in line {
            if !self.cells[index].1 {
                return false;
            }
        }
        true
    }
    fn calculate_base_score(&self) -> u64 {
        // just the sum of the unscored numbers; caller is responsible for multiplying by the last called number
        self.cells
            .iter()
            .filter_map(|(value, marked)| if *marked { None } else { Some(value) })
            .sum()
    }
}

struct ScoreCard {
    turn_finished: usize,
    score: u64,
}
