use std::fs;

fn main() {
    println!("Hello, world!");

    // Steps:
    // 1. Parse the input file into two vectors of directon strings
    //    a. Read input file by lines, split on commas, have strings
    //       with directions
    // 2. Map the direction strings into vectors of duples for each
    //    vector coordinates
    //    a. For each element, extract and match on first character
    //    b. Apply to current position and store as newly pushed duple.
    // 3. Take each pairwise element of each vector in every pair and
    //    check for intersections:
    //    a. Check that one duple changes X co-ordinate and other
    //       changes Y co-ordinate
    //    b. Check that the constant X co-ordinate is within the changing
    //       value, similarly for Y co-ordinate.
    //    c. If this is the case, record the intersection (constant X and
    //       constant Y values).
    //    d. Calculate the Manhattan distance.  If it's smaller than the
    //       recorded value, overwrite it.
    // 4. Report the lowest recorded Manhattan value.

    // 1.
    let mut wire_instructions: Vec<&str> = Vec::new();
    parse_line(&wire_instructions);

}


fn parse_line(wire_instructions: &mut Vec<&str>) {
    let wire_directions = fs::read_to_string("wire_directions.txt")
        .expect("Failed to read file");

    let mut fuel_total = 0;

    wire_instructions = wire_directions.lines().collect();

    // for wire_direction in wire_directions.lines() {
    //     let _module_mass: i32 = match _module_mass.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     let _fuel_value = fuel_for_component(_module_mass);
    //     fuel_total += _fuel_value;
    // }
}
