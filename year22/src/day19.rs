use std::{collections::HashMap, str::FromStr, string::ParseError};

use once_cell::sync::OnceCell;
use regex::Regex;

pub fn day19(input_lines: &str) -> (String, String) {
    let answer1 = input_lines
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap().quality_level())
        .sum::<usize>();

    // Skip part 2 for the test input because it's too slow (as it is for the real input, but that 'has' to run)
    let answer2 = if input_lines.lines().count() > 2 {
        input_lines
            .lines()
            .take(3)
            .map(|line| {
                line.parse::<Blueprint>()
                    .unwrap()
                    .extended_geode_production()
            })
            .product::<usize>()
    } else {
        0
    };
    (format!("{}", answer1), format!("{}", answer2))
}

struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost_ore: usize,
    obsidian_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obsidian: usize,
}

impl Blueprint {
    fn quality_level(&self) -> usize {
        self.develop_snapshots(Snapshot::new(24)) * self.id
    }

    fn extended_geode_production(&self) -> usize {
        self.develop_snapshots(Snapshot::new(32))
    }

    fn develop_snapshots(&self, starting_snapshot: Snapshot) -> usize {
        let material_types = [
            Material::Ore,
            Material::Clay,
            Material::Obsidian,
            Material::Geode,
        ];
        let mut maximum_geode_yield = 0;
        let mut candidate_snapshots = Vec::from([starting_snapshot]);
        let max_ore_per_tick_required = *[
            self.clay_robot_cost,
            self.obsidian_robot_cost_ore,
            self.geode_robot_cost_ore,
        ]
        .iter()
        .max()
        .unwrap();

        // println!("Let's get to work!");

        while !candidate_snapshots.is_empty() {
            let mut next_snapshots = Vec::new();
            candidate_snapshots.iter().for_each(|snapshot| {
                // println!("Progress a snapshot: {:?}", snapshot);
                for next_robot_type in material_types {
                    // println!("Decide on the next robot type: {:?}", next_robot_type);
                    if (next_robot_type == Material::Geode
                        && snapshot.robots(&Material::Obsidian) == 0)
                        || (next_robot_type == Material::Obsidian
                            && snapshot.robots(&Material::Clay) == 0)
                        || (next_robot_type != Material::Geode && snapshot.time_remaining < 4)
                        || (next_robot_type == Material::Ore
                            && snapshot.robots(&Material::Ore) >= max_ore_per_tick_required)
                    {
                        // println!("Skip a robot type");
                        continue;
                    }
                    let new_snapshot = snapshot.run_step(self, &next_robot_type);
                    if new_snapshot.time_remaining == 0 {
                        // Run out of time. Save off the amount of Geode got, if it's a record.
                        // println!("Snapshot finished: new candidate yield {}", new_snapshot.stock(&Material::Geode));
                        maximum_geode_yield =
                            usize::max(maximum_geode_yield, new_snapshot.stock(&Material::Geode));
                    } else {
                        // Save off this step to iterate on.
                        next_snapshots.push(new_snapshot.clone());
                    }
                }
            });

            candidate_snapshots = next_snapshots;
        }
        maximum_geode_yield
    }
}

static BLUEPRINT: OnceCell<Regex> = OnceCell::new();

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BLUEPRINT
            .get_or_init(|| Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap())
            .captures(s)
            .map(|cap| Self {
                id: cap[1].parse::<usize>().unwrap(),
                ore_robot_cost: cap[2].parse::<usize>().unwrap(),
                clay_robot_cost: cap[3].parse::<usize>().unwrap(),
                obsidian_robot_cost_ore: cap[4].parse::<usize>().unwrap(),
                obsidian_robot_cost_clay: cap[5].parse::<usize>().unwrap(),
                geode_robot_cost_ore: cap[6].parse::<usize>().unwrap(),
                geode_robot_cost_obsidian: cap[7].parse::<usize>().unwrap(),
            })
            .expect("Didn't parse"))
    }
}

#[derive(Clone, Debug)]
struct Snapshot {
    time_remaining: usize,
    stock: HashMap<Material, usize>,
    robots: HashMap<Material, usize>,
}

impl Snapshot {
    fn new(time_remaining: usize) -> Self {
        Self {
            time_remaining,
            stock: HashMap::from([
                (Material::Ore, 0),
                (Material::Clay, 0),
                (Material::Obsidian, 0),
                (Material::Geode, 0),
            ]),
            robots: HashMap::from([
                (Material::Ore, 1),
                (Material::Clay, 0),
                (Material::Obsidian, 0),
                (Material::Geode, 0),
            ]),
        }
    }

    fn run_step(&self, blueprint: &Blueprint, next_build: &Material) -> Self {
        // println!("Run a step");
        let mut next_step = self.clone();
        loop {
            next_step = next_step.tick(blueprint, next_build);
            if next_step.robots != self.robots || next_step.time_remaining == 0 {
                return next_step;
            }
        }
    }

    fn tick(&self, blueprint: &Blueprint, next_build: &Material) -> Self {
        // println!("Run a tick, old time_remaining: {}", self.time_remaining);
        assert_ne!(self.time_remaining, 0);
        let mut new_snapshot = self.clone();
        new_snapshot.time_remaining -= 1;
        // println!(" - New time remaining {}", new_snapshot.time_remaining);

        // Check if we can build the matching type
        match next_build {
            Material::Ore => {
                if new_snapshot
                    .spend_stock(&Material::Ore, blueprint.ore_robot_cost)
                    .is_ok()
                {
                    new_snapshot.build_robot(next_build)
                }
            }
            Material::Clay => {
                if new_snapshot
                    .spend_stock(&Material::Ore, blueprint.clay_robot_cost)
                    .is_ok()
                {
                    new_snapshot.build_robot(next_build)
                }
            }
            Material::Obsidian => {
                if new_snapshot
                    .spend_stock(&Material::Clay, blueprint.obsidian_robot_cost_clay)
                    .is_ok()
                {
                    if new_snapshot
                        .spend_stock(&Material::Ore, blueprint.obsidian_robot_cost_ore)
                        .is_ok()
                    {
                        new_snapshot.build_robot(next_build)
                    } else {
                        new_snapshot.add_stock(&Material::Clay, blueprint.obsidian_robot_cost_clay)
                    }
                }
            }
            Material::Geode => {
                if new_snapshot
                    .spend_stock(&Material::Obsidian, blueprint.geode_robot_cost_obsidian)
                    .is_ok()
                {
                    if new_snapshot
                        .spend_stock(&Material::Ore, blueprint.geode_robot_cost_ore)
                        .is_ok()
                    {
                        new_snapshot.build_robot(next_build)
                    } else {
                        new_snapshot
                            .add_stock(&Material::Obsidian, blueprint.geode_robot_cost_obsidian)
                    }
                }
            }
        };

        // Add stock. USE THE ORIGINAL'S ROBOT COUNT
        new_snapshot.add_stock(&Material::Ore, self.robots(&Material::Ore));
        new_snapshot.add_stock(&Material::Clay, self.robots(&Material::Clay));
        new_snapshot.add_stock(&Material::Obsidian, self.robots(&Material::Obsidian));
        new_snapshot.add_stock(&Material::Geode, self.robots(&Material::Geode));

        new_snapshot
    }

    fn stock(&self, material: &Material) -> usize {
        *self.stock.get(material).unwrap()
    }

    fn spend_stock(&mut self, material: &Material, amount: usize) -> Result<(), ()> {
        if self.stock(material) >= amount {
            *self.stock.get_mut(material).unwrap() -= amount;
            Ok(())
        } else {
            Err(())
        }
    }

    fn add_stock(&mut self, material: &Material, amount: usize) {
        *self.stock.get_mut(material).unwrap() += amount;
    }

    fn robots(&self, material: &Material) -> usize {
        *self.robots.get(material).unwrap()
    }

    fn build_robot(&mut self, material: &Material) {
        *self.robots.get_mut(material).unwrap() += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day19_part1_case1() {
        assert_eq!(day19("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.").0, "33".to_string())
    }
}
