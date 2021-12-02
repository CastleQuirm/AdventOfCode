// Shortcut: manual coding of input.
// Shortcut: ugly lack of function use due to stuggles with mutability
// Unsolved: how do you do part 2?

use std::cmp::Ordering;

fn main() {

    // Real values
    let mut moon_1 = create_moon(3, -6, 6);
    let mut moon_2 = create_moon(10, 7, -9);
    let mut moon_3 = create_moon(-3, -7, 9);
    let mut moon_4 = create_moon(-8, 0, 4);
    let time_steps = 10000000;

    // Test values
    // let mut moon_1 = create_moon(-1, 0, 2);
    // let mut moon_2 = create_moon(2, -10, -7);
    // let mut moon_3 = create_moon(4, -8, 8);
    // let mut moon_4 = create_moon(3, 5, -1);
    // let time_steps = 2772;

    // let major_moons: Vec<&mut Moon> = vec![
    //     &mut moon_1, &mut moon_2, &mut moon_3, &mut moon_4
    // ];

    let mut moon_1_cycle = 0;
    let mut moon_2_cycle = 0;
    let mut moon_3_cycle = 0;
    let mut moon_4_cycle = 0;
    let mut zero_energy_cycle = 0;

    for t in 1..time_steps+1 {
        // time_step(&major_moons);
        let old_energy =
            moon_1.total_energy() + moon_2.total_energy() + moon_3.total_energy() + moon_4.total_energy();
        moon_1.update_velocity(&moon_2);
        moon_1.update_velocity(&moon_3);
        moon_1.update_velocity(&moon_4);
        moon_2.update_velocity(&moon_1);
        moon_2.update_velocity(&moon_3);
        moon_2.update_velocity(&moon_4);
        moon_3.update_velocity(&moon_1);
        moon_3.update_velocity(&moon_2);
        moon_3.update_velocity(&moon_4);
        moon_4.update_velocity(&moon_1);
        moon_4.update_velocity(&moon_2);
        moon_4.update_velocity(&moon_3);
        moon_1.update_position();
        moon_2.update_position();
        moon_3.update_position();
        moon_4.update_position();

        if moon_1.velocity.x == 0 &&
            moon_1.velocity.y == 0 &&
            moon_1.velocity.z == 0 &&
            moon_1_cycle == 0 {
                moon_1_cycle = t;
                println!("Moon 1 position at vel 0: ({}, {}, {})", moon_1.position.x, moon_1.position.y, moon_1.position.z);
            }

        if moon_2.velocity.x == 0 &&
            moon_2.velocity.y == 0 &&
            moon_2.velocity.z == 0 &&
            moon_2_cycle == 0 {
                moon_2_cycle = t;
                println!("Moon 2 position at vel 0: ({}, {}, {})", moon_2.position.x, moon_2.position.y, moon_2.position.z);
            }

        if moon_3.velocity.x == 0 &&
            moon_3.velocity.y == 0 &&
            moon_3.velocity.z == 0 &&
            moon_3_cycle == 0 {
                moon_3_cycle = t;
                println!("Moon 3 position at vel 0: ({}, {}, {})", moon_3.position.x, moon_3.position.y, moon_3.position.z);
            }

        if moon_4.velocity.x == 0 &&
            moon_4.velocity.y == 0 &&
            moon_4.velocity.z == 0 &&
            moon_4_cycle == 0 {
                moon_4_cycle = t;
                println!("Moon 4 position at vel 0: ({}, {}, {})", moon_4.position.x, moon_4.position.y, moon_4.position.z);
            }

        if moon_1.total_energy() + moon_2.total_energy() + moon_3.total_energy() + moon_4.total_energy() == 0 &&
            zero_energy_cycle   == 0 {
            zero_energy_cycle = t;
        }
        let new_energy = moon_1.total_energy() + moon_2.total_energy() + moon_3.total_energy() + moon_4.total_energy();
        println!("System energy at time {}: {}, delta {}",
            t,
            new_energy,
            new_energy - old_energy);
    }

    // println!("Part 1 Answer: {}", major_moons.iter().map(|moon| moon.total_energy()).sum::<i64>());
    println!("Part 1 Answer: {}",
             moon_1.total_energy() + moon_2.total_energy() + moon_3.total_energy() + moon_4.total_energy());

    println!("Moon 1 cycle {}", moon_1_cycle);
    println!("Moon 2 cycle {}", moon_2_cycle);
    println!("Moon 3 cycle {}", moon_3_cycle);
    println!("Moon 4 cycle {}", moon_4_cycle);
    println!("Zero energy cycle {}", zero_energy_cycle);
}

struct Moon {
    position: Triple,
    velocity: Triple,
}
impl Moon {
    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn update_velocity(&mut self, other_moon: &Moon) {
        match self.position.x.cmp(&other_moon.position.x) {
            Ordering::Less => self.velocity.x += 1,
            Ordering::Greater => self.velocity.x -= 1,
            Ordering::Equal => (),
        }
        match self.position.y.cmp(&other_moon.position.y) {
            Ordering::Less => self.velocity.y += 1,
            Ordering::Greater => self.velocity.y -= 1,
            Ordering::Equal => (),
        }
        match self.position.z.cmp(&other_moon.position.z) {
            Ordering::Less => self.velocity.z += 1,
            Ordering::Greater => self.velocity.z -= 1,
            Ordering::Equal => (),
        }
    }

    fn potential_energy(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

struct Triple {
    x: i64,
    y: i64,
    z: i64,
}

fn create_moon(x: i64, y: i64, z: i64) -> Moon {
    Moon {
        position: Triple { x, y, z },
        velocity: Triple { x: 0, y: 0, z: 0 }
    }
}

// fn time_step(moons: &Vec<&mut Moon>) {
//     for moon in moons {
//         for second_moon in moons {
//             moon.update_velocity(&second_moon);
//         }
//     }

//     for moon in moons {
//         moon.update_position();
//     }
// }
