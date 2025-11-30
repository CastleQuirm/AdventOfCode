// Potential improvements:
//

use std::{collections::HashMap, ops::RangeInclusive};

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
    let answer2 = process_ranges(&rules);
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
            action = (rule.rule)(&part);
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

fn process_ranges(rules: &HashMap<String, RuleSet>) -> u64 {
    let mut starting_ranges = HashMap::new();
    starting_ranges.insert('x', 1..=4000);
    starting_ranges.insert('m', 1..=4000);
    starting_ranges.insert('a', 1..=4000);
    starting_ranges.insert('s', 1..=4000);
    let mut candidate_ranges_and_actions = Vec::new();
    candidate_ranges_and_actions.push((
        Catalogue {
            ranges: starting_ranges,
        },
        "in".to_string(),
    ));
    let mut accepted_ranges: Vec<Catalogue> = Vec::new();

    while let Some((catalogue, label)) = candidate_ranges_and_actions.pop() {
        // Process the range through this rule, subdividing it into Ranges that do one of:
        // - Accept (push it onto accepted_ranges)
        // - Reject (just drop it)
        // - GoTo (push to the candidate_ranges_and_actions along with the action)

        let ruleset = rules.get(&label).expect("No rule");
        let mut processing_catalogue = catalogue.clone();

        for rule in &ruleset.rules {
            let (meets_criteria, doesnt_meet) = if let Some(test) = &rule.test {
                // See if the range needs to be subdivided
                let comp_range = processing_catalogue
                    .ranges
                    .get(&test.variable)
                    .expect("Must have variable");
                // Should do something neat here but my brain is melting
                if (*comp_range.start() > test.threshold && test.comparison == '>')
                    || (*comp_range.end() < test.threshold && test.comparison == '<')
                {
                    (Some(processing_catalogue), None)
                } else if (*comp_range.end() <= test.threshold && test.comparison == '>')
                    || (*comp_range.start() >= test.threshold && test.comparison == '<')
                {
                    (None, Some(processing_catalogue))
                } else {
                    assert!(comp_range.contains(&test.threshold));
                    let mut meet_criteria = processing_catalogue.clone();
                    let mut doesnt_meet = processing_catalogue.clone();
                    if test.comparison == '>' {
                        let _ = meet_criteria
                            .ranges
                            .insert(test.variable, test.threshold + 1..=*comp_range.end());
                        let _ = doesnt_meet
                            .ranges
                            .insert(test.variable, *comp_range.start()..=test.threshold);
                    } else {
                        assert_eq!(test.comparison, '<');
                        let _ = meet_criteria
                            .ranges
                            .insert(test.variable, *comp_range.start()..=test.threshold - 1);
                        let _ = doesnt_meet
                            .ranges
                            .insert(test.variable, test.threshold..=*comp_range.end());
                    }

                    (Some(meet_criteria), Some(doesnt_meet))
                }
            } else {
                // Final catchall, process the whole range
                (Some(processing_catalogue.clone()), None)
            };

            if let Some(meets_criteria) = meets_criteria {
                match &rule.action {
                    RuleResult::Accept => accepted_ranges.push(meets_criteria.clone()),
                    RuleResult::Reject => (),
                    RuleResult::GoTo { label } => {
                        candidate_ranges_and_actions.push((meets_criteria.clone(), label.clone()))
                    }
                    RuleResult::Continue => panic!(),
                }
            }
            if let Some(doesnt_meet) = doesnt_meet {
                processing_catalogue = doesnt_meet;
            } else {
                // Nothing left in this rule
                break;
            }
        }
    }

    accepted_ranges
        .iter()
        .map(|cat| {
            cat.ranges
                .values()
                .map(|range| range.end() - range.start() + 1)
                .product::<u64>()
        })
        .sum::<u64>()
}

struct RuleSet {
    rules: Vec<SingleRule>,
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
                if rule_txt.contains(['<', '>']) {
                    // main rule
                    let chars = rule_txt.chars().collect_vec();
                    let (variable, comparison) = (chars[0], chars[1]);
                    let (threshold, result) =
                        rule_txt[2..].split_once(':').expect("Couldn't parse rule");
                    let threshold = threshold.parse::<u64>().expect("Couldn't parse threshold");
                    let meet_threshold_result = RuleResult::from(result);
                    let meet_threshold_result_for_struct = meet_threshold_result.clone();
                    let rule = Box::new(move |part: &Part| match comparison {
                        '<' => {
                            if *part.values.get(&variable).expect("unknown variable") < threshold {
                                meet_threshold_result.clone()
                            } else {
                                RuleResult::Continue
                            }
                        }
                        '>' => {
                            if *part.values.get(&variable).expect("unknown variable") > threshold {
                                meet_threshold_result.clone()
                            } else {
                                RuleResult::Continue
                            }
                        }
                        _ => panic!(),
                    });
                    SingleRule {
                        rule,
                        test: Some(Test {
                            variable,
                            comparison,
                            threshold,
                        }),
                        action: meet_threshold_result_for_struct,
                    }
                } else {
                    // Fall-through rule
                    let action = RuleResult::from(rule_txt.as_str());
                    let action_for_struct = action.clone();
                    SingleRule {
                        rule: Box::new(move |_: &Part| action.clone()),
                        test: None,
                        action: action_for_struct,
                    }
                }
            })
            .collect_vec();
        (rule_name, Self { rules })
    }
}

struct SingleRule {
    rule: Box<dyn Fn(&Part) -> RuleResult>, // Used for part 1 'elegance'
    test: Option<Test>,
    action: RuleResult,
}

struct Test {
    variable: char,
    comparison: char,
    threshold: u64,
}

#[derive(Debug, Clone)]
struct Part {
    values: HashMap<char, u64>,
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
                    let (variable, value) = v.split_once('=').expect("Invalid part");
                    let mut variable_iter = variable.chars();
                    let variable = variable_iter.next().unwrap();
                    assert!(variable_iter.next().is_none());
                    (variable, value.parse::<u64>().expect("Invalid value"))
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

/// A Catalogue is a conecptual selection of Ranges for parts!
#[derive(Clone, Debug)]
struct Catalogue {
    ranges: HashMap<char, RangeInclusive<u64>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RuleResult {
    Accept,
    Reject,
    Continue,
    GoTo { label: String },
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            label => RuleResult::GoTo {
                label: label.to_string(),
            },
        }
    }
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
            "19114",           // PART 1 RESULT
            "167409079868000", // PART 2 RESULT
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
