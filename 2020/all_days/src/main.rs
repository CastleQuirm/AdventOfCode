mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use std::env;

type DayFunction = fn(&[String]) -> (u64, u64);
static DAY_FUNCTIONS: [DayFunction; 25] = [
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    day7::day7,
    day8::day8,
    day9::day9,
    day10::day10,
    day11::day11,
    day12::day12,
    day13::day13,
    day14::day14,
    day15::day15,
    day16::day16,
    day17::day17,
    day18::day18,
    day19::day19,
    day20::day20,
    day21::day21,
    day22::day22,
    day23::day23,
    day24::day24,
    day25::day25,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let min_day: usize;
    let max_day: usize;
    if args.len() < 2 {
        min_day = 1;
        max_day = DAY_FUNCTIONS.len();
    } else {
        min_day = args[1]
            .parse::<usize>()
            .expect("Please provide the day number as an integer.");
        if (min_day < 1) || (min_day > DAY_FUNCTIONS.len()) {
            panic!("Invalid day specified.");
        }
        max_day = min_day;
    }

    for day in min_day..=max_day {
        println!("Day {}", day);
        let input_lines = utils::load_inputs(day);
        let start_time = std::time::Instant::now();
        let (part1, part2) = DAY_FUNCTIONS[day - 1](&input_lines);
        let elapsed = start_time.elapsed().as_micros();
        println!("Part 1: {}\nPart 2: {}", part1, part2);
        println!("{}.{}ms", elapsed / 1000, elapsed % 1000);
        println!("----------");
    }
}
