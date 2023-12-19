// Potential improvements:
//

use std::collections::HashMap;

use itertools::Itertools;

pub fn day19(input_lines: &[Vec<String>]) -> (String, String) {
    assert_eq!(input_lines.len(), 2);

    let rules = input_lines[0]
        .iter()
        .map(|line| RuleSet::from(line))
        .collect::<HashMap<String, RuleSet>>();
    let answer1 = input_lines[1]
        .iter()
        .filter_map(|line| process(line, &rules))
        .sum::<u64>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn process(line: &str, rules: &HashMap<String, RuleSet>) -> Option<u64> {
    let part = Part::from(line);
    let mut action = RuleResult::GoTo {
        label: "in".to_string(),
    };

    while action != RuleResult::Accept && action != RuleResult::Reject {
        let ruleset = rules
            .get(action.get_goto_label())
            .expect("Couldn't find rule");
        for rule in &ruleset.rules {
            action = rule(part.clone());
            if action != RuleResult::Continue {
                break;
            }
        }
    }

    if action == RuleResult::Accept {
        Some(part.values.values().sum::<u64>())
    } else {
        assert_eq!(action, RuleResult::Reject);
        None
    }
}

struct RuleSet {
    rules: Vec<Box<dyn Fn(Part) -> RuleResult>>,
}

impl RuleSet {
    fn from(line: &str) -> (String, Self) {
        let mut sections = line.split('{');
        let rule_name = sections.next().expect("No text?").to_owned();
        let rules_str = sections
            .next()
            .and_then(|rules| rules.strip_suffix('}'))
            .expect("Parsing failed")
            .to_owned();
        assert!(sections.next().is_none());
        let rules = rules_str
            .split(',')
            .map(|rule_txt| {
                let rule_txt = rule_txt.to_owned();
                if rule_txt.contains(|c| c == '<' || c == '>') {
                    // main rule
                    let chars = rule_txt.chars().collect_vec();
                    let (variable, comparison) = (chars[0].to_string(), chars[1]);
                    let (threshold, result) =
                        rule_txt[2..].split_once(':').expect("Couldn't parse rule");
                    let threshold = threshold.parse::<u64>().expect("Couldn't parse threshold");
                    // let result_owned = result.to_owned();
                    // TODO this could be commonised with the else branch
                    let result = match result {
                        "A" => RuleResult::Accept,
                        "R" => RuleResult::Reject,
                        _ => RuleResult::GoTo {
                            label: result.to_string(),
                        },
                    };
                    Box::new(move |part: Part| match comparison {
                        '<' => {
                            if *part.values.get(&variable).expect("unknown variable") < threshold {
                                result.clone()
                            } else {
                                RuleResult::Continue
                            }
                        }
                        '>' => {
                            if *part.values.get(&variable).expect("unknown variable") > threshold {
                                result.clone()
                            } else {
                                RuleResult::Continue
                            }
                        }
                        _ => panic!(),
                    })
                } else {
                    // Fall-through rule
                    match rule_txt.as_str() {
                        "A" => {
                            Box::new(move |_| RuleResult::Accept) as Box<dyn Fn(Part) -> RuleResult>
                        }
                        "R" => Box::new(move |_| RuleResult::Reject),
                        _ => Box::new(move |_| RuleResult::GoTo {
                            label: rule_txt.to_string(),
                        }),
                    }
                }
            })
            .collect_vec();
        (rule_name, Self { rules })
    }
}

#[derive(Debug, Clone)]
struct Part {
    values: HashMap<String, u64>,
}

impl From<&str> for Part {
    fn from(line: &str) -> Self {
        Self {
            values: line
                .strip_prefix('{')
                .and_then(|l| l.strip_suffix('}'))
                .expect("Bad line")
                .split(',')
                .map(|v| {
                    let (property, value) = v.split_once('=').expect("Invalid part");
                    (
                        property.to_string(),
                        value.parse::<u64>().expect("Invalid value"),
                    )
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RuleResult {
    Accept,
    Reject,
    Continue,
    GoTo { label: String },
}

impl RuleResult {
    /// Get the GoTo label. Panics if called on a type other than ``RuleResult::GoTo``.
    fn get_goto_label(&self) -> &str {
        match self {
            RuleResult::GoTo { label } => label,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day19;
    use crate::utils::load_input;

    #[test]
    fn check_day19_case01() {
        full_test(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}", // INPUT STRING
            "19114", // PART 1 RESULT
            "0",     // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day19(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
