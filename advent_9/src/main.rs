use std::fs;

struct Computer {
    program: Vec<i64>,
    relative_base: usize,
}

fn main() {
    let string = fs::read_to_string("input_program.txt")
        .expect("Failed to read file");

    let program_iter = string.split(",").map(|i| {
        match i.trim().parse() {
            Ok(num) => num,
            Err(_) => 7,
        }
    });

    print!("Part 1 answer: ");
    run_computer(&mut Computer {program: program_iter.clone().collect(), relative_base: 0}, &mut vec![1], 0);
    print!("Part 2 answer: ");
    run_computer(&mut Computer {program: program_iter.clone().collect(), relative_base: 0}, &mut vec![2], 0);

}

fn run_computer(computer: &mut Computer, inputs: &mut Vec<usize>, start_index: usize) {

    let mut index = start_index;
    let mut instruction = computer.program[index];

    while instruction != 99 {
        match instruction % 100 {
            1 => add(computer, &mut index),
            2 => multiply(computer, &mut index),
            3 => input(computer, &mut index, inputs.pop().expect("Didn't have a parameter")),
            4 => output(computer, &mut index),
            5 => jump_if_true(computer, &mut index),
            6 => jump_if_false(computer, &mut index),
            7 => less_than(computer, &mut index),
            8 => equals_to(computer, &mut index),
            9 => change_relative_base(computer, &mut index),
            _ => { panic!("Unknown command {}", instruction); }
        }

        instruction = computer.program[index];
    }
}

fn add(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    computer.program[indices[2]] = computer.program[indices[0]] + computer.program[indices[1]];
    *index += 4;
}

fn multiply(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    computer.program[indices[2]] = computer.program[indices[0]] * computer.program[indices[1]];
    *index += 4;
}

fn input(computer: &mut Computer, index: &mut usize, input: usize) {
    let indices = parameter_indices(computer, index);
    computer.program[indices[0]] = input as i64;
    *index += 2;
}

fn output(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    *index += 2;
    println!("{}", computer.program[indices[0]]);
}

fn jump_if_true(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    if computer.program[indices[0]] != 0 {
        *index = computer.program[indices[1]] as usize;
    }
    else {
        *index += 3;
    }
}

fn jump_if_false(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    if computer.program[indices[0]] == 0 {
        *index = computer.program[indices[1]] as usize;
    }
    else {
        *index += 3;
    }
}

fn less_than(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    computer.program[indices[2]] =
        if computer.program[indices[0]] < computer.program[indices[1]] { 1 }
        else { 0 } ;
    *index += 4;
}

fn equals_to(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    computer.program[indices[2]] =
        if computer.program[indices[0]] == computer.program[indices[1]] { 1 }
        else { 0 } ;
    *index += 4;
}

fn change_relative_base(computer: &mut Computer, index: &mut usize) {
    let indices = parameter_indices(computer, index);
    computer.relative_base = (computer.relative_base as i64 + computer.program[indices[0]]) as usize;
    *index += 2;
}

fn parameter_indices(computer: &mut Computer, index: &usize) -> Vec<usize> {
    let instruction = computer.program[*index];
    let num_params = match instruction % 100 {
        1 | 2 | 7 | 8 => 3,
        3 | 4 | 9 => 1,
        5 | 6 => 2,
        _ => panic!("Impossible instruction"),
    };

    let mut parameter_string = instruction / 100;
    let mut return_indices: Vec<usize> = Vec::new();

    for i in 0..num_params {
        let return_index = match parameter_string % 10 {
            0 => computer.program[index + i + 1] as usize,
            1 => index + i + 1,
            2 => ((computer.relative_base as i64) + computer.program[index + i + 1]) as usize,
            _ => panic!("Param type was {} for index {}", parameter_string % 10, index),
        };
        fill_in_zeroes(computer, return_index);
        return_indices.push(return_index);
        parameter_string /= 10;
    }

    return_indices
}

fn fill_in_zeroes(computer: &mut Computer, length_needed: usize) {
    while length_needed >= computer.program.len() {
        computer.program.push(0);
    }
}
