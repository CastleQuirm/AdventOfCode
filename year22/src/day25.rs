use std::{fmt::Display, ops::Add, str::FromStr, string::ParseError};

pub fn day25(input_lines: &str) -> (String, String) {
    (
        input_lines
            .lines()
            .map(|line| line.parse::<Base5Sym>().unwrap())
            .fold(Base5Sym::zero(), |acc, x| acc + x)
            .to_string(),
        "MERRY CHRISTMAS!".to_string(),
    )
}

struct Base5Sym {
    value: Vec<i8>,
}

impl Base5Sym {
    fn zero() -> Self {
        Self { value: Vec::new() }
    }
}

impl FromStr for Base5Sym {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s
                .chars()
                .rev()
                .map(|c| match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!(),
                })
                .collect(),
        })
    }
}

impl Display for Base5Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .value
            .iter()
            .rev()
            .map(|i| match i {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => unreachable!(),
            })
            .collect::<String>();
        write!(f, "{}", text)
    }
}

// impl ToString for Base5Sym {
//     fn to_string(&self) -> String {
//         self.value
//             .iter()
//             .rev()
//             .map(|i| match i {
//                 2 => '2',
//                 1 => '1',
//                 0 => '0',
//                 -1 => '-',
//                 -2 => '=',
//                 _ => unreachable!(),
//             })
//             .collect::<String>()
//     }
// }

impl Add for Base5Sym {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut carry_over_digit = 0;
        let mut sum = Vec::new();
        let mut i = 0;

        while i < self.value.len() || i < rhs.value.len() || carry_over_digit != 0 {
            let mut value =
                self.value.get(i).unwrap_or(&0) + rhs.value.get(i).unwrap_or(&0) + carry_over_digit;
            if value < -2 {
                value += 5;
                carry_over_digit = -1;
            } else if value > 2 {
                value -= 5;
                carry_over_digit = 1;
            } else {
                carry_over_digit = 0;
            }

            sum.push(value);

            i += 1;
        }

        Self { value: sum }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day25_both_case1() {
        assert_eq!(
            day25(
                "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
            ),
            ("2=-1=0".to_string(), "MERRY CHRISTMAS!".to_string())
        )
    }
}
