use std::fs;

fn main() {

    let mut noun = 0;
    let mut verb = 0;

    loop {
        let string = fs::read_to_string("input_program.txt")
            .expect("Failed to read file");

        let program_iter = string.split(",").map(|i| {
            match i.trim().parse() {
                Ok(num) => num,
                Err(_) => 7,
            }
        });

        let mut program_vector: Vec<i32> = program_iter.collect();

        // Special instructions!
        program_vector[1] = noun;
        program_vector[2] = verb;
        // End special instructions

        let mut index = 0;

        loop {
            let instruction = program_vector[index];
            // println!("Value at index {} is {}", index, instruction);
            if instruction == 99 {
                break;
            };
            let first_index = program_vector[index + 1] as usize;
            let second_index = program_vector[index + 2] as usize;
            let output_index = program_vector[index + 3] as usize;
            // println!("Indices: {}, {} and {}", first_index, second_index, output_index);

            let first_value = program_vector[first_index];
            let second_value = program_vector[second_index];
            // println!("Values: {}, {}", first_value, second_value);

            program_vector[output_index] = match instruction {
                1 => first_value + second_value,
                2 => first_value * second_value,
                _ => program_vector[output_index],
            };
            index += 4;
        }

        if program_vector[0] == 19690720 {
            break;
        } else if verb == 99 {
            verb = 0;
            noun += 1;
        } else {
            verb += 1;
        }
    }

println!("Noun is {}, Verb is {}", noun, verb);
}
