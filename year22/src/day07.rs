use std::collections::HashMap;

pub fn day07(input_lines: &str) -> (String, String) {
    let mut root_directory = Directory::default();
    let mut current_directory = &mut root_directory;
    let mut path: Vec<String> = Vec::new();

    for line in input_lines.lines() {
        let first_char = line.chars().next().expect("Empty line?");

        match first_char {
            x if x.is_numeric() => current_directory.files.push(
                line.split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Couldn't parse file size"),
            ),
            'd' => {
                let dir_name = line
                    .split_ascii_whitespace()
                    .nth(1)
                    .expect("Nothing following the dir?")
                    .to_string();
                let existing_dir = current_directory
                    .subdirectories
                    .insert(dir_name, Directory::default());
                assert!(existing_dir.is_none());
            }
            '$' => match line.split_ascii_whitespace().nth(2) {
                Some("\\") => {
                    current_directory = &mut root_directory;
                    path = Vec::new();
                }
                Some("..") => {
                    current_directory = &mut root_directory;
                    path.iter().for_each(|dir| 
                        current_directory = current_directory.subdirectories.get_mut(dir).expect("Moving to a directory we don't know!")
                    );
                    path.pop().expect("Tried to go up from root");
                }
                Some(dir) => {
                    // Rewalk from root to avoid mutable borrow issues.
                    current_directory = &mut root_directory;




                    // let target_directory = current_directory
                    //     .subdirectories
                    //     .get_mut(dir)
                    //     .expect("Moving to a directory we don't know!");
                    // path.push(current_directory);
                    // current_directory = target_directory;
                }
                None => assert_eq!(line, "$ ls"),
            },
            _ => panic!(),
        }
    }

    let _ = root_directory.set_size();
    let answer1 = root_directory.sum_recursively_if_under();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Default)]
struct Directory {
    subdirectories: HashMap<String, Directory>,
    files: Vec<usize>,
    total_size: usize,
}

impl Directory {
    fn set_size(&mut self) -> usize {
        assert_eq!(self.total_size, 0);
        self.subdirectories
            .iter_mut()
            .map(|(_, d)| d.set_size())
            .sum::<usize>()
            + self.files.iter().sum::<usize>()
    }

    fn sum_recursively_if_under(&self) -> usize {
        let size = self
            .subdirectories
            .iter()
            .map(|(_, d)| d.sum_recursively_if_under())
            .sum::<usize>()
            + self.total_size;
        if size <= 100_000 {
            size
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(day07("").0, "0".to_string())
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(day07("").1, "0".to_string())
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(
            day07(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            ),
            ("95437".to_string(), "0".to_string())
        )
    }
}
