use std::collections::HashMap;

// Potential improvements:
// ...Actually pretty pleased with this one!  It's probably overengineered with the various maps and function calls,
//    and there's a couple of small bits of semi-duplicated code e.g. the gain_energy calls with checks in both
//    charge_octopus() and next() but otherwise I quite like this one.
// That said:
// 1. The above mentioned duplicated code bits
// 2. The amount of cloning may be high; I suspect with lifetimes and struct pointers we could do better.
// 3. The HashMap rather than Vec<Vec<Octopus>> for the map might be relatively slow
// 4. Wanted to create an iterator so that part1 could be map.nth(100) or the like but gave up without trying very hard.

pub fn day11(input_lines: &[String]) -> (u64, u64) {
    let mut octopus_map = OctopusMap::new(input_lines);

    let mut part1_answer: Option<u64> = None;
    let mut part2_answer: Option<u64> = None;
    let mut steps = 0_u64;

    while part1_answer.is_none() || part2_answer.is_none() {
        steps += 1;
        octopus_map.next();
        if steps == 100 {
            part1_answer = Some(octopus_map.total_flashes);
        }
        if octopus_map.last_step_flashes == 100 {
            part2_answer = Some(steps);
        }
    }

    (part1_answer.unwrap(), part2_answer.unwrap())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    i: u64,
    j: u64,
}

#[derive(PartialEq, Eq)]
enum Flash {
    Charging,
    Ready,
    Discharged,
}

struct Octopus {
    location: Coordinate,
    energy: u64,
    flash: Flash,
    neighbors: Vec<Coordinate>,
}

impl Octopus {
    fn new(i: u64, j: u64, energy: u64) -> Self {
        Octopus {
            location: Coordinate { i, j },
            energy,
            flash: Flash::Charging,
            neighbors: Octopus::neighbors(i as i32, j as i32),
        }
    }

    fn gain_energy(&mut self) -> bool {
        self.energy += 1;
        if self.energy == 10 {
            if self.flash != Flash::Charging {
                panic!("Just got Octopus to 10 enery but was in unexpected state!");
            }
            self.flash = Flash::Ready;
            true
        } else {
            false
        }
    }

    fn neighbors(i: i32, j: i32) -> Vec<Coordinate> {
        let min_di = if i == 0 { 0 } else { -1 };
        let max_di = if i == 9 { 1 } else { 2 }; // NOTE: This uses knowledge from the puzzle that the grid is 10*10
        let min_dj = if j == 0 { 0 } else { -1 };
        let max_dj = if j == 9 { 1 } else { 2 }; // NOTE: This uses knowledge from the puzzle that the grid is 10*10

        (min_di..max_di)
            .flat_map(|di| {
                (min_dj..max_dj)
                    .flat_map(|dj| {
                        if di != 0 || dj != 0 {
                            Some(Coordinate {
                                i: (i + di) as u64,
                                j: (j + dj) as u64,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Coordinate>>()
            })
            .collect::<Vec<Coordinate>>()
    }
}

struct OctopusMap {
    map: HashMap<Coordinate, Octopus>,
    ready_to_flash: Vec<Coordinate>,
    last_step_flashes: u64,
    total_flashes: u64,
}

impl OctopusMap {
    fn new(input_lines: &[String]) -> Self {
        let mut map: HashMap<Coordinate, Octopus> = HashMap::new();
        input_lines.iter().enumerate().for_each(|(i, line)| {
            let i = i as u64;
            line.chars().enumerate().for_each(|(j, char)| {
                let j = j as u64;
                let energy = char.to_string().parse::<u64>().expect("Couldn't parse");
                map.insert(Coordinate { i, j }, Octopus::new(i, j, energy));
            })
        });

        OctopusMap {
            map,
            ready_to_flash: Vec::new(),
            last_step_flashes: 0,
            total_flashes: 0,
        }
    }

    fn next(&mut self) {
        // Count how many flashes in this step.
        self.last_step_flashes = 0;

        // Increment every Octopus by 1.
        for octopus in self.map.values_mut() {
            if octopus.gain_energy() {
                self.ready_to_flash.push(octopus.location)
            };
        }

        // Flash until we need flash no more
        self.process_flashes();

        // Discharge the map
        self.discharge_map();

        // Count how many flashes we've had total.
        self.total_flashes += self.last_step_flashes;
    }

    fn process_flashes(&mut self) {
        while !self.ready_to_flash.is_empty() {
            let flashing_octopus = self
                .map
                .get_mut(&self.ready_to_flash.pop().unwrap())
                .expect("No octopus at these coords");
            flashing_octopus.flash = Flash::Discharged;
            self.last_step_flashes += 1;
            for neighbor in flashing_octopus.neighbors.clone() {
                self.charge_octopus(&neighbor);
            }
        }
    }

    fn charge_octopus(&mut self, coords: &Coordinate) {
        let octopus = self
            .map
            .get_mut(coords)
            .expect("Couldn't find octopus at these coords");
        if octopus.gain_energy() {
            self.ready_to_flash.push(octopus.location)
        };
    }

    fn discharge_map(&mut self) {
        for octopus in self.map.values_mut() {
            match octopus.flash {
                Flash::Discharged => {
                    octopus.energy = 0;
                    octopus.flash = Flash::Charging;
                }
                Flash::Charging => {
                    if octopus.energy > 9 {
                        panic!("Octopus was too high energy without a flash!");
                    }
                }
                Flash::Ready => panic!("Octopus was ready when we finished up!"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day11;

    #[test]
    fn check_day11() {
        let input_lines = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day11(&input_lines), (1656, 195));
    }
}
