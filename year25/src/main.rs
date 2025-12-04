mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod utils;

use std::env;

type DayFunction = fn(&[Vec<String>]) -> (String, String);
static DAY_FUNCTIONS: [DayFunction; 12] = [
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    day09::day09,
    day10::day10,
    day11::day11,
    day12::day12,
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
        let input_lines: Vec<Vec<String>> = load_input(day);
        let start_time = std::time::Instant::now();
        let (part1, part2) = DAY_FUNCTIONS[day - 1](&input_lines);
        let elapsed = start_time.elapsed().as_micros();
        println!("Part 1: {}\nPart 2: {}", part1, part2);
        println!("{}.{:03}ms", elapsed / 1000, elapsed % 1000);
        println!("----------");
    }

    pub fn load_input(day: usize) -> Vec<Vec<String>> {
        let whole_input = std::fs::read_to_string(format!("{}/{}", "inputs", day))
            .expect("Can't open/read input file");
        utils::load_input(&whole_input)
    }
}
