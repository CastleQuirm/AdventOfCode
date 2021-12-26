use std::collections::HashSet;

// Potential improvements:
// Seems alright, I think!

pub fn day25(input_lines: &[String]) -> (u64, u64) {
    let mut current_state = CucumberState::new(input_lines);
    let mut next_state = current_state.cucumber_move();
    let mut moves_to_stable = 1;
    while current_state != next_state {
        moves_to_stable += 1;
        current_state = next_state;
        next_state = current_state.cucumber_move();
    }

    println!("MERRY CHRISTMAS!");

    (moves_to_stable, 0)
}

#[derive(PartialEq, Eq)]
struct CucumberState {
    east_cucumbers: HashSet<Coord>,
    south_cucumbers: HashSet<Coord>,
    map_x_size: usize,
    map_y_size: usize,
}

impl CucumberState {
    fn new(input_lines: &[String]) -> Self {
        let mut east_cucumbers = HashSet::new();
        let mut south_cucumbers = HashSet::new();
        let map_y_size = input_lines.len();
        let map_x_size = input_lines[0].len();
        input_lines.iter().enumerate().for_each(|(y, line)| {
            assert_eq!(map_x_size, line.len());
            line.chars().enumerate().for_each(|(x, space)| match space {
                '>' => {
                    east_cucumbers.insert(Coord { x, y });
                }
                'v' => {
                    south_cucumbers.insert(Coord { x, y });
                }
                _ => assert_eq!(space, '.'),
            });
        });

        Self {
            east_cucumbers,
            south_cucumbers,
            map_x_size,
            map_y_size,
        }
    }

    fn cucumber_move(&self) -> Self {
        // move east
        let east_cucumbers = self
            .east_cucumbers
            .iter()
            .map(|cucumber| {
                // Where would this cucumber want to move to?
                let target_x = if cucumber.x + 1 < self.map_x_size {
                    cucumber.x + 1
                } else {
                    0
                };
                let target_loc = Coord {
                    x: target_x,
                    y: cucumber.y,
                };
                if !self.east_cucumbers.contains(&target_loc)
                    && !self.south_cucumbers.contains(&target_loc)
                {
                    target_loc
                } else {
                    *cucumber
                }
            })
            .collect::<HashSet<Coord>>();

        // move south
        let south_cucumbers = self
            .south_cucumbers
            .iter()
            .map(|cucumber| {
                // Where would this cucumber want to move to?
                let target_y = if cucumber.y + 1 < self.map_y_size {
                    cucumber.y + 1
                } else {
                    0
                };
                let target_loc = Coord {
                    x: cucumber.x,
                    y: target_y,
                };
                if !east_cucumbers.contains(&target_loc)
                    && !self.south_cucumbers.contains(&target_loc)
                {
                    target_loc
                } else {
                    *cucumber
                }
            })
            .collect::<HashSet<Coord>>();

        Self {
            east_cucumbers,
            south_cucumbers,
            map_x_size: self.map_x_size,
            map_y_size: self.map_y_size,
        }
    }

    // Function for printing the state of the map.
    // fn display(&self) {
    //     for y in 0..self.map_y_size {
    //         for x in 0..self.map_x_size {
    //             let print_coord = Coord{ x, y };
    //             if self.east_cucumbers.contains(&print_coord) {
    //                 print!(">");
    //                 assert!(!self.south_cucumbers.contains(&print_coord));
    //             } else if self.south_cucumbers.contains(&print_coord) {
    //                 print!("v");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    // }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::day25;

    #[test]
    fn check_day25() {
        let input_lines = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day25(&input_lines), (58, 0));
    }
}
