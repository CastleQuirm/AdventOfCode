pub fn split_input_by_blocks<S>(lines: &str, struct_new: fn(&[&str]) -> S) -> Vec<S> {
    let mut full_set = Vec::new();
    let mut build_set = Vec::new();

    for line in lines.lines() {
        if line.is_empty() {
            full_set.push(struct_new(&build_set));
            build_set = Vec::new();
        } else {
            build_set.push(line);
        }
    }
    full_set.push(struct_new(&build_set));

    full_set
}
