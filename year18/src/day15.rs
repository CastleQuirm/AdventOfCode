// Potential improvements:
//

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::utils::{Coord, Direction};

pub fn day15(input_lines: &[Vec<String>]) -> (String, String) {
    // Roll for initiative! (Read input)
    let mut game = Game::new(&input_lines[0]);

    // Run combat
    let mut initial_fight = game.clone();
    initial_fight.fight_full();

    // Run combat with increasing strengths until the elves all survive.
    let elf_count = game
        .fighters
        .values()
        .filter(|f| f.team == Team::Elf)
        .count();
    let needed_strength = (4..)
        .find(|new_strength| {
            let mut altered_history = game.clone();
            altered_history.fight_til_elf_death(*new_strength);
            let surviving_elves = altered_history
                .fighters
                .values()
                .filter(|f| f.team == Team::Elf)
                .count();
            surviving_elves == elf_count
        })
        .expect("...how did this finish?");
    game.fight_til_elf_death(needed_strength);

    // Declare answer
    let answer1 = (initial_fight.round - 1)
        * (initial_fight
            .fighters
            .values()
            .map(|f| f.health.max(0))
            .sum::<i64>() as u64);
    let answer2 =
        (game.round - 1) * (game.fighters.values().map(|f| f.health.max(0)).sum::<i64>() as u64);
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Fighter {
    team: Team,
    health: i64,
    position: Coord,
    turns_taken: u64,
    attack_power: i64,
}

impl Fighter {
    fn new(team: Team, position: Coord) -> Self {
        Self {
            team,
            position,
            health: 200,
            turns_taken: 0,
            attack_power: 3,
        }
    }

    fn take_turn(&mut self, game: &mut Game) {
        // Is there an enemy adjacent?  If not, move.
        if !game.enemy_adjacent(&self.position, &self.team) {
            self.hunt_enemy(game);
        }

        // Is there an enemy adjacent?  If so, attack.
        self.attack(game);

        self.turns_taken += 1;
    }

    fn hunt_enemy(&mut self, game: &mut Game) {
        // Get the set of spaces this fighter can move to, tracking which the first move is.
        let mut reachable_spaces: HashMap<Coord, Direction> = HashMap::new();
        let mut considered_spaces: HashSet<Coord> = HashSet::from([self.position]);

        let dist_1 = vec![
            (self.position.plus(0, -1), Direction::North),
            (self.position.plus(-1, 0), Direction::West),
            (self.position.plus(1, 0), Direction::East),
            (self.position.plus(0, 1), Direction::South),
        ]
        .into_iter()
        .filter(|(s, _)| game.is_space_empty(s))
        .collect::<Vec<(Coord, Direction)>>();

        for (step, dir) in dist_1 {
            if game.is_space_empty(&step) {
                if game.enemy_adjacent(&step, &self.team) {
                    // Just move here and be done
                    game.fighters.remove(&self.position);
                    self.position = step;
                    game.fighters.insert(self.position, *self);
                    return;
                } else {
                    // This is just a candidate direction
                    reachable_spaces.insert(step, dir);
                }
            }
        }

        // If we're here, no one move gets us adjacent to an enemy.  Track the spaces we have reached and start looking for the next possible steps.
        // If we didn't have any previous steps, we're done and no move occurs.
        while !reachable_spaces.is_empty() {
            // Store off everywhere we've been, and the last set of locations we reached to take next steps from.
            considered_spaces.extend(reachable_spaces.keys());
            let previous_distance = reachable_spaces.clone();
            reachable_spaces = HashMap::new();

            // Find everywhere that's one away from the previous distances
            let ordered_previous_distance = [
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::South,
            ]
            .iter()
            .map(|direction| {
                previous_distance
                    .iter()
                    .filter_map(|(s, d)| if d == direction { Some((*s, *d)) } else { None })
                    .collect::<HashMap<Coord, Direction>>()
            })
            .collect::<Vec<HashMap<Coord, Direction>>>();
            for steps_with_fixed_first_dir in ordered_previous_distance {
                for (previous_space, first_dir) in steps_with_fixed_first_dir {
                    vec![
                        previous_space.plus(0, -1),
                        previous_space.plus(-1, 0),
                        previous_space.plus(1, 0),
                        previous_space.plus(0, 1),
                    ]
                    .into_iter()
                    .filter(|s| !considered_spaces.contains(s) && game.is_space_empty(s))
                    .for_each(|step| {
                        if !reachable_spaces.keys().contains(&step) {
                            reachable_spaces.insert(step, first_dir);
                        }
                    });
                }
            }

            let target_direction = reachable_spaces
                .iter()
                .filter(|(c, _)| game.enemy_adjacent(c, &self.team))
                .sorted_by(|(c1, _), (c2, _)| {
                    let y_cmp = c1.y.cmp(&c2.y);
                    if y_cmp == Ordering::Equal {
                        c1.x.cmp(&c2.x)
                    } else {
                        y_cmp
                    }
                })
                .next()
                .map(|(_, d)| *d);

            if let Some(direction) = target_direction {
                game.fighters.remove(&self.position);
                self.position = match direction {
                    Direction::North => self.position.plus(0, -1),
                    Direction::East => self.position.plus(1, 0),
                    Direction::South => self.position.plus(0, 1),
                    Direction::West => self.position.plus(-1, 0),
                };
                game.fighters.insert(self.position, *self);
                return;
            }
        }
    }

    fn attack(&mut self, game: &mut Game) {
        // Find the right target - if there is one.
        let target_coord = self
            .position
            .adjacencies()
            .iter()
            .filter_map(|c| {
                if let Some(f) = game.fighters.get(c) {
                    if f.team != self.team {
                        Some(f)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .min_by_key(|f| f.health)
            .map(|f| f.position);

        // Attack!
        if let Some(target) = target_coord {
            let f = game.fighters.remove(&target);
            let mut attacked_fighter = f.unwrap();
            attacked_fighter.health -= self.attack_power;
            // If the target survived, re-add it to the list.
            if attacked_fighter.health > 0 {
                game.fighters.insert(target, attacked_fighter);
            }
        }
    }
}

#[derive(Clone)]
struct Game {
    map: HashMap<Coord, Terrain>,
    fighters: HashMap<Coord, Fighter>,
    round: u64,
    end_battle: bool,
}

impl Game {
    fn new(input: &[String]) -> Self {
        let mut fighters = HashMap::new();
        let mut map = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            let y = y.try_into().unwrap();
            for (x, space) in line.chars().enumerate() {
                let x = x.try_into().unwrap();
                let terrain = if space == '#' {
                    Terrain::Wall
                } else {
                    Terrain::Cave
                };
                let coord = Coord { x, y };
                map.insert(coord, terrain);
                if space == 'E' {
                    let elf = Fighter::new(Team::Elf, coord);
                    fighters.insert(coord, elf);
                } else if space == 'G' {
                    let goblin = Fighter::new(Team::Goblin, coord);
                    fighters.insert(coord, goblin);
                }
            }
        }
        Self {
            map,
            fighters,
            round: 0,
            end_battle: false,
        }
    }

    fn combat_ongoing(&self) -> bool {
        self.fighters.values().any(|f| f.team == Team::Goblin)
            && self.fighters.values().any(|f| f.team == Team::Elf)
    }

    fn fight_til_elf_death(&mut self, strength: i64) {
        for fighter in self.fighters.values_mut() {
            if fighter.team == Team::Elf {
                fighter.attack_power = strength;
            }
        }
        let elf_count = self
            .fighters
            .values()
            .filter(|f| f.team == Team::Elf)
            .count();
        while !self.end_battle {
            self.round += 1;
            self.round();

            if elf_count
                > self
                    .fighters
                    .values()
                    .filter(|f| f.team == Team::Elf)
                    .count()
            {
                // Abandon!
                self.end_battle = true;
            }
        }
    }

    fn fight_full(&mut self) {
        while !self.end_battle {
            self.round += 1;
            self.round();
        }
    }

    fn round(&mut self) {
        let start_of_round_fighter_locs = self
            .fighters
            .iter()
            .sorted_by(|(c1, _), (c2, _)| {
                let y_cmp = c1.y.cmp(&c2.y);
                if y_cmp == Ordering::Equal {
                    c1.x.cmp(&c2.x)
                } else {
                    y_cmp
                }
            })
            .map(|(c, _)| *c)
            .collect::<Vec<Coord>>();

        for loc in start_of_round_fighter_locs {
            if self.combat_ongoing() {
                if let Some(mut fighter) = self.fighters.remove(&loc) {
                    if fighter.turns_taken < self.round {
                        fighter.take_turn(self);
                    }
                    self.fighters.insert(fighter.position, fighter);
                }
            } else {
                self.end_battle = true;
            }
        }
    }

    fn enemy_adjacent(&self, coord: &Coord, own_team: &Team) -> bool {
        coord
            .adjacencies()
            .iter()
            .filter_map(|c| self.fighters.get(c))
            .any(|f| f.team != *own_team)
    }

    fn is_space_empty(&self, space: &Coord) -> bool {
        *self.map.get(space).unwrap() == Terrain::Cave && !self.fighters.contains_key(space)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Team {
    Elf,
    Goblin,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Terrain {
    Wall,
    Cave,
}

impl Coord {
    fn adjacencies(&self) -> Vec<Coord> {
        vec![
            self.plus(0, -1),
            self.plus(-1, 0),
            self.plus(1, 0),
            self.plus(0, 1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::day15;
    use crate::utils::load_input;

    #[test]
    fn check_day15_case01() {
        full_test(
            "#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#   
#######", // INPUT STRING
            "27730", // PART 1 RESULT
            "4988",  // PART 2 RESULT
        )
    }

    #[test]
    fn check_day15_case02() {
        full_test(
            "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
            "36334",
            "29064",
        )
    }

    #[test]
    fn check_day15_case03() {
        full_test(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
            "39514",
            "31284",
        )
    }

    #[test]
    fn check_day15_case04() {
        full_test(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
            "27755",
            "3478",
        )
    }

    #[test]
    fn check_day15_case05() {
        full_test(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
            "28944",
            "6474",
        )
    }

    #[test]
    fn check_day15_case06() {
        full_test(
            "#########  
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
            "18740",
            "1140",
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day15(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
