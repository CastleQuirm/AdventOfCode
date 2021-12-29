// Potential improvements:
// 1: Is there a way of writing crack_loop_efficient that doesn't use a mut, doesn't repeat all the steps each time we do a new step, and doesn't do some sort of mid-loop return function?
// 2:...that's about it, it's a pretty simple puzzle.  I guess I could remove crack_loop_size, or make the 20201227 number be codified a little more?

pub fn day25(input_lines: &[String]) -> (u64, u64) {
    assert_eq!(input_lines.len(), 2);
    // No part 2 to this puzzle!
    (
        transform_number(
            input_lines[1]
                .parse::<u64>()
                .expect("Couldn't parse second line"),
            crack_loop_efficient(
                input_lines[0]
                    .parse::<u64>()
                    .expect("Couldn't parse first line"),
            ),
        ),
        0,
    )
}

fn crack_loop_efficient(public_key: u64) -> u64 {
    let mut value = 1;
    // Start from 1 because the first time we do it is loop 1.  End before 20201227 because the last time is guaranteed to have already hit a loop.
    (1..20201227)
        .find(|_| {
            value = (value * 7) % 20201227;
            value == public_key
        })
        .expect("Couldn't crack")
}

// This is inefficienct!  We end up doing the fold from scratch each time instead of just adding another one and checking!
// fn crack_loop_size(public_key: u64) -> u64 {
//     (0..20201227)
//         .find(|&x| transform_number(7, x) == public_key)
//         .expect("Couldn't crack")
// }

fn transform_number(subject: u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1, |value, _| (value * subject) % 20201227)
}

#[cfg(test)]
mod tests {
    use super::{crack_loop_efficient, day25, transform_number};

    #[test]
    fn test_transform() {
        assert_eq!(transform_number(7, 8), 5764801);
        assert_eq!(transform_number(7, 11), 17807724);
    }

    // #[test]
    // fn test_crack() {
    //     assert_eq!(crack_loop_size(5764801), 8);
    //     assert_eq!(crack_loop_size(17807724), 11);
    // }

    #[test]
    fn test_crack_efficient() {
        assert_eq!(crack_loop_efficient(5764801), 8);
        assert_eq!(crack_loop_efficient(17807724), 11);
    }

    #[test]
    fn day25_part1_example() {
        let input = "5764801
17807724"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day25(&input), (14897079, 0))
    }
}
