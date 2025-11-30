// Potential improvements:
//

pub fn day25(input_lines: &[Vec<String>]) -> (String, String) {
    let schematics = input_lines
        .iter()
        .map(|diagram| Schematic::from_diagram(diagram))
        .collect::<Vec<Schematic>>();

    let answer1 = (0..schematics.len())
        .map(|i| {
            (0..i)
                .filter(|j| schematics[i].candidate_match(&schematics[*j]))
                .count()
        })
        .sum::<usize>();

    let answer2 = "MERRY CHRISTMAS";
    (format!("{}", answer1), format!("{}", answer2))
}

struct Schematic {
    s_type: SchematicType,
    columns: [usize; 5],
}

impl Schematic {
    fn from_diagram(lines: &[String]) -> Self {
        let s_type = if lines[0].chars().nth(0).unwrap() == '#' {
            SchematicType::Lock
        } else {
            SchematicType::Key
        };

        let mut columns= [0; 5];
        match s_type {
            SchematicType::Lock => {
                for column_ix in 0..5 {
                    columns[column_ix] = (1usize..7).find(|&i| lines[i].chars().nth(column_ix).unwrap() == '.').unwrap() - 1;
                }
            }
            SchematicType::Key => {
                for column_ix in 0..5 {
                    columns[column_ix] = 5 - (0usize..6).rev().find(|&i| lines[i].chars().nth(column_ix).unwrap() == '.').unwrap();
                }
            },
        }

        Self { s_type, columns }
    }

    fn candidate_match(&self, other: &Self) -> bool {
        self.s_type != other.s_type
            && (0..5).all(|col_ix| self.columns[col_ix] + other.columns[col_ix] <= 5)
    }
}

#[derive(PartialEq, Eq)]
enum SchematicType {
    Lock,
    Key,
}

#[cfg(test)]
mod tests {
    use super::day25;
    use crate::utils::load_input;

    #[test]
    fn check_day25_case01() {
        full_test(
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####", // INPUT STRING
            "3", // PART 1 RESULT
            "MERRY CHRISTMAS", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day25(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
