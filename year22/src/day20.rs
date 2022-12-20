use std::collections::VecDeque;

use itertools::Itertools;

pub fn day20(input_lines: &str) -> (String, String) {
    let encrypted_data = input_lines
        .lines()
        .enumerate()
        .map(|(ix, num)| (ix, num.parse::<i64>().unwrap()))
        .collect::<VecDeque<_>>();

    // for each element in the original encrypted_data, find it in mixed data and move it appropriately.
    let mut mixed_data = encrypted_data.clone();
    mix_data(&encrypted_data, &mut mixed_data);

    // Repeat but with pre-multiplying and 10 mixes.
    let decryption_key = 811589153;
    let decrypted_data = encrypted_data
        .iter()
        .map(|(p, x)| (*p, x * decryption_key))
        .collect::<VecDeque<_>>();
    let mut remixed_data = decrypted_data.clone();
    (0..10).for_each(|_| mix_data(&decrypted_data, &mut remixed_data));

    (
        format!("{}", derive_answer(&mixed_data)),
        format!("{}", derive_answer(&remixed_data)),
    )
}

fn mix_data(encrypted_data: &VecDeque<(usize, i64)>, mixed_data: &mut VecDeque<(usize, i64)>) {
    for entry in encrypted_data {
        let current_index = mixed_data.iter().find_position(|&x| x == entry).unwrap().0;
        mixed_data.remove(current_index);
        let new_index = (current_index as i64 + entry.1) % (encrypted_data.len() as i64 - 1);
        let new_index = if new_index < 0 {
            (new_index + encrypted_data.len() as i64 - 1) as usize
        } else {
            new_index as usize
        };
        mixed_data.insert(new_index, *entry);
    }
}

fn derive_answer(mixed_data: &VecDeque<(usize, i64)>) -> i64 {
    let zero_loc = mixed_data
        .iter()
        .find_position(|(_, x)| *x == 0)
        .expect("No 0 location?")
        .0;
    mixed_data
        .get((zero_loc + 1000) % mixed_data.len())
        .unwrap()
        .1
        + mixed_data
            .get((zero_loc + 2000) % mixed_data.len())
            .unwrap()
            .1
        + mixed_data
            .get((zero_loc + 3000) % mixed_data.len())
            .unwrap()
            .1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day20_both_case1() {
        assert_eq!(
            day20(
                "1
2
-3
3
-2
0
4"
            ),
            ("3".to_string(), "1623178306".to_string())
        )
    }
}
