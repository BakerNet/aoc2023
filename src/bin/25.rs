advent_of_code::solution!(25);

use std::collections::{HashMap, HashSet};

use petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

type Edge = (NodeIndex, NodeIndex);
type Component = (String, Vec<String>);

fn parse_component(s: &str) -> Component {
    let mut splits = s.split(": ");
    let from = splits.next().unwrap();
    let v = splits
        .next()
        .unwrap()
        .split_whitespace()
        .map(|st| st.to_string())
        .collect::<Vec<String>>();
    (from.to_string(), v)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let graph_input = input
        .lines()
        .map(parse_component)
        .collect::<Vec<Component>>();
    let graph_map: HashMap<&str, (NodeIndex, Vec<&str>)> =
        graph_input
            .iter()
            .fold(HashMap::new(), |mut acc, component| {
                acc.entry(&component.0)
                    .and_modify(|(_, v)| {
                        v.extend(component.1.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
                    })
                    .or_insert_with(|| {
                        (
                            graph.add_node(()),
                            component.1.iter().map(AsRef::as_ref).collect(),
                        )
                    });
                component.1.iter().for_each(|item| {
                    acc.entry(item)
                        .and_modify(|(_, v)| v.push(&component.0))
                        .or_insert_with(|| (graph.add_node(()), vec![&component.0]));
                });
                acc
            });
    let num_nodes = graph_map.len() as u64;
    let edges = edges_from_graph(&graph_map);
    graph.extend_with_edges(edges);

    let (min_cut, partition) = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1))
        .expect("Expect stoer wagner to work")
        .expect("Expect stoer wagner to be Some");
    assert_eq!(min_cut, 3);
    let partition_len = partition.len() as u64;
    Some(partition_len * (num_nodes - partition_len))
}

fn edges_from_graph(graph: &HashMap<&str, (NodeIndex, Vec<&str>)>) -> HashSet<Edge> {
    let mut ret = HashSet::new();

    graph.values().for_each(|(n, v)| {
        v.iter().for_each(|s2| {
            let n2 = graph.get(s2).unwrap().0;
            if n < &n2 {
                ret.insert((*n, n2));
            } else {
                ret.insert((n2, *n));
            }
        })
    });
    ret
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
