static INPUTS_DIR: &str = "inputs";

pub fn load_inputs_as_one(day: usize) -> Vec<String> {
    vec![std::fs::read_to_string(format!("{}/{}", INPUTS_DIR, day))
        .expect("Can't open/read input file")]
}

pub fn load_inputs_by_line(day: usize) -> Vec<String> {
    let input = load_inputs_as_one(day);
    input[0]
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}
