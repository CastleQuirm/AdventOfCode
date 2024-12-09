// Potential improvements:
//

use itertools::Itertools;

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    let summary_string = input_lines[0][0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let mut answer1 = 0i64;
    let mut summary_ptr_left = 0;
    let mut summary_ptr_right = summary_string.len() - 1;
    let mut file_id_left = 0i64;
    assert_eq!(summary_string.len() % 2, 1);
    let mut file_id_right = (summary_string.len() / 2) as i64;
    let mut file_index_right = 0;
    let mut uncompressed_index = 0i64;

    while summary_ptr_left <= summary_ptr_right {
        // Process the next left digit, then move one along
        if summary_ptr_left % 2 == 0 {
            // This digit is a left-file.
            let mut file_cells = summary_string[summary_ptr_left];
            if summary_ptr_left == summary_ptr_right {
                file_cells -= file_index_right;
            }

            (0..file_cells).for_each(|i| {
                answer1 += file_id_left * (uncompressed_index + i);
            });
            uncompressed_index += summary_string[summary_ptr_left];
            file_id_left += 1;
        } else {
            // This digit is space for right-files.
            for _ in 0..summary_string[summary_ptr_left] {
                answer1 += file_id_right * uncompressed_index;
                uncompressed_index += 1;
                file_index_right += 1;

                if file_index_right == summary_string[summary_ptr_right] {
                    summary_ptr_right -= 2;
                    file_id_right -= 1;
                    file_index_right = 0;
                    if summary_ptr_right < summary_ptr_left {
                        break;
                    }
                }
            }
        }

        summary_ptr_left += 1;
    }

    // Completely different approach for part 2.
    let mut file_ix = 0;
    let mut memory_ix = 0;
    let mut is_file = true;
    let starting_disk = input_lines[0][0]
        .chars()
        .map(|c| {
            let length = c.to_digit(10).unwrap() as i64;
            let file = FileObject {
                file_id: if is_file { Some(file_ix) } else { None },
                start_memory_ix: memory_ix,
                length,
            };
            memory_ix += length;
            if is_file {
                file_ix += 1;
            }
            is_file = !is_file;
            file
        })
        .collect::<Vec<FileObject>>();
    let mut dynamic_disk = starting_disk.clone();
    (0..starting_disk.len()).rev().for_each(|i| {
        // Try to relocate the ith object if it's a file.
        let candidate_file = &starting_disk[i];
        if candidate_file.file_id.is_some() {
            let target = dynamic_disk.iter().find_position(|target_space| {
                target_space.file_id.is_none()
                    && target_space.length >= candidate_file.length
                    && target_space.start_memory_ix <= candidate_file.start_memory_ix
            });

            if let Some((target_ix, target_space)) = target {
                let target_space = target_space.clone();
                if target_space.length > candidate_file.length {
                    dynamic_disk[target_ix] = FileObject {
                        file_id: None,
                        start_memory_ix: target_space.start_memory_ix + candidate_file.length,
                        length: target_space.length - candidate_file.length,
                    };
                    dynamic_disk.insert(
                        target_ix,
                        FileObject {
                            file_id: candidate_file.file_id,
                            start_memory_ix: target_space.start_memory_ix,
                            length: candidate_file.length,
                        },
                    );
                } else {
                    dynamic_disk[target_ix] = FileObject {
                        file_id: candidate_file.file_id,
                        start_memory_ix: target_space.start_memory_ix,
                        length: candidate_file.length,
                    };
                }
                // remove the original version
                let (removal_ix, _) = dynamic_disk
                    .iter()
                    .enumerate()
                    .rfind(|(_, file)| file.file_id == candidate_file.file_id)
                    .expect("We just moved it, it must be here");
                dynamic_disk.remove(removal_ix);
            }
        }
    });

    let answer2 = dynamic_disk
        .iter()
        .map(|file| {
            // println!("File ID {:?} starts at {} with length {} - checksum {}", file.file_id, file.start_memory_ix, file.length, file.checksum());
            file.checksum()
        })
        .sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone)]
struct FileObject {
    file_id: Option<i64>,
    start_memory_ix: i64,
    length: i64,
}

impl FileObject {
    fn checksum(&self) -> i64 {
        (0..self.length)
            .map(|i| self.file_id.unwrap_or(0) * (self.start_memory_ix + i))
            .sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case01() {
        full_test(
            "2333133121414131402", // INPUT STRING
            "1928",                // PART 1 RESULT
            "2858",                // PART 2 RESULT
        )
    }

    #[test]
    fn check_day09_case02() {
        full_test(
            "12345", // INPUT STRING
            "60",    // PART 1 RESULT
            "132",   // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day09(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
