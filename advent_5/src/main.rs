use std::fs;
use std::io;

fn main() {
    let string = fs::read_to_string("input_program.txt")
        .expect("Failed to read file");

    let program_iter = string.split(",").map(|i| {
        match i.trim().parse() {
            Ok(num) => num,
            Err(_) => 7,
        }
    });

    let mut program_vector: Vec<i32> = program_iter.collect();
    let mut index = 0;
    let mut instruction = program_vector[index];
    let mut happy = true;

    while instruction != 99 {
        // println!("Value at index {} is {}", index, instruction);
        if !happy {
            panic!("program is unhappy at new index {}", index);
        }

        match instruction - (instruction / 100) * 100 {
            1 => {
                add(&mut program_vector, &index);
                index += 4;
            }
            2 => {
                multiply(&mut program_vector, &index);
                index += 4;
            }
            3 => {
                input(&mut program_vector, &index);
                index += 2;
            }
            4 => {
                happy = output(&mut program_vector, &index);
                index += 2;
            }
            5 => index = jump_if_true(&mut program_vector, &index),
            6 => index = jump_if_false(&mut program_vector, &index),
            7 => {
                less_than(&mut program_vector, &index);
                index += 4;
            }
            8 => {
                equals_to(&mut program_vector, &index);
                index += 4;
            }
            _ => { panic!("Unknown command {}", instruction); }
        }

        instruction = program_vector[index];
    }

}

fn add(program_vector: &mut Vec<i32>, index: &usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] = program_vector[indices[0]] + program_vector[indices[1]];
}

fn multiply(program_vector: &mut Vec<i32>, index: &usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] = program_vector[indices[0]] * program_vector[indices[1]];
}

fn input(program_vector: &mut Vec<i32>, index: &usize) {
    let mut input = String::new();

    println!("Provide an input:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(msg) => {println!("{}", msg); panic!("Enter a valid value"); }
    };

    let output_index = program_vector[index + 1] as usize;
    program_vector[output_index] = input;
}

fn output(program_vector: &mut Vec<i32>, index: &usize) -> bool {
    let output_value = match program_vector[*index] {
        4 => program_vector[program_vector[index + 1] as usize],
        104 => program_vector[(index + 1) as usize],
        _ => panic!("Output instruction of {} didn't make sense", program_vector[*index]),
    };
    println!("Output {}", output_value);
    if output_value != 0 { return false; }
    else { return true; }
}

fn jump_if_true(program_vector: &mut Vec<i32>, index: &usize) -> usize {
    let indices = parameter_indices(program_vector, index);
    if program_vector[indices[0]] != 0 {
        return program_vector[indices[1]] as usize;
    }
    else {
        return index + 3;
    }
}

fn jump_if_false(program_vector: &mut Vec<i32>, index: &usize) -> usize {
    let indices = parameter_indices(program_vector, index);
    if program_vector[indices[0]] == 0 {
        return program_vector[indices[1]] as usize;
    }
    else {
        return index + 3;
    }
}

fn less_than(program_vector: &mut Vec<i32>, index: &usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] =
        if program_vector[indices[0]] < program_vector[indices[1]] { 1 }
        else { 0 } ;
}

fn equals_to(program_vector: &mut Vec<i32>, index: &usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] =
        if program_vector[indices[0]] == program_vector[indices[1]] { 1 }
        else { 0 } ;
}

fn parameter_indices(program_vector: &mut Vec<i32>, index: &usize) -> Vec<usize> {
    let mut parameter_string = program_vector[*index] / 100;
    let mode1 = parameter_string - (parameter_string / 10) * 10;
    parameter_string /= 10;
    let mode2 = parameter_string - (parameter_string / 10) * 10;
    parameter_string /= 10;
    let mode3 = parameter_string - (parameter_string / 10) * 10;

    let first_index = match mode1 {
        0 => program_vector[index + 1] as usize,
        1 => index + 1,
        _ => panic!("First index wasn't 0 or 1, index {}", index),
    };
    let second_index = match mode2 {
        0 => program_vector[index + 2] as usize,
        1 => index + 2,
        _ => panic!("Second index wasn't 0 or 1, index {}", index),
    };
    let output_index = match mode3 {
        0 => program_vector[index + 3] as usize,
        1 => index + 3,
        _ => panic!("Third index wasn't 0 or 1, index {}", index),
    };

    vec![first_index, second_index, output_index]
}
