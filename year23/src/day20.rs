// Potential improvements:
// - The use of two different HashMaps to initialize the state is a slightly tacky workaround
//   for the fact I want to (immutably) iterate through the network looking for sources and
//   then mutably update another element in the network. It shouldn't ever go to itself but
//   I don't know that we can tell Rust that.  We could probably do differently by (a) working
//   the other way around (iterate mutably through Conjunctions, then search the list for sources)
//   which should make the borrows happy but converts the looping to O^2, or possibly (b) improving
//   borrow referencing, maybe using .iter().for_each() instead of for x in y?

use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub fn day20(input_lines: &[Vec<String>]) -> (String, String) {
    // Read and initialize state

    // Initial read - this sets up the FlipFlops and Broadcast correctly, and creates the
    // Conjunctions, but the latter need to know their inputs which we don't set up until the
    // following step.
    let network_with_empty_conjunctions: HashMap<String, Module> = input_lines[0]
        .iter()
        .map(|line| Module::from(line))
        .collect::<HashMap<_, _>>();

    // Create an updated list with filled Conjunctions.
    let mut network = network_with_empty_conjunctions.clone();
    for (module_name, module) in &network_with_empty_conjunctions {
        for target in module.get_connections() {
            if let Some(Module::Conjunction {
                input_pulses,
                connections: _,
            }) = network.get_mut(target)
            {
                input_pulses.insert(module_name.clone(), PulseType::Low);
            }
        }
    }

    // Find the output conjunctions. We want to know when they will each produce a High out.
    // Assume a fixed structure of one final conjunction goes to RX, and each input to it is the
    // output of an independent subnetwork which outputs a single High once every cycle that also
    // resets the state of that subnetwork.
    let output_module = network
        .iter()
        .find(|(_, module)| *module.get_connections() == vec!["rx"])
        .map(|(name, _)| name.to_string())
        .unwrap_or_default();
    let subnetwork_egresses = network
        .iter()
        .filter_map(|(name, module)| {
            if *module.get_connections() == [output_module.to_string()] {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Naive part 1: just press the button 1000 times.
    let (high_count, low_count) = (1..=1000).fold((0, 0), |(cur_high, cur_low), _| {
        // Press the button and run until the pulses have finished.
        let (new_high, new_low, sunbnetwork_complete) =
            press_button(&mut network, &subnetwork_egresses);
        assert!(sunbnetwork_complete.is_none());
        ((cur_high + new_high), (cur_low + new_low))
    });

    let answer1 = high_count * low_count;

    // Part 2 doesn't have an answer in the test cases so we'll hack around it.
    let mut subnetwork_cycle_values = HashMap::new();

    // We'll assume each subsection answer is more than 1000. (They are).
    if input_lines[0].len() > 10 {
        // Keep pushing the button.
        let mut button_pushes = 1000u64;
        while subnetwork_cycle_values.len() < subnetwork_egresses.len() {
            button_pushes += 1;
            if let Some(submodule_egress) = press_button(&mut network, &subnetwork_egresses).2 {
                // For simplicity we'll assume all the submodules finish before any one of them finishes twice.
                // If this weren't the case we'd just have to not overwrite, which is easy but I don't need to
                // do it given my input.
                assert!(subnetwork_cycle_values.get(&submodule_egress).is_none());
                subnetwork_cycle_values.insert(submodule_egress, button_pushes);
            }
        }
    }

    // This is assuming that (a) the High input into the final Conjunction module occurs as the last step of a cycle that
    // complete resets the sub-network (so we can just do a LCM), and (b) that the subnetworks' cycles are co-prime (so
    // just a product).
    let answer2 = subnetwork_cycle_values.values().product::<u64>();

    (format!("{}", answer1), format!("{:?}", answer2))
}

fn press_button(
    network: &mut HashMap<String, Module>,
    subnetwork_egresses: &[String],
) -> (u64, u64, Option<String>) {
    let mut pulses_to_resolve = VecDeque::from([SentPulse {
        source: "button".to_string(),
        pulse: PulseType::Low,
        target: "broadcaster".to_string(),
    }]);
    let (mut high_count, mut low_count, mut subnetwork_complete) = (0, 0, None);

    while let Some(pulse) = pulses_to_resolve.pop_front() {
        match pulse.pulse {
            PulseType::High => {
                high_count += 1;
                if subnetwork_egresses.contains(&pulse.source) {
                    subnetwork_complete = Some(pulse.source.clone())
                }
            }
            PulseType::Low => low_count += 1,
        }
        let mut new_pulses = network
            .get_mut(&pulse.target)
            .map(|module| module.receive_input(pulse))
            .unwrap_or_default();
        pulses_to_resolve.append(&mut new_pulses);
    }

    (high_count, low_count, subnetwork_complete)
}

#[derive(Clone, Debug)]
enum Module {
    FlipFlop {
        on: bool,
        connections: Vec<String>,
    },
    Conjunction {
        input_pulses: HashMap<String, PulseType>,
        connections: Vec<String>,
    },
    Broadcaster {
        connections: Vec<String>,
    },
}

impl Module {
    fn from(line: &str) -> (String, Self) {
        let (module_name, targets) = line.split_once(" -> ").expect("Bad input line");
        let connections = targets.split(", ").map(String::from).collect_vec();
        match module_name.chars().next().expect("Empty first string") {
            '&' => (
                module_name[1..].to_owned(),
                Self::Conjunction {
                    input_pulses: HashMap::new(),
                    connections,
                },
            ),
            '%' => (
                module_name[1..].to_owned(),
                Self::FlipFlop {
                    on: false,
                    connections,
                },
            ),
            'b' => {
                assert_eq!(module_name, "broadcaster");
                (module_name.to_owned(), Self::Broadcaster { connections })
            }
            _ => panic!(),
        }
    }

    fn get_connections(&self) -> &Vec<String> {
        match self {
            Module::FlipFlop { on: _, connections }
            | Module::Conjunction {
                input_pulses: _,
                connections,
            }
            | Module::Broadcaster { connections } => connections,
        }
    }

    /// Processes an input Pulse for this module, updating its state and returning a list of
    /// output Pulses to process.
    fn receive_input(&mut self, input: SentPulse) -> VecDeque<SentPulse> {
        match self {
            Module::FlipFlop { on, connections } => {
                if input.pulse == PulseType::Low {
                    // Low input flips the state and sends a pulse for the new state
                    *on = !(*on);
                    connections
                        .iter()
                        .map(|dest| SentPulse {
                            source: input.target.clone(),
                            pulse: if *on { PulseType::High } else { PulseType::Low },
                            target: dest.clone(),
                        })
                        .collect::<VecDeque<SentPulse>>()
                } else {
                    // Nothing happens on a High input.
                    VecDeque::new()
                }
            }
            Module::Conjunction {
                input_pulses,
                connections,
            } => {
                // Record the received pulse for the given input
                assert!(input_pulses.contains_key(&input.source));
                input_pulses
                    .entry(input.source)
                    .and_modify(|e| *e = input.pulse);
                connections
                    .iter()
                    .map(|dest| SentPulse {
                        source: input.target.clone(),
                        pulse: if input_pulses.values().all(|p| *p == PulseType::High) {
                            PulseType::Low
                        } else {
                            PulseType::High
                        },
                        target: dest.clone(),
                    })
                    .collect::<VecDeque<SentPulse>>()
            }
            Module::Broadcaster { connections } => connections
                .iter()
                .map(|dest| SentPulse {
                    source: input.target.clone(),
                    pulse: input.pulse.clone(),
                    target: dest.clone(),
                })
                .collect::<VecDeque<SentPulse>>(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum PulseType {
    High,
    Low,
}

#[derive(PartialEq, Eq, Debug)]
struct SentPulse {
    source: String,
    pulse: PulseType,
    target: String,
}

#[cfg(test)]
mod tests {
    use super::day20;
    use crate::utils::load_input;

    #[test]
    fn check_day20_case01() {
        full_test(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a", // INPUT STRING
            "32000000", // PART 1 RESULT
            // Part 2 doesn't have a test, so we just return 1 (as the product of an empty iterator)
            "1", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day20_case02() {
        full_test(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output", // INPUT STRING
            "11687500", // PART 1 RESULT
            // Part 2 doesn't have a test, so we just return 1 (as the product of an empty iterator)
            "1", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day20(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// The four sub-sections of my network produce the high input into the final
// 4073 * 4091 * 4093 * 3853
