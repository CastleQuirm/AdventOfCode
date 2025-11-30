use std::collections::HashMap;

pub fn day07(input_lines: &str) -> (String, String) {
    let mut root_directory = Directory::default();
    let mut current_directory = &mut root_directory;
    let mut current_directory_name = "root".to_string();
    let mut path: Vec<String> = Vec::new();

    for line in input_lines.lines() {
        let first_char = line.chars().next().expect("Empty line?");
        // println!("Process: {}", line);

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
                Some("/") => {
                    current_directory = &mut root_directory;
                    current_directory_name = "root".to_string();
                    path = Vec::new();
                }
                Some("..") => {
                    current_directory = &mut root_directory;
                    // current_directory = current_directory
                    //     .subdirectories
                    //     .get_mut("hello")
                    //     .expect("Moving to a directory we don't know!");

                    for dir in &path[1..] {
                        current_directory = current_directory
                            .subdirectories
                            .get_mut(dir)
                            .expect("Moving to a directory we don't know!");
                    }

                    current_directory_name = path.pop().unwrap_or_else(|| "root".to_string());

                    // let temp_directory = &mut current_directory;
                    // path.iter().for_each(|dir| {
                    //     *temp_directory = (temp_directory.subdirectories)
                    //         .get_mut(dir)
                    //         .expect("Moving to a directory we don't know!");
                    // });
                    // current_directory = path.pop().expect("Tried to go up from root");
                }
                Some(dir) => {
                    current_directory = current_directory
                        .subdirectories
                        .get_mut(dir)
                        .expect("Moving to a directory we don't know!");
                    path.push(current_directory_name.clone());
                    current_directory_name = dir.to_string();
                }
                None => assert_eq!(line, "$ ls"),
            },
            _ => panic!(),
        }

        // println!(
        //     "New directory: {}, path: {:?}",
        //     current_directory_name, path
        // );
    }

    let _ = root_directory.set_size();

    // println! {"{:?}", root_directory};

    let answer1 = root_directory.sum_recursively_if_under();

    let free_space = 70_000_000 - root_directory.total_size;
    let min_required_space = 30_000_000 - free_space;

    let answer2 = root_directory
        .find_min_size_above(min_required_space)
        .expect("Couldn't find anything");
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Default, Debug)]
struct Directory {
    subdirectories: HashMap<String, Directory>,
    files: Vec<usize>,
    total_size: usize,
}

impl Directory {
    fn set_size(&mut self) -> usize {
        assert_eq!(self.total_size, 0);
        self.total_size = self
            .subdirectories
            .iter_mut()
            .map(|(_, d)| d.set_size())
            .sum::<usize>()
            + self.files.iter().sum::<usize>();
        self.total_size
    }

    fn sum_recursively_if_under(&self) -> usize {
        self.subdirectories
            .values()
            .map(|d| d.sum_recursively_if_under())
            .sum::<usize>()
            + if self.total_size <= 100_000 {
                self.total_size
            } else {
                0
            }
    }

    fn find_min_size_above(&self, threshold: usize) -> Option<usize> {
        // if this directory itself isn't big enough, just return None.
        if self.total_size < threshold {
            return None;
        }
        let min_sub_dir = self
            .subdirectories
            .iter()
            .filter_map(|(_, d)| d.find_min_size_above(threshold))
            .min();
        min_sub_dir.or(Some(self.total_size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            ("95437".to_string(), "24933642".to_string())
        )
    }
}
