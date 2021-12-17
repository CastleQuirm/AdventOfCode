// Potential improvements:
//

pub fn day17(input_lines: &[String]) -> (u64, u64) {
    let target = Target::new(input_lines[0]);


    // // Let's produce a hashmap of valid starting Xs to the number of steps when they'll land in range (noting we'll need something special for "infinitely far")
    // let mut steps_for_starting_x: HashMap<u64, (u64, Option<u64>)> = HashMap::new();
    // let mut x = 0;
    // while x < max_x {
    //     x += 1;
    //     let triangle_x = x * (x+1) / 2;
    //     // Check if triangle numbers let us ever reach it?
    //     if triangle_x < min_x { continue; }
    //     // Fewest steps to reach the range...
    //     while steps 
    //     let fewest_steps = ???;
    //     let most_steps = if triangle_x <= max_x {
    //         None
    //     } else {
    //         if ??? < fewest_steps { continue; }
    //         Some(???)
    //     }
    //     if most_steps < 
    //     steps_for_starting_x.insert(x, (fewest_steps, most_steps));
    // }

    let best_y = (0..target.max_x+1).filter_map(|starting_x| {
        // Bypass x that's too short
        if starting_x * (starting_x+1) / 2 < target.min_x { return None; }
        (target.min_y..0).find(|&starting_y| {
            // Return true if we hit the target, false if we don't 
            target.does_trajectory_hit(starting_x, starting_y.abs())
        })
    }).max().expect("No best y");

    let part1 = (best_y * (best_y + 1) / 2) as u64;

    (part1, 0)
}

struct Target {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64
}
impl Target {
    fn new(input: String) -> Self {
        let coordinate_ranges = input
            .split(": ")
            .nth(1)
            .expect("")
            .split(", ")
            .collect::<Vec<&str>>();
        let x_values = coordinate_ranges
            .iter()
            .find(|range| range.contains('x'))
            .expect("No x values")
            .split('=')
            .nth(1)
            .expect("Direction should equal something")
            .split("..")
            .map(|val| val.parse::<i64>().expect("Couldn't parse value"))
            .collect::<Vec<i64>>();
        let y_values = coordinate_ranges
            .iter()
            .find(|range| range.contains('y'))
            .expect("No y values")
            .split('=')
            .nth(1)
            .expect("Direction should equal something")
            .split("..")
            .map(|val| val.parse::<i64>().expect("Couldn't parse value"))
            .collect::<Vec<i64>>();

        let min_x = x_values[0];
        let max_x = x_values[1];
        let min_y = y_values[0];
        let max_y = y_values[1];

        Self { min_x, max_x, min_y, max_y }
    }

    fn does_trajectory_hit(&self, starting_x: i64, starting_y: i64) -> bool {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut velocity_x = starting_x;
        let mut velocity_y = starting_y;

        while current_x <= self.max_x && current_y >= self.min_y {
            if current_x >= self.min_x && current_y <= self.max_y { return true; }
            current_x += velocity_x;
            current_y += velocity_y;
            if velocity_x > 0 { velocity_x -= 1; }
            velocity_y -= 1;
        }
    
        false
    }
}


#[cfg(test)]
mod tests {
    use super::day17;

    #[test]
    fn check_day17() {
        let input_lines = "target area: x=20..30, y=-10..-5"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day17(&input_lines), (45, 0));
    }
}
