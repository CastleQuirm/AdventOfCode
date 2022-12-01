use std::fmt::Debug;
use std::str::FromStr;

pub fn split_input_on_line_breaks<T>(lines: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut full_set = Vec::new();
    let mut build_set = Vec::new();

    for line in lines.lines() {
        if line.is_empty() {
            full_set.push(build_set);
            build_set = Vec::new();
        } else {
            build_set.push(line.parse::<T>().expect("Oh no - Parsing failed"));
        }
    }
    full_set.push(build_set);

    full_set
}
