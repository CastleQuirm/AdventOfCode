use std::fs;

fn main() {
    let input = fs::read_to_string("input/input01").expect("Failed to read file").lines().map(|line| line.parse()).filter_map(Result::ok).collect();

    let answer_1 = calculate2(&input);
    println!("Day 1 Part 1: {}", answer_1);

    let answer_2 = calculate3(&input);
    println!("Day 1 Part 2: {}", answer_2);
}

fn calculate2(input: &Vec<i32>) -> i32 {
    let mut output: i32 = -1;
    
    for i in 0..input.len() {
        for j in i+1..input.len() {
            if input[i]+input[j] == 2020 {
                output = input[i]*input[j];
            }
        }
    }

    return output
}

fn calculate3(input: &Vec<i32>) -> i32 {
    let mut output: i32 = -1;
    
    for i in 0..input.len() {
        for j in i+1..input.len() {
            for k in j+1..input.len() {
                if input[i]+input[j]+input[k] == 2020 {
                    output = input[i]*input[j]*input[k];
                }
            }
        }
    }

    return output
}