mod day00;
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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::env;

type DayFunction = fn(&str) -> (String, String);
const DAY_FUNCTIONS: [DayFunction; 26] = [
    day00::day00,
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
        max_day = DAY_FUNCTIONS.len() - 1;
    } else {
        min_day = args[1]
            .parse::<usize>()
            .expect("Please provide the day number as an integer.");
        if min_day > DAY_FUNCTIONS.len() {
            panic!("Invalid day specified.");
        }
        max_day = min_day;
    }

    for day in min_day..=max_day {
        println!("Day {}", day);
        let input_lines = load_input(day);
        let start_time = std::time::Instant::now();
        let (part1, part2) = DAY_FUNCTIONS[day](&input_lines);
        let elapsed = start_time.elapsed().as_micros();
        println!("Part 1: {}\nPart 2: {}", part1, part2);
        println!("{}.{:03}ms", elapsed / 1000, elapsed % 1000);
        println!("----------");
    }

    pub fn load_input(day: usize) -> String {
        std::fs::read_to_string(format!("{}/{}", "inputs", day))
            .expect("Can't open/read input file")
    }
}
