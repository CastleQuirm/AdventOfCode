// Potential improvements:
//

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let lines = input_lines[0]
        .iter()
        .map(|line| line.parse::<usize>().expect("Couldn't parse"));
    let answer1 = lines.clone().map(calc_fuel).sum::<usize>();
    let answer2 = lines.map(calc_fuel_iteratively).sum::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn calc_fuel_iteratively(mass: usize) -> usize {
    let mut total_mass = 0;
    let mut new_fuel = calc_fuel(mass);
    while new_fuel != 0 {
        total_mass += new_fuel;
        new_fuel = calc_fuel(new_fuel);
    }
    total_mass
}

fn calc_fuel(mass: usize) -> usize {
    if mass >= 6 {
        mass / 3 - 2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::calc_fuel_iteratively;

    #[test]
    fn check_day01_part2_calculations() {
        assert_eq!(calc_fuel_iteratively(14), 2);
        assert_eq!(calc_fuel_iteratively(1969), 966);
        assert_eq!(calc_fuel_iteratively(100756), 50346);
    }
}
