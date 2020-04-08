use std::fs;

#[derive(Copy, Clone)]
struct Body<'main> {
    name: &'main str,
    distance: usize,
    focus: &'main str
}

fn main() {
    let orbit_input = fs::read_to_string("input_file.txt")
                                .expect("Failed to read file");
    let orbit_lines: Vec<&str> = orbit_input.lines().collect();
    let com_body = Body {
        name: "COM",
        distance: 0,
        focus: ""
    };

    let mut examine_orbits: Vec<Body> = vec![com_body];
    let mut examined_orbits: Vec<Body> = Vec::new();
    let mut orbit_checksum = 0;

    while !examine_orbits.is_empty() {
        let check_body = examine_orbits.pop().expect("This was just non-empty!");
        examine_orbits.append(&mut find_orbiting(&orbit_lines, &check_body));
        examined_orbits.push(check_body);
        orbit_checksum = orbit_checksum + check_body.distance;
    }

    println!("Total checksum = {}", orbit_checksum);
}

fn find_orbiting<'main>(orbit_input: &Vec<&'main str>, focal_body: &Body) -> Vec<Body<'main>> {
    let mut orbiting_bodies: Vec<Body> = Vec::new();
    for line in orbit_input {
        if line[0..3].to_string() == focal_body.name {
            orbiting_bodies.push(create_body(line, focal_body.distance));
        }
    }
    return orbiting_bodies;
}

fn create_body(line: &str, inner_distance: usize) -> Body {
    Body {
        name: &line[4..7],
        distance: inner_distance + 1,
        focus: &line[0..3]
    }
}



// Want to get SAN.distance + YOU.distance - 2 * (X.distance + 1)
// where X is the furthest out shared point in the pathing to SAN and YOU.
// Two tasks: (1) find a given Body from its name (put them into a hash-map?)
//            (2) identify X (replace .focus with .path, a vector of Body pointers?  Or an ever growing &str?)
