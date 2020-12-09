// Possible improvements:
// 1: Better Part 2 algorithm. Once we've stopped working on a particular starting point slice, instead of starting fresh from the next number, just drop the first element and continue.
// 2: More involved Part 1 algorithm?  Could do something clever with hash-maps of every number a value can pair up to reach, then replace that entry when moving the window by incrementing each element by the delta.  Is it actually more efficient, though?
// ... otherwise, not much else?  Pretty happy with the Rust in this one!

static PREAMBLE_LEN: usize = 25;

pub fn day9(input_lines: &[String]) -> (u64, u64) {
    let input_nums = input_lines
        .iter()
        .map(|x| x.parse::<u64>().expect("Couldn't parse line as number"))
        .collect::<Vec<u64>>();
    let error_index = (0..(input_nums.len() - PREAMBLE_LEN))
        .find(|&x| {
            !found_sum(
                &input_nums[x..(x + PREAMBLE_LEN)],
                input_nums[x + PREAMBLE_LEN],
            )
        })
        .expect("Couldn't find an entry point for hack");
    let part1_result = input_nums[error_index + PREAMBLE_LEN];
    let part2_start_index = (0..(input_nums.len()))
        .find(|&x| try_finding_sum(&input_nums[x..], part1_result).is_some())
        .expect("Couldn't answer Part 2");

    (
        part1_result,
        try_finding_sum(&input_nums[part2_start_index..], part1_result)
            .expect("Just had an answer"),
    )
}

fn found_sum(input_slice: &[u64], target_num: u64) -> bool {
    (0..PREAMBLE_LEN - 1)
        .any(|i| (i..PREAMBLE_LEN).any(|j| input_slice[i] + input_slice[j] == target_num))
}

fn try_finding_sum(input_slice: &[u64], target_num: u64) -> Option<u64> {
    let result = (1..input_slice.len())
        .find(|&i| input_slice[0..i].iter().sum::<u64>() >= target_num)
        .expect("Didn't exceed count by the end of the input?");
    if input_slice[0..result].iter().sum::<u64>() == target_num {
        Some(
            input_slice[0..result].iter().min().expect("No min?")
                + input_slice[0..result].iter().max().expect("No max?"),
        )
    } else {
        None
    }
}
