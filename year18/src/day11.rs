// Potential improvements:
//

use crate::utils::Coord;

pub fn day11(input_lines: &[Vec<String>]) -> (String, String) {

    let serial_no = input_lines[0][0].parse::<i32>().expect("Can read");

    // Guess why the original version was bugged!
    // let answer_coord =(0..(298^2)).map(|i| {
    let answer_coord_1 =(0..(298_i32.pow(2))).map(|i| {
        let x = i / 298;
        let y = i % 298;
        Coord { x, y }
    }).max_by_key(|c| {
        // We're inefficiently calculating (almost) every cell's power 9 times
        // so we should cache instead...but Part 1 runs in < 3ms anyway, so eh.
        c.power_grid(serial_no, 3)
    }).expect("no best?");

    let (answer_coord_2, size) =(0..(298_i32.pow(2))).map(|i| {
        let x = i / 298;
        let y = i % 298;
        let size = 3;
        (Coord { x, y }, size)
    }).max_by_key(|(c, size)| {
        // We're inefficiently calculating (almost) every cell's power 9 times
        // so we should cache instead...but Part 1 runs in < 3ms anyway, so eh.
        c.power_grid(serial_no, size)
    }).expect("no best?");

    let answer1 = format!("{},{}", answer_coord_1.x, answer_coord_1.y);
    let answer2 = format!("{},{},{}", answer_coord_2.x, answer_coord_2.y, size);
    (format!("{}", answer1), format!("{}", answer2))
}

impl Coord {
    fn power(&self, serial_no: i32) -> i32 {
        let rack_id = self.x + 10;
        let mut power_level = rack_id * self.y;
        power_level += serial_no;
        power_level *= rack_id;
        power_level /= 100;
        power_level %= 10;
        power_level - 5
    }

    fn power_grid(&self, serial_no: i32, grid_size: i32) -> i32 {
        (0..grid_size.pow(2)).map(|dc| {
            let dx = dc / 3;
            let dy = dc % 3;
            let coord = Coord { x: self.x + dx, y: self.y + dy };
            coord.power(serial_no)
        }).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::day11;
    use crate::utils::load_input;
    use crate::utils::Coord;

    #[test]
    fn check_day11_case01() {
        full_test(
            "18",  // INPUT STRING
            "33,45", // PART 1 RESULT
            "90,269,16", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day11_case02() {
        full_test(
            "42",  // INPUT STRING
            "21,61", // PART 1 RESULT
            "232,251,12", // PART 2 RESULT
        )
    }

    #[test]
    fn check_power_levels() {
        let coord = Coord { x: 3, y: 5 };
        assert_eq!(coord.power(8), 4);

        let coord = Coord { x: 122, y: 79 };
        assert_eq!(coord.power(57), -5);

        let coord = Coord { x: 217, y: 196 };
        assert_eq!(coord.power(39), 0);

        let coord = Coord { x: 101, y: 153 };
        assert_eq!(coord.power(71), 4);
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day11(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
