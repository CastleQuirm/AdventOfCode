// Potential enhancements
// 1: Can we make calc_part more functional?  it's got a while loop and other ugly stuff?
// 2: We're doing the replace and split stuff on the input twice, maybe that'd be nicer to do just once
// 3: Some of the detail re: handling the first number and last number is a little unobvious.  Could try to improve its readability.

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    (
        calculate_answer(input_lines, set_multiply_operator),
        calculate_answer(input_lines, push_num_to_multiply),
    )
}

fn calculate_answer(
    input_lines: &[String],
    multiply_func: fn(&mut Vec<u64>, &mut u64, &mut Operator),
) -> u64 {
    input_lines
        .iter()
        .map(|line| calculate(line, multiply_func))
        .sum()
}

fn calculate(line: &str, multiply_func: fn(&mut Vec<u64>, &mut u64, &mut Operator)) -> u64 {
    let spaced_line = line.replace('(', "( ").replace(')', " )");
    let split_line = spaced_line.split(' ').collect::<Vec<&str>>();
    let mut ptr: usize = 0;
    calc_part(&split_line, &mut ptr, multiply_func)
}

fn calc_part(
    split_line: &[&str],
    ptr: &mut usize,
    multiply_func: fn(&mut Vec<u64>, &mut u64, &mut Operator),
) -> u64 {
    // println!("Entered new brackets");
    let mut final_product_seq: Vec<u64> = Vec::new();
    let mut num1 = 0;
    let mut num2 = 0;
    let mut operator = Operator::Add;

    while *ptr < split_line.len() {
        let read_element = split_line.get(*ptr).expect("Ptr exceeded the line");
        *ptr += 1;

        // println!(
        //     "Looking at character: {}",
        //     read_element.chars().next().unwrap()
        // );

        match read_element.chars().next().unwrap() {
            '(' => num2 = calc_part(split_line, ptr, multiply_func),
            '+' => operator = Operator::Add,
            '*' => multiply_func(&mut final_product_seq, &mut num1, &mut operator),
            ')' => {
                assert_eq!(operator, Operator::None);
                break;
            }
            _ => {
                num2 = read_element
                    .parse::<u64>()
                    .expect("Couldn't parse the number")
            }
        }

        if num2 != 0 {
            match operator {
                Operator::Add => num1 += num2,
                Operator::Multiply => num1 *= num2,
                Operator::None => unreachable!(),
            }

            num2 = 0;
            operator = Operator::None;
        }
    }

    final_product_seq.push(num1);
    // println!("Leaving brackets: product seq = {:?}", final_product_seq);
    final_product_seq.iter().product()
}

fn set_multiply_operator(
    _final_product_seq: &mut Vec<u64>,
    _num1: &mut u64,
    operator: &mut Operator,
) {
    *operator = Operator::Multiply;
}

fn push_num_to_multiply(final_product_seq: &mut Vec<u64>, num1: &mut u64, operator: &mut Operator) {
    final_product_seq.push(*num1);
    *num1 = 0;
    *operator = Operator::Add;
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
    None,
}

#[cfg(test)]
mod tests {
    use super::{calculate, push_num_to_multiply, set_multiply_operator};

    #[test]
    fn day18_part1_examples() {
        assert_eq!(
            calculate("1 + 2 * 3 + 4 * 5 + 6", set_multiply_operator),
            71
        );
        assert_eq!(
            calculate("1 + (2 * 3) + (4 * (5 + 6))", set_multiply_operator),
            51
        );
        assert_eq!(calculate("2 * 3 + (4 * 5)", set_multiply_operator), 26);
        assert_eq!(
            calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)", set_multiply_operator),
            437
        );
        assert_eq!(
            calculate(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                set_multiply_operator
            ),
            12240
        );
        assert_eq!(
            calculate(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                set_multiply_operator
            ),
            13632
        );
    }

    #[test]
    fn day18_part2_examples() {
        assert_eq!(
            calculate("1 + 2 * 3 + 4 * 5 + 6", push_num_to_multiply),
            231
        );
        assert_eq!(
            calculate("1 + (2 * 3) + (4 * (5 + 6))", push_num_to_multiply),
            51
        );
        assert_eq!(calculate("2 * 3 + (4 * 5)", push_num_to_multiply), 46);
        assert_eq!(
            calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)", push_num_to_multiply),
            1445
        );
        assert_eq!(
            calculate(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                push_num_to_multiply
            ),
            669060
        );
        assert_eq!(
            calculate(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                push_num_to_multiply
            ),
            23340
        );
    }
}
