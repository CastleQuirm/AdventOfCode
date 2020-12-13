// Potential improvements:
// 1: Try and make part2_calc() more readable, somehow.
// 2: Clean up the assert in bus_to_num_and_wait() so we behave correctly in the off-chance that there is a 0 answer.

pub fn day13(input_lines: &[String]) -> (u64, u64) {
    let arrival_time = input_lines[0]
        .parse::<u64>()
        .expect("Couldn't read first line number");
    let bus_list = input_lines[1]
        .split(',')
        .map(|slot| slot.parse::<u64>())
        .collect::<Vec<Result<u64, _>>>();
    (part1_calc(arrival_time, &bus_list), part2_calc(&bus_list))
}

fn part1_calc(arrival_time: u64, bus_list: &[Result<u64, std::num::ParseIntError>]) -> u64 {
    let (chosen_bus, wait) = bus_list
        .iter()
        .filter(|entry| entry.is_ok())
        .map(|bus| bus_to_num_and_wait(arrival_time, *bus.as_ref().expect("Filtered out the x's")))
        .fold((0, arrival_time), |previous, new| {
            if new.1 < previous.1 {
                new
            } else {
                previous
            }
        });
    chosen_bus * wait
}

fn bus_to_num_and_wait(arrival_time: u64, bus: u64) -> (u64, u64) {
    let wait = bus - (arrival_time % bus);
    // The assert's in here because if it fails, we'd need to change the value of wait.
    // Mechanically it could fail, but if it does, the answer (to Part 1) is just 0, which seems unlikely.
    assert!(wait != bus);
    (bus, wait)
}

fn part2_calc(bus_list: &[Result<u64, std::num::ParseIntError>]) -> u64 {
    (0..bus_list.len())
        .fold((0, 1 as u64), |(candidate_t, lcm), i| {
            match bus_list[i] {
                Ok(bus_num) => {
                    // find minimum X such that (candidate_t + X * lcm + i) % bus_num == 0; then return ((candidate_t + X * lcm), LCM(lcm, bus_num))
                    let x = find_x(candidate_t, lcm, i, bus_num);
                    (
                        candidate_t + x * lcm,
                        extended_rational::lcm(lcm, bus_num).expect("LCM overflowed"),
                    )
                }
                Err(_) => (candidate_t, lcm),
            }
        })
        .0
}

fn find_x(candidate_t: u64, lcm: u64, i: usize, bus_num: u64) -> u64 {
    (0..bus_num)
        .find(|x| (candidate_t + x * lcm + i as u64) % bus_num == 0)
        .expect("Didn't find a multiple; that's impossible!")
}

#[cfg(test)]
mod tests {
    use super::{day13, part2_calc};

    #[test]
    fn sample_input() {
        let sample = "939
7,13,x,x,59,x,31,19"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day13(&sample), (295, 1068781));
    }

    #[test]
    fn check_rounding() {
        let first_bus = 7;
        let test_number: u64 = ((100 / first_bus) + 1) * first_bus;
        assert_eq!(test_number, 105);
    }

    #[test]
    fn smaller_examples() {
        let example_1 = "17,x,13,19"
            .split(',')
            .map(|slot| slot.parse::<u64>())
            .collect::<Vec<Result<u64, _>>>();
        let example_2 = "67,7,59,61"
            .split(',')
            .map(|slot| slot.parse::<u64>())
            .collect::<Vec<Result<u64, _>>>();
        let example_3 = "67,x,7,59,61"
            .split(',')
            .map(|slot| slot.parse::<u64>())
            .collect::<Vec<Result<u64, _>>>();
        let example_4 = "67,7,x,59,61"
            .split(',')
            .map(|slot| slot.parse::<u64>())
            .collect::<Vec<Result<u64, _>>>();
        let example_5 = "1789,37,47,1889"
            .split(',')
            .map(|slot| slot.parse::<u64>())
            .collect::<Vec<Result<u64, _>>>();
        assert_eq!(part2_calc(&example_1), 3417);
        assert_eq!(part2_calc(&example_2), 754018);
        assert_eq!(part2_calc(&example_3), 779210);
        assert_eq!(part2_calc(&example_4), 1261476);
        assert_eq!(part2_calc(&example_5), 1202161486);
    }
}
