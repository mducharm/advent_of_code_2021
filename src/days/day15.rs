use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap}
};

use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day15"));

    let answer_1 = find_lowest_total_risk_for_any_path(data).unwrap();

    dbg!(answer_1);

    Ok(())
}

fn parse_data(input: String) -> Graph {
    let grid = input
        .lines()
        .map(|s: &str| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();

    grid_to_graph(grid)
}

fn grid_to_graph(grid: Vec<Vec<u8>>) -> Graph {
    let mut nodes: HashMap<Point, usize> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            *nodes.entry(Point(x as isize, y as isize)).or_insert(0) = *value as usize;
        }
    }

    let mut adjacency_matrix: HashMap<(Point, Point), bool> = HashMap::new();

    for (node, _value) in nodes.iter() {
        adjacency_matrix.entry((*node, *node)).or_insert(false);

        let potential_neighbors = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let Point(x, y) = node;

        for (dx, dy) in potential_neighbors {
            let neighbor = Point(x - dx, y - dy);

            if nodes.contains_key(&neighbor) {
                // Add both directions, since undirected graph
                *adjacency_matrix.entry((*node, neighbor)).or_insert(false) = true;
                *adjacency_matrix.entry((neighbor, *node)).or_insert(false) = true;
            }
        }
    }

    Graph {
        nodes,
        adjacency_matrix,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(isize, isize);

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    point: Point,
    from: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    nodes: HashMap<Point, usize>,
    adjacency_matrix: HashMap<(Point, Point), bool>,
}

fn find_lowest_total_risk_for_any_path(graph: Graph) -> Option<usize> {
    let start = Point(0, 0);
    let start_risk = 0;

    let mut distance: HashMap<Point, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();


    for (point, _value) in graph.nodes.iter() {
        distance.entry(*point).or_insert(usize::MAX);
    }
    heap.push(State {
        cost: start_risk,
        point: start,
        from: start,
    });

    *distance.entry(start).or_insert(usize::MAX) = start_risk as usize;

    let end = *distance.clone().keys().max().unwrap();

    while let Some(State { cost, point, from: _ }) = heap.pop() {
        if point == end {
            return Some(cost);
        }

        if let Some(d) = distance.get(&point) {
            if cost > *d {
                continue;
            }
        }

        let neighbors = graph
            .adjacency_matrix
            .iter()
            .filter(|(pair, exists)| **exists && pair.0 == point)
            .collect_vec();


        for (edge, _exists) in neighbors {
            let (from, to) = edge;

            let to_cost = &graph.nodes.get(to).unwrap();

            let next = State {
                cost: cost + **to_cost,
                point: *to,
                from: *from
            };

            if let Some(d) = distance.get(&next.point) {
                if next.cost < *d {
                    heap.push(next);
                    *distance.entry(next.point).or_insert(usize::MAX) = next.cost;
                }
            }


        }
        
    }

    None
}

#[cfg(test)]
mod tests {

    use crate::days::day15::find_lowest_total_risk_for_any_path;

    use super::parse_data;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(find_lowest_total_risk_for_any_path(data).unwrap(), 40);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        todo!();

        Ok(())
    }
}
