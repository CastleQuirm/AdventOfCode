use std::fs;

struct Amplifier {
    program: Vec<i32>
}

struct AmpOut {
    return_value: usize,
    index: usize,
    terminating: bool
}

fn main() {
    part1();
    part2();
}

fn part1() {
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
    println!("Part 1 Answer: {}", best_result);
}

fn part2() {
    let mut best_result = 0;
    'outer: for index in 56789..98766 {
        let mut test_vector: Vec<usize> = Vec::new();

        for digit_char in index.to_string().trim().chars() {
            let digit_string: usize = digit_char.to_string().parse().expect("");
            if digit_string < 5 { continue 'outer; }
            if test_vector.contains(&digit_string) { continue 'outer; }
            test_vector.push(digit_string);
        }
        let result = trial(test_vector);
        if best_result < result { best_result = result; }
    }
    println!("Part 2 Answer: {}", best_result);
}

fn trial(amplifier_order: Vec<usize>) -> usize {
    let mut amplifiers: Vec<Amplifier> = Vec::new();
    let mut start_indices: Vec<usize> = Vec::new();
    let mut create_amplifier: Amplifier;
    for _amp in 0..5 {
        let string = fs::read_to_string("input_program.txt")
            .expect("Failed to read file");

        let program_iter = string.split(",").map(|i| {
            match i.trim().parse() {
                Ok(num) => num,
                Err(_) => 7,
            }
        });

        create_amplifier = Amplifier {program: program_iter.collect()};
        amplifiers.push(create_amplifier) ;
        start_indices.push(0);
    }

    let mut amplifier_input = 0;
    let mut first_loop = true;
    let mut next_amplifier = 0;
    let mut last_e_output: usize;

    loop {
        let mut input_vec = vec![amplifier_input];
        if first_loop { input_vec.push(amplifier_order[next_amplifier]); }
        // println!("Call Amp {} with input {:?} at start_index {}", next_amplifier, input_vec, start_indices[next_amplifier]);
        let output_amp = run_amplifier(&mut amplifiers[next_amplifier],
                                       &mut input_vec,
                                       start_indices[next_amplifier]);
        // println!("Got out: {}", output_amp.return_value);
        amplifier_input = output_amp.return_value;
        start_indices[next_amplifier] = output_amp.index;
        if next_amplifier < 4 {
            next_amplifier += 1;
        } else {
            first_loop = false;
            next_amplifier = 0;
            last_e_output = amplifier_input;
            if output_amp.terminating { break; }
        }
    }

    return last_e_output;
}

fn run_amplifier(amplifier: &mut Amplifier, inputs: &mut Vec<usize>, start_index: usize) -> AmpOut {

    let mut index = start_index;
    let mut instruction = amplifier.program[index];
    let mut return_value = 0;

    while instruction != 99 {
        match instruction % 100 {
            1 => add(&mut amplifier.program, &mut index),
            2 => multiply(&mut amplifier.program, &mut index),
            3 => input(&mut amplifier.program, &mut index, inputs.pop().expect("Didn't have a parameter")),
            4 => { return_value = output(&mut amplifier.program, &mut index); break; },
            5 => jump_if_true(&mut amplifier.program, &mut index),
            6 => jump_if_false(&mut amplifier.program, &mut index),
            7 => less_than(&mut amplifier.program, &mut index),
            8 => equals_to(&mut amplifier.program, &mut index),
            _ => { panic!("Unknown command {}", instruction); }
        }

        instruction = amplifier.program[index];
        // println!("New index {}, new instruction {}", index, instruction);
    }

    if inputs.len() != 0 { panic!("Didn't use all the inputs"); }
    if amplifier.program[index] != 3 && amplifier.program[index] != 99 { panic!("Still doing work"); }
    // println!("Leaving with index {}", index);
    return AmpOut{ return_value, index, terminating: amplifier.program[index] == 99 };
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
    let instruction = program_vector[*index];
    let num_params = match instruction % 100 {
        1 | 2 | 7 | 8 => 3,
        3 | 4 => 1,
        5 | 6 => 2,
        _ => panic!("Impossible instruction"),
    };

    let mut parameter_string = instruction / 100;
    let mut return_indices: Vec<usize> = Vec::new();

    for i in 0..num_params {
        let return_index = match parameter_string % 10 {
            0 => program_vector[index + i + 1] as usize,
            1 => index + i + 1,
            _ => panic!("Param type was {} for index {}", parameter_string % 10, index),
        };
        return_indices.push(return_index);
        parameter_string /= 10;
    }

    return_indices
}
