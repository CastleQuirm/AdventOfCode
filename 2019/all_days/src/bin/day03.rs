use std::fs;

fn main() {
    let string = fs::read_to_string("input/day03.txt").expect("Failed to read file");

    let wire_vecs: Vec<Vec<&str>> = string
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|wire| wire.split(',').collect())
        .collect();
    assert!(wire_vecs.len() == 2);

    let wire_1_path: Vec<(i32, i32, usize)> = parse_wire(&wire_vecs[0]);
    let wire_2_path: Vec<(i32, i32, usize)> = parse_wire(&wire_vecs[1]);

    let mut intersections = Vec::new();
    for i in 0..wire_1_path.len() - 1 {
        for j in 0..wire_2_path.len() - 1 {
            let intersection = find_intersection(&wire_1_path[i..i+2],
                                                 &wire_2_path[j..j+2]);
            if intersection.is_some() {
                intersections.push(intersection.expect(""));
            }
        }
    }

    let min_distance = intersections
        .iter()
        .map(|cross| cross.0.abs() + cross.1.abs())
        .min()
        .expect("");
    println!("Part 1 Answer: {}", min_distance);

    let min_path = intersections
        .iter()
        .map(|cross| cross.2)
        .min()
        .expect("");
    println!("Part 2 Answer: {}", min_path);
}

fn parse_wire(wire: &Vec<&str>) -> Vec<(i32, i32, usize)> {
    let mut vertices: Vec<(i32, i32, usize)> = vec![(0, 0, 0)];
    wire.iter()
        .for_each(|segment| apply_seg(&mut vertices, segment));
    vertices
}

fn apply_seg(vertices: &mut Vec<(i32, i32, usize)>, segment: &str) {
    let dir = segment.chars().next().expect("");
    let number = segment[1..].parse::<i32>().expect("Didn't parse number");
    let (last_x, last_y, last_dist): (i32, i32, usize) =
        *vertices.clone().last().expect("Missing last element");
    match dir {
        'L' => vertices.push((last_x - number, last_y, last_dist + number as usize)),
        'R' => vertices.push((last_x + number, last_y, last_dist + number as usize)),
        'U' => vertices.push((last_x, last_y - number, last_dist + number as usize)),
        'D' => vertices.push((last_x, last_y + number, last_dist + number as usize)),
        _ => panic!("Unknown direction"),
    }
}

fn find_intersection(wire_1: &[(i32, i32, usize)],
                     wire_2: &[(i32, i32, usize)]) -> Option<(i32, i32, usize)> {
    // Check if the Orderings are as expected for an intersection
    if wire_1[0].0.cmp(&wire_2[0].0) == wire_2[1].0.cmp(&wire_1[1].0) &&
        wire_1[0].1.cmp(&wire_2[0].1) == wire_2[1].1.cmp(&wire_1[1].1) {
            if wire_1[0].0 == wire_1[1].0 {
                let distance = wire_1[0].2 + wire_2[0].2
                    + (wire_2[0].1 - wire_1[0].1).abs() as usize
                    + (wire_1[0].0 - wire_2[0].0).abs() as usize;
                Some((wire_1[0].0, wire_2[0].1, distance))
            } else {
                let distance = wire_1[0].2 + wire_2[0].2
                    + (wire_1[0].1 - wire_2[0].1).abs() as usize
                    + (wire_2[0].0 - wire_1[0].0).abs() as usize;
                Some((wire_2[0].0, wire_1[0].1, distance))
            }
        } else {
            None
        }
}
