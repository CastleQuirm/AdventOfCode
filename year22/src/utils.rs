pub fn split_input_on_line_breaks(lines: &str) -> Vec<Vec<&str>> {
    let mut full_set = Vec::new();
    let mut build_set = Vec::new();

    for line in lines.lines() {
        if line.is_empty() {
            full_set.push(build_set);
            build_set = Vec::new();
        } else {
            build_set.push(line);
        }
    }
    full_set.push(build_set);

    full_set
}
