use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/input/input01")
        .expect("Failed to read file")
        .lines()
        .map(|line| line.parse().expect("Couldn't parse line"))
        .collect();
    println!("Day 1 Part 1: {}", calculate2(&input));
    println!("Day 1 Part 2: {}", calculate3(&input));
}

fn calculate2(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    0
}

fn calculate3(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}
