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

// So probably a better format overall is to drop the Body struct
// and just have a hash-map: key is name of Body, value is a Vec of the names of the route?
// Distance is then derivable and any given vec is easy?

// Does Vec have an "intersection" method?  If so we could just get that
// and count it (carefully) to get X.distance

// Alternative: Do a full second bit of program.  Re-trace from YOU to COM,
// creating new Vec along the way as a path.  Then start doing the same from SAN,
// but check each element for presence in the YOU-path Vec.

// Oh no I should have written this first

// This is a bad idea

// Keep this one entirely seperate!
