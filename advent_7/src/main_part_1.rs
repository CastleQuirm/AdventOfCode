use std::fs;

fn main() {
    let mut best_result = 0;
    'outer: for index in 12345..54322 {
        let mut test_vector: Vec<usize> = Vec::new();

        for digit_char in index.to_string().trim().chars() {
            let mut digit_string: usize = digit_char.to_string().parse().expect("");
            if digit_string == 0 { continue 'outer; }
            digit_string -= 1;
            if digit_string > 4 { continue 'outer; }
            if test_vector.contains(&digit_string) { continue 'outer; }
            test_vector.push(digit_string);
        }

        let result = trial(test_vector);
        if best_result < result { best_result = result; }
    }

    println!("End result: {}", best_result);
}

fn trial(amplifier_order: Vec<usize>) -> usize {
    let mut amplifier_input = 0;
    for amplifier_value in amplifier_order {
        let mut amplifier_pair: Vec<usize> = vec![amplifier_input, amplifier_value];
        amplifier_input = amplifier(&mut amplifier_pair);
    }
    return amplifier_input;
}

fn amplifier(inputs: &mut Vec<usize>) -> usize {
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
    let mut return_value = 0;

    while instruction != 99 {
        match instruction - (instruction / 100) * 100 {
            1 => add(&mut program_vector, &mut index),
            2 => multiply(&mut program_vector, &mut index),
            3 => input(&mut program_vector, &mut index, inputs.pop().expect("Didn't have a parameter")),
            4 => return_value = output(&mut program_vector, &mut index),
            5 => jump_if_true(&mut program_vector, &mut index),
            6 => jump_if_false(&mut program_vector, &mut index),
            7 => less_than(&mut program_vector, &mut index),
            8 => equals_to(&mut program_vector, &mut index),
            _ => { panic!("Unknown command {}", instruction); }
        }

        instruction = program_vector[index];
    }

    return return_value;
}

fn add(program_vector: &mut Vec<i32>, index: &mut usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] = program_vector[indices[0]] + program_vector[indices[1]];
    *index += 4;
}

fn multiply(program_vector: &mut Vec<i32>, index: &mut usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] = program_vector[indices[0]] * program_vector[indices[1]];
    *index += 4;
}

fn input(program_vector: &mut Vec<i32>, index: &mut usize, input: usize) {
    let output_index = program_vector[*index + 1] as usize;
    program_vector[output_index] = input as i32;
    *index += 2;
}

fn output(program_vector: &mut Vec<i32>, index: &mut usize) -> usize {
    let indices = parameter_indices(program_vector, index);
    *index += 2;
    if (program_vector[indices[0]] != 0) && program_vector[*index] != 99 {
        panic!("OH NO");
    }
    return program_vector[indices[0]] as usize;
}

fn jump_if_true(program_vector: &mut Vec<i32>, index: &mut usize) {
    let indices = parameter_indices(program_vector, index);
    if program_vector[indices[0]] != 0 {
        *index = program_vector[indices[1]] as usize;
    }
    else {
        *index += 3;
    }
}

fn jump_if_false(program_vector: &mut Vec<i32>, index: &mut usize) {
    let indices = parameter_indices(program_vector, index);
    if program_vector[indices[0]] == 0 {
        *index = program_vector[indices[1]] as usize;
    }
    else {
        *index += 3;
    }
}

fn less_than(program_vector: &mut Vec<i32>, index: &mut usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] =
        if program_vector[indices[0]] < program_vector[indices[1]] { 1 }
        else { 0 } ;
    *index += 4;
}

fn equals_to(program_vector: &mut Vec<i32>, index: &mut usize) {
    let indices = parameter_indices(program_vector, index);
    program_vector[indices[2]] =
        if program_vector[indices[0]] == program_vector[indices[1]] { 1 }
        else { 0 } ;
    *index += 4;
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
