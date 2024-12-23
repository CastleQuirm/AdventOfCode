// Potential improvements:
//

use std::collections::{HashMap, HashSet};

pub fn day23(input_lines: &[Vec<String>]) -> (String, String) {
    let mut network_outs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut triplets: HashSet<Vec<&str>> = HashSet::new();
    for line in &input_lines[0] {
        let (comp1, comp2) = line.split_once('-').expect("bad line");
        network_outs
            .entry(comp1)
            .and_modify(|targets| targets.push(comp2))
            .or_insert(Vec::from([comp2]));
        network_outs
            .entry(comp2)
            .and_modify(|targets| targets.push(comp1))
            .or_insert(Vec::from([comp1]));
    }

    network_outs.iter().for_each(|(computer, partners)| {
        (0..partners.len()).for_each(|i| {
            (i..partners.len()).for_each(|j| {
                if network_outs
                    .get(partners[i])
                    .expect("Ghost in the machines")
                    .contains(&partners[j])
                {
                    let mut trio = Vec::from([computer, partners[i], partners[j]]);
                    trio.sort();
                    triplets.insert(trio);
                }
            })
        })
    });

    let answer1 = triplets
        .iter()
        .filter(|trio| trio.iter().any(|comp| comp.starts_with('t')))
        .count();

    let mut ordered_largest = triplets.into_iter().collect::<Vec<Vec<&str>>>();
    while ordered_largest.len() > 1 {
        let mut even_bigger_groups = Vec::new();
        ordered_largest.sort();
        for (ix, group) in ordered_largest.iter().enumerate() {
            let mut comparison_ix = ix + 1;
            while comparison_ix < ordered_largest.len() {
                let comparison_group = &ordered_largest[comparison_ix];
                if group[0..group.len() - 1] == comparison_group[0..group.len() - 1] {
                    let candidate_new = comparison_group[group.len() - 1];
                    if group.iter().all(|computer| {
                        network_outs
                            .get(computer)
                            .expect("phantom")
                            .contains(&candidate_new)
                    }) {
                        let mut new_group = (*group).clone();
                        new_group.push(candidate_new);
                        even_bigger_groups.push(new_group);
                    }
                    comparison_ix += 1;
                } else {
                    break;
                }
            }
        }
        ordered_largest = even_bigger_groups;
    }

    assert_eq!(ordered_largest.len(), 1);

    let answer2 = ordered_largest[0].join(",");
    (format!("{}", answer1), answer2.to_string())
}

#[cfg(test)]
mod tests {
    use super::day23;
    use crate::utils::load_input;

    #[test]
    fn check_day23_case01() {
        full_test(
            "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn", // INPUT STRING
            "7",           // PART 1 RESULT
            "co,de,ka,ta", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day23(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
