use std::fs;
use std::cmp::min;

fn main() {
    // Read the input number as a Vec of digits.
    let input_string = read_input_as_vec();
    let mut part_1_string = input_string.clone();

    // Run the specified number of FTT phases
    for i in 0..100 {
        println!("Phase {}", i);
        part_1_string = run_ftt_phase(part_1_string);
    }

    println!("Part 1 Answer: {}{}{}{}{}{}{}{}",
             part_1_string[0],
             part_1_string[1],
             part_1_string[2],
             part_1_string[3],
             part_1_string[4],
             part_1_string[5],
             part_1_string[6],
             part_1_string[7]);

    let mut part_2_string: Vec<usize> = Vec::new();
    for _i in 0..10_000 {
        part_2_string.extend(input_string.iter().clone())
    }
    // The first seven digits of this string give the offset of the answer -
    // but this offset can be applied to the string as is, because every digit
    // will only ever be affected by itself and the digits after it!
    // This cuts out nearly 6 million of the 6.5 million digits, allowing us to
    // solve the process in a much more plausible time - especially since we
    // can easily see that by the time we're over half-way through the number,
    // the process for each digit is simply "sum this and every digit afterwards"
    let offset = input_string[0] * 1_000_000 +
        input_string[1] * 100_000 +
        input_string[2] * 10_000 +
        input_string[3] * 1_000 +
        input_string[4] * 100 +
        input_string[5] * 10 +
        input_string[6];
    println!("Offset: {}", offset);
    assert!(offset < part_2_string.len());
    assert!(offset > part_2_string.len() / 2);
    part_2_string = part_2_string[offset..].to_vec();

    // Run the specified number of simplified FTT phases
    for i in 0..100 {
        println!("Phase {}", i);
        part_2_string = run_easy_ftt_phase(part_2_string);
    }

    println!("Part 2 Answer: {}{}{}{}{}{}{}{}",
             part_2_string[0],
             part_2_string[1],
             part_2_string[2],
             part_2_string[3],
             part_2_string[4],
             part_2_string[5],
             part_2_string[6],
             part_2_string[7]);
}

fn read_input_as_vec() -> Vec<usize> {
    fs::read_to_string("input/day16.txt")
        .expect("")
        .trim()
        .chars()
        .map(|digit_char| digit_char.to_string().parse().expect(""))
        .collect()
}

fn run_easy_ftt_phase(in_vec: Vec<usize>) -> Vec<usize> {
    let mut out_vec: Vec<usize> = vec![in_vec[in_vec.len() - 1]];
    for i in 2..in_vec.len() + 1 {
        let new_num = out_vec[0] + in_vec[in_vec.len() - i];
        out_vec.insert(0, new_num % 10);
    }
    out_vec
}

fn run_ftt_phase(in_vec: Vec<usize>) -> Vec<usize> {
    let mut out_vec: Vec<usize> = Vec::new();
    for i in 0..in_vec.len() {
        // Calculate the i'th digit of the output vec
        // println!("Calculate digit {} out of {}:", i + 1, in_vec.len());
        out_vec.push(calc_new_digit(i, &in_vec));
    }
    out_vec
}

fn calc_new_digit(index: usize, in_vec: &Vec<usize>) -> usize {
    let mut total: i64 = 0;
    let mut i = index;
    while i < in_vec.len() {
        let upper_bound = min(i + index + 1, in_vec.len());
        match ((i + 1) % (4 * (index + 1))) / (index + 1) {
            0 | 2 => (),
            1 => { total += in_vec[i..upper_bound].iter().sum::<usize>() as i64;}
            3 => { total -= in_vec[i..upper_bound].iter().sum::<usize>() as i64;}
            bad_num => panic!("Can't have value {} here", bad_num),
        }
        i += index + 1;
    }
    (total.abs() % 10) as usize
}
