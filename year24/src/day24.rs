// Potential improvements:
//

use std::collections::HashMap;

use num::pow;

pub fn day24(input_lines: &[Vec<String>]) -> (String, String) {
    let mut wire_states: HashMap<String, bool> = HashMap::new();
    for line in &input_lines[0] {
        wire_states.insert(line[0..3].to_string(), line[5..6] == *"1");
    }
    let base_states = wire_states.clone();

    let all_gates = input_lines[1]
        .iter()
        .map(|line| {
            let values = line.split_ascii_whitespace().collect::<Vec<_>>();
            let z_value = if values[4].starts_with('z') {
                Some(values[4][1..].parse::<usize>().expect("Bad parse"))
            } else {
                None
            };
            let mut inputs = vec![values[0].to_string(), values[2].to_string()];
            inputs.sort();
            LogicGate {
                inputs,
                output: values[4].to_string(),
                logic_type: match values[1] {
                    "AND" => LogicType::And,
                    "OR" => LogicType::Or,
                    "XOR" => LogicType::Xor,
                    _ => panic!(),
                },
                z_value,
            }
        })
        .collect::<Vec<LogicGate>>();

    let answer1 = solve(&all_gates, &wire_states);

    let mut bad_indices = Vec::new();
    (0..45).for_each(|i| {
        wire_states = base_states.clone();
        base_states.keys().for_each(|key| {
            wire_states
                .entry(key.clone())
                .and_modify(|val| *val = false);
        });
        wire_states
            .entry(format!("y{:02}", i))
            .and_modify(|val| *val = true);
        if 2u64.pow(i) != solve(&all_gates, &wire_states) {
            bad_indices.push(i);
        }
        // println!("0 + 2^{i} = {}", solve(&all_gates, &wire_states));
    });
    println!("Bad indices: {:?}", bad_indices);

    // How does binary addition work?
    // Each target bit is the three-way XOR of the two source bits at the same level and the carry-over from the bit below.
    // The carry-over is then the three-way AT-LEAST-TWO of those three bits.

    // Z18 is wrong (shouldn't be an output of x18 AND y18!)

    // So expect x00 XOR y00 -> z00
    //           x00 AND y00 -> jfw // carry-bit from units

    //           x01 XOR y01 -> gnj
    //           x01 AND y01 -> ntt
    //           jfw XOR gnj -> z01
    //           jfw AND gnj -> spq // Set if carry-bit and exactly one current-bit
    //           ntt OR spq -> ndd  // carry-bit from ones

    // So for each number -> find the XOR output, it should be used in two inputs, an XOR and an AND, each with the same partner.
    // The partner should be the carried-up bit
    // The XOR should give the z for the same number
    // The AND should then get ORd with the AND of the x/y AND output, and gives the next carry-over.

    //           x02 XOR y02 -> jgw (assuming right)
    //           ndd XOR jgw -> z02
    //           ndd AND jgw -> qnq

    //

    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn solve(all_gates: &[LogicGate], starting_bits: &HashMap<String, bool>) -> u64 {
    let mut answer = 0u64;
    let mut unused_gates: Vec<LogicGate> = all_gates.to_vec();
    let mut wire_states = starting_bits.clone();
    while !unused_gates.is_empty() {
        let mut still_unsolved = Vec::new();
        for gate in unused_gates {
            if let Some(output_val) = wire_states.get(&gate.inputs[0]).and_then(|wire_one| {
                wire_states
                    .get(&gate.inputs[1])
                    .map(|wire_two| gate.logic_type.resolve(wire_one, wire_two))
            }) {
                wire_states.insert(gate.output.clone(), output_val);
                if let Some(z_ix) = gate.z_value {
                    if output_val {
                        answer += pow(2, z_ix);
                    }
                }
            } else {
                still_unsolved.push(gate.clone());
            }
        }
        unused_gates = still_unsolved;
    }
    answer
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LogicGate {
    inputs: Vec<String>,
    output: String,
    logic_type: LogicType,
    z_value: Option<usize>,
}

// struct BitVals {
//     xor: Option<String>,
//     and: Option<String>,
//     xor_with_carry_up: Option<String>,
//     and_with_carry_up: Option<String>,
//     carry_over: Option<String>,
// }

// impl Default for BitVals {
//     fn default() -> Self {
//         Self { xor: None, and: None, xor_with_carry_up: None, and_with_carry_up: None, carry_over: None }
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum LogicType {
    And,
    Or,
    Xor,
}

impl LogicType {
    fn resolve(&self, wire_one: &bool, wire_two: &bool) -> bool {
        match self {
            LogicType::And => *wire_one && *wire_two,
            LogicType::Or => *wire_one || *wire_two,
            LogicType::Xor => (*wire_one || *wire_two) && !(*wire_one && *wire_two),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day24;
    use crate::utils::load_input;

    #[test]
    fn check_day24_case01() {
        full_test(
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj", // INPUT STRING
            "2024", // PART 1 RESULT
            "0",    // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day24(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
