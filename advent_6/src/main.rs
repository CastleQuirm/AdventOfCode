use std::fs;
use std::collections::HashMap;

fn main() {
    let orbit_input = fs::read_to_string("input_file.txt")
                                .expect("Failed to read file");
    let orbit_lines: Vec<&str> = orbit_input.lines().collect();

    let mut distance_hash = HashMap::new();
    distance_hash.insert(String::from("COM"), 0);

    let mut focus_hash = HashMap::new();

    let mut examine_orbits: Vec<String> = vec![String::from("COM")];
    let mut orbit_checksum = 0;

    while !examine_orbits.is_empty() {
        let check_body = examine_orbits.pop().expect("This was just non-empty!");
        let orbiting_bodies: Vec<String> = find_orbiting(&orbit_lines, &check_body);
        for new_body in orbiting_bodies {
            distance_hash.insert(String::from(&new_body),
                                 distance_hash.get(&check_body).expect("Missing number!") + 1);
            focus_hash.insert(String::from(&new_body),
                              String::from(&check_body));
            orbit_checksum += distance_hash.get(&check_body).expect("Missing number 2!") + 1;
            examine_orbits.push(new_body);
        }
    }

    println!("Part 1: Total checksum = {}", orbit_checksum);

    // Part 2.  Create a vector of the path from YOU to COM.
    let mut your_orbit_chain: Vec<String> = Vec::new();
    let mut current_body = String::from("YOU");
    while current_body.to_string() != "COM" {
        let new_body = focus_hash.get(&current_body).expect("Missing focus").to_string();
        your_orbit_chain.push(String::from(current_body));
        current_body = new_body;
    }

    current_body = String::from("SAN");
    while current_body != "COM" {
        let new_body = focus_hash.get(&current_body).expect("Missing focus").to_string();
        if your_orbit_chain.contains(&current_body) { break; }
        current_body = new_body;
    }

    println!("Shared focus {}", current_body);

    let travel_distance = distance_hash.get(&String::from("YOU")).expect("YOU distance broke") +
                          distance_hash.get(&String::from("SAN")).expect("SAN distance broke") -
                          2 * (1 + distance_hash.get(&current_body).expect("mid-point distance broke"));

     println!("Travel distance {}", travel_distance);
}

fn find_orbiting(orbit_input: &Vec<&str>, focal_body: &String) -> Vec<String> {
    let mut found_orbiting_bodies: Vec<String> = Vec::new();
    for line in orbit_input {
        if line[0..3].to_string() == *focal_body {
            found_orbiting_bodies.push(line[4..7].to_string());
        }
    }
    return found_orbiting_bodies;
}
