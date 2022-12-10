pub fn day10(input_lines: &str) -> (String, String) {
    let mut register = 1;
    let mut accumulator = 0;
    let mut cycle = 1;
    let mut answer2 = "\n".to_string();

    for line in input_lines.lines() {
        let mut line_parts = line.split_ascii_whitespace();
        match line_parts.next().expect("No line?") {
            "noop" => {
                accumulator += in_cycle_process(&mut cycle, register, &mut answer2);
            }
            "addx" => {
                accumulator += in_cycle_process(&mut cycle, register, &mut answer2);
                accumulator += in_cycle_process(&mut cycle, register, &mut answer2);
                register += line_parts
                    .next()
                    .map(|s| s.parse::<i32>().expect("Oh no"))
                    .expect("No addx value");
            }
            _ => panic!(),
        };
        // if cycle > 220 {
        //     break;
        // }
    }

    let answer1 = accumulator;
    (format!("{}", answer1), answer2.replace('.', " "))
}

fn in_cycle_process(cycle: &mut i32, register: i32, screen: &mut String) -> i32 {
    let return_value = if *cycle % 40 == 20 {
        register * *cycle
    } else {
        0
    };
    print_char(cycle, register, screen);
    *cycle += 1;
    return_value
}

fn print_char(cycle: &i32, register: i32, screen: &mut String) {
    let column_traced = (cycle - 1) % 40;
    let char_traced = if (column_traced - register).abs() < 2 {
        '#'
    } else {
        '.'
    };
    screen.push(char_traced);

    if cycle % 40 == 0 {
        screen.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day10_both_case1() {
        assert_eq!(
            day10(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            ),
            (
                "13140".to_string(),
                "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .replace('.', " ")
                .to_string()
            )
        )
    }
}
