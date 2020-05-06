// extern crate ndarray;

use std::fs;
// use ndarray::arr1;

fn main() {
    // Read the input number as a Vec of digits.
    let mut input_string = read_input_as_vec();

    let mut part_2_string: Vec<usize> = Vec::new();
    for _i in 0..10_000 {
        part_2_string.extend(input_string.iter().cloned())
    }

    // Run the specified number of FTT phases
    for i in 0..100 {
        println!("Phase {}", i);
        part_2_string = run_ftt_phase(part_2_string);
    }

    println!("Part 1 Answer: {}{}{}{}{}{}{}{}",
             input_string[0],
             input_string[1],
             input_string[2],
             input_string[3],
             input_string[4],
             input_string[5],
             input_string[6],
             input_string[7]);
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
        println!("Calculate digit {} out of {}", i + 1, in_vec.len());
        let mult_vec = build_mult_vec(in_vec.len(), i);
        assert_eq!(mult_vec.len(), in_vec.len());
        out_vec.push((mult_and_sum_arrays(&in_vec, &mult_vec).abs() % 10) as usize);
    }
    out_vec
}

fn build_mult_vec(length: usize, leading_zeroes: usize) -> Vec<i8> {
    let mut out_vec: Vec<i8> = Vec::new();
    for i in 0..length + 1 {
        let value = match (i % (4 * (leading_zeroes + 1))) / (leading_zeroes + 1) {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            bad_num => panic!("Can't have value {} here", bad_num),
        };
        out_vec.push(value);
    }
    out_vec[1..].to_vec()
}

fn mult_and_sum_arrays(arr1: &Vec<usize>, arr2: &Vec<i8>) -> i64 {
    assert_eq!(arr1.len(), arr2.len());
    let mut total: i64 = 0;
    for i in 0..arr1.len() {
        total += arr1[i] as i64 * arr2[i] as i64;
    }
    total
}
