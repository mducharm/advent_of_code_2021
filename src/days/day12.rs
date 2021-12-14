use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::{
    algo::all_simple_paths,
    dot::Dot,
    graph::NodeIndex,
    visit::{self, Dfs, IntoNeighbors},
    Graph,
};

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, ""))?;

    let answer_1 = count_total_paths(data);
    // let answer_2 =

    dbg!(answer_1);
    // dbg!(answer_2);

    Ok(())
}

fn parse_data(
    input: String,
) -> anyhow::Result<(
    petgraph::Graph<String, (), petgraph::Undirected>,
    // Vec<String>,
    Vec<NodeIndex>,
)> {
    let paths = input
        .lines()
        .map(|s: &str| s.split('-'))
        .map(|mut s| -> (&str, &str) { (s.next().unwrap(), s.next().unwrap()) })
        .collect_vec();

    let mut unique_cave_names = HashSet::new();

    for p in &paths {
        unique_cave_names.insert(p.0);
        unique_cave_names.insert(p.1);
    }

    let cave_names: Vec<&str> = unique_cave_names.into_iter().collect_vec();

    let mut nodes = vec![];

    let mut graph = Graph::new_undirected();

    for name in &cave_names {
        nodes.push(graph.add_node(name.to_string()));
    }

    for n in &nodes {
        // println!("{:?}", n);
        println!("{:?}", graph[*n]);
    }

    for (a, b) in paths {
        let node_a = nodes.iter().find(|n| graph[**n] == a).unwrap();
        let node_b = nodes.iter().find(|n| graph[**n] == b).unwrap();

        graph.add_edge(*node_a, *node_b, ());
    }

    println!("{:#?}", Dot::new(&graph));

    Ok((
        graph, // cave_names.into_iter().map(|s| s.to_string()).collect_vec(),
        nodes,
    ))
}

// part 1
fn count_total_paths(
    graph_and_nodes: (
        petgraph::Graph<String, (), petgraph::Undirected>,
        Vec<NodeIndex>,
    ),
) -> u128 {
    let (mut graph, mut nodes) = graph_and_nodes;

    let (start, end) = (
        nodes.iter().find(|n| graph[**n] == "start").unwrap(),
        nodes.iter().find(|n| graph[**n] == "end").unwrap(),
    );

    let mut visited = HashMap::new();

    for node in &nodes {
        visited.insert(graph[*node].clone(), false);
    }

    let mut path_count: u128 = 0;

    count_paths(&graph, *start, *end, &mut visited, &mut path_count)

}

fn count_paths(
    graph: &Graph<String, (), petgraph::Undirected>,
    a: NodeIndex,
    b: NodeIndex,
    visited: &mut HashMap<String, bool>,
    path_count: &mut u128,
) -> u128 {

    let mut visited_entry = visited.entry(graph[a].clone()).or_insert(false);
    visited_entry = &mut true;

    if a == b {
        *path_count += 1;
    } else {
        // let mut i = 0;

        // let num_of_neighbors = graph.neighbors(a);
        // let num_of_neighbors = graph.neighbors(a).collect_vec().len();
        // let neighbors = graph.neighbors(a).collect_vec();
        let mut neighbors = graph.neighbors(a).detach();

        while let Some(neighbor) = neighbors.next_node(graph) {
            if let Some(false) = visited.get(&graph[neighbor]) {
                // count_paths(graph, neighbor, b, visited, path_count);
            }
        }

        // println!("{:?}", graph[a]);
        // println!("neighbors");
        // // println!("{:?}", neighbors);
        // for neighbor in neighbors {
        // println!("{:?}", graph[neighbor]);
        //     if let Some(false) = visited.get(&graph[neighbor]) {
        //         count_paths(graph, neighbor, b, visited, path_count);
        //     }
        // }
    }

    *path_count
}

#[cfg(test)]
mod tests {

    use crate::days::day12::count_total_paths;

    use super::parse_data;

    const INPUT: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT_2: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT))?;
        let data2 = parse_data(String::from(INPUT_2))?;

        assert_eq!(count_total_paths(data), 19);
        assert_eq!(count_total_paths(data2), 226);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        todo!();

        Ok(())
    }
}
