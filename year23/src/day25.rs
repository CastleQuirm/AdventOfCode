// Potential improvements:
//

use rand::Rng;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn day25(input_lines: &[Vec<String>]) -> (String, String) {
    let mut node_set: HashMap<String, Node> = HashMap::new();
    let mut edge_set: HashMap<String, Edge> = HashMap::new();
    let mut reverse_list: Vec<(String, String)> = Vec::new();

    for line in &input_lines[0] {
        let (source, dest_list) = line.split_once(": ").unwrap();
        let source_node = node_set.entry(source.to_owned()).or_default();
        let dest_nodes = dest_list.split(' ').map(|s| s.to_string()).collect_vec();

        for dest in &dest_nodes {
            let edge_name = source.to_owned() + dest;
            source_node.edges.insert(edge_name.clone());
            reverse_list.push((dest.clone(), source.to_owned()));
            edge_set.insert(
                edge_name,
                Edge {
                    nodes: [source.to_owned(), dest.to_owned()],
                },
            );
        }
    }

    for (rev_dest, rev_source) in reverse_list {
        let dest_node = node_set.entry(rev_dest).or_default();
        dest_node.edges.insert(rev_source.clone());
    }

    // Attempt the Karger algorithm repeatedly until we find a resulting pair of collapsed nodes with
    // just three edges.
    let answer1;
    loop {
        if let Some(answer) = attempt_karger(&node_set, &edge_set) {
            answer1 = answer;
            break;
        }
    }

    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn attempt_karger(
    node_set: &HashMap<String, Node>,
    edge_set: &HashMap<String, Edge>,
) -> Option<usize> {
    let mut reducing_nodes = node_set.clone();
    let mut reducing_edges = edge_set.clone();

    while reducing_nodes.len() > 2 {
        // Pick a random edge.
        let cut_options = reducing_edges.keys().collect_vec();
        let mut rng = rand::thread_rng();
        let edge_index = rng.gen_range(0..cut_options.len());
        let collapsed_edge_name = cut_options[edge_index].clone();
        let collapsed_edge = reducing_edges.remove(&collapsed_edge_name).unwrap();

        // Need to collapse the nodes, combining their edges (and removing any just connecting that pair)
        // and updating other edges to point at the new combined node.
        let node1_name = collapsed_edge.nodes[0].clone();
        let node2_name = collapsed_edge.nodes[1].clone();

        // Combine the name for the new joint node - since all nodes are given a three letter code we'll
        // be able to use this to get our answer eventually.
        let combo_name = node1_name.clone() + &node2_name;
        reducing_edges.retain(|_, edge| {
            edge.nodes != [node1_name.clone(), node2_name.clone()]
                && edge.nodes != [node2_name.clone(), node1_name.clone()]
        });

        for (_, edge) in reducing_edges.iter_mut() {
            for i in 0..=1 {
                if edge.nodes[i] == node1_name || edge.nodes[i] == node2_name {
                    edge.nodes[i] = combo_name.clone();
                }
            }
        }

        let node1 = reducing_nodes
            .remove(&node1_name)
            .expect("Unrecognized node name");
        let node2 = reducing_nodes
            .remove(&node2_name)
            .expect("Unrecognized node name {}");
        let mut edges = node1
            .edges
            .union(&node2.edges)
            .cloned()
            .collect::<HashSet<_>>();
        edges.remove(&collapsed_edge_name);
        reducing_nodes.insert(combo_name, Node { edges });
    }
    if reducing_edges.len() == 3 {
        // This is the minimal set!
        println!("{:#?}", reducing_edges);
        // The set size is determined by the length of the node name
        Some(
            reducing_nodes
                .keys()
                .map(|name| name.len() / 3)
                .product::<usize>(),
        )
    } else {
        println!("Found a cut with {} edges left", reducing_edges.len());
        None
    }
}

#[derive(Default, Clone)]
struct Node {
    // name: String,
    edges: HashSet<String>,
}

#[derive(Debug, Clone)]
struct Edge {
    // name: String,
    nodes: [String; 2],
}

#[cfg(test)]
mod tests {
    use super::day25;
    use crate::utils::load_input;

    #[test]
    fn check_day25_case01() {
        full_test(
            "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr", // INPUT STRING
            "54", // PART 1 RESULT
            "0",  // PART 2 RESULT
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
