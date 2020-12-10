// Possible improvements
// 1: make functional (no for loop, no muts)
// 2: make the "perms_of" function better (scale infinitely, not use hard-coding, better name)
// 3: split parts into their own neater functions.
// 4: just generally a lot, it's a mess.  Fast though!

pub fn day10(input_lines: &[String]) -> (u64, u64) {
    // println!("number of lines is {}", input_lines.len());
    let max_value: usize = input_lines.iter().map(|line| line.parse().expect("Count't parse a line")).max().expect("List was empty?");
    // println!("Max Value is {}", max_value);
    let count_threes = (max_value - input_lines.len()) / 2;
    let count_ones = input_lines.len() - count_threes;
    // println!("Count of 1s: {}, Count of 3s: {}", count_ones, count_threes + 1);
    
    // Part 2: Count sub-arrays of 1-gaps (can I use split?) and collect these.  Map them to permutations and compute the product.
    // "Map to permutations": mapping of [1,2,3,...] -> [1,1,2,<sum of three previous elements>]

    let mut sorted_array = input_lines.iter().map(|line| line.parse().expect("Count't parse a line")).collect::<Vec<usize>>();
    sorted_array.sort();
    let mut blocks_of_ones: Vec<usize> = Vec::new();
    let mut counter = 0;

    for i in 0..sorted_array.len() {
        // println!("i: {}, array_num: {}, Current counter: {}", i, sorted_array[i], counter);
        counter += 1;
        if (i == 0 && sorted_array[0] == 3) || (i != 0 && sorted_array[i] == sorted_array[i-1] + 3) {
            blocks_of_ones.push(counter);
            counter = 0;
            // println!("blocks_of_ones: {:?}", blocks_of_ones);
        }
    }
    blocks_of_ones.push(counter+1);
    // println!("{:?}", blocks_of_ones);

    ((count_ones * (count_threes + 1)) as u64, blocks_of_ones.iter().map(|&x| perms_of(x)).product::<u64>())
}

fn perms_of(size: usize) -> u64 {
    // Only bothered going up to 7, and can't be bothered to code filling it - if needed, it would be N_x = N_(x-2) + N_(x-1) + N_x
    match size {
        1 => 1,
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 7,
        6 => 13,
        7 => 24,
        _ => 0 
    }
}

#[cfg(test)]
mod tests {
    use super::day10;

    #[test]
    fn simple_test() {
        let input_lines = 
"16
10
15
5
1
11
7
19
6
12
4"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day10(&input_lines), (35, 8));
    }

    #[test]
    fn complex_test() {
        let input_lines = 
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
                    .lines()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>();
                assert_eq!(day10(&input_lines), (220, 19208));
    }
}