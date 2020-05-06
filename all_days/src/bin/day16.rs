// extern crate ndarray;

use std::fs;
// use ndarray::arr1;

fn main() {
    // Read the input number as a Vec of digits.
    let mut input_string = read_input_as_vec();

    // // Run the specified number of FTT phases
    // for i in 0..100 {
    //     println!("Phase {}", i);
    //     input_string = run_ftt_phase(input_string);
    // }

    // println!("Part 1 Answer: {}{}{}{}{}{}{}{}",
    //          input_string[0],
    //          input_string[1],
    //          input_string[2],
    //          input_string[3],
    //          input_string[4],
    //          input_string[5],
    //          input_string[6],
    //          input_string[7]);

    let mut part_2_string: Vec<usize> = Vec::new();
    for _i in 0..10_000 {
        part_2_string.extend(input_string.iter().cloned())
    }

    // Run the specified number of FTT phases
    for i in 0..100 {
        println!("Phase {}", i);
        part_2_string = run_ftt_phase(part_2_string);
    }

}

fn read_input_as_vec() -> Vec<usize> {
    fs::read_to_string("input/day16.txt")
        .expect("")
        .trim()
        .chars()
        .map(|digit_char| digit_char.to_string().parse().expect(""))
        .collect()
}

fn run_ftt_phase(in_vec: Vec<usize>) -> Vec<usize> {
    let mut out_vec: Vec<usize> = Vec::new();
    for i in 0..in_vec.len() {
        // Calculate the i'th digit of the output vec
        println!("Calculate digit {} out of {}:", i + 1, in_vec.len());
        out_vec.push(calc_new_digit(i, &in_vec));
    }
    out_vec
}

fn calc_new_digit(index: usize, in_vec: &Vec<usize>) -> usize {
    let mut total: i64 = 0;
    for i in 0..in_vec.len() {
        let mult = match ((i + 1) % (4 * (index + 1))) / (index + 1) {
                0 => 0,
                1 => 1,
                2 => 0,
                3 => -1,
                bad_num => panic!("Can't have value {} here", bad_num),
            };
        total += in_vec[i] as i64 * mult;
    }
    (total.abs() % 10) as usize
}
