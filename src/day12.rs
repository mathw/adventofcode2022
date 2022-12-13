use std::{collections::HashMap, error::Error};

use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph};

use crate::{
    common::grid::Grid,
    day::{Day, DayResult, PartResult},
};

pub struct Day12 {
    input: &'static str,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            input: include_str!("inputs/day12.txt"),
        }
    }
}

impl Day for Day12 {
    fn run(&mut self) -> crate::day::Result {
        let part1 = run_part1(self.input)?;
        let part2 = run_part2(self.input)?;
        Ok(DayResult::new(
            PartResult::Success(format!("{} steps to the highest point", part1)),
            PartResult::Success(format!(
                "{} steps on the shortest path from any zero elevation",
                part2
            )),
        ))
    }
}

type Point = (usize, usize);

fn parse_to_grid(input: &str) -> Result<(Grid<u8>, Point, Point), Box<dyn Error>> {
    let mut start_point = None;
    let mut target_point = None;
    let lines_of_chars = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut grid = Grid::new(lines_of_chars[0].len(), lines_of_chars.len());

    for (y, row) in lines_of_chars.into_iter().enumerate() {
        for (x, mut c) in row.into_iter().enumerate() {
            if c == 'S' {
                start_point = Some((x, y));
                c = 'a';
            }
            if c == 'E' {
                target_point = Some((x, y));
                c = 'z';
            }
            let height = char_to_height(c);
            grid.set(x, y, height)?;
        }
    }

    Ok((grid, start_point.unwrap(), target_point.unwrap()))
}

fn char_to_height(c: char) -> u8 {
    c as u8 - b'a'
}

type Terrain = Graph<Point, ()>;

fn grid_to_graph(grid: &Grid<u8>) -> (Terrain, HashMap<Point, NodeIndex>) {
    let mut graph = Graph::new();
    let mut node_indicies = HashMap::new();

    for (x, y) in grid.iter_coords() {
        let ni = graph.add_node((x, y));
        node_indicies.insert((x, y), ni);
    }

    for (x, y) in grid.iter_coords() {
        let start_height = grid.get(x, y).unwrap();
        let start_ni = node_indicies[&(x, y)];
        let surrounds = grid.surrounding(x, y).unwrap();
        for (sx, sy) in surrounds {
            let end_height = grid.get(sx, sy).unwrap();
            let end_ni = node_indicies[&(sx, sy)];
            if start_height >= end_height
            // can step down to any height
            {
                graph.add_edge(start_ni, end_ni, ());
            }
            if end_height > start_height && end_height - start_height == 1 {
                graph.add_edge(start_ni, end_ni, ());
            }
        }
    }

    (graph, node_indicies)
}

fn find_shortest_path_between(graph: &Terrain, start: NodeIndex, end: NodeIndex) -> Option<usize> {
    let r = dijkstra(graph, start, Some(end), |_| 1);
    r.get(&end).copied()
}

fn run_part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let (grid, start, end) = parse_to_grid(input)?;
    let (graph, node_indicies) = grid_to_graph(&grid);
    let start = node_indicies[&start];
    let end = node_indicies[&end];
    find_shortest_path_between(&graph, start, end).ok_or_else(|| "Unable to find path".into())
}

fn run_part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let (grid, start, end) = parse_to_grid(input)?;
    let (graph, node_indicies) = grid_to_graph(&grid);
    let zero_nodes = grid
        .iter_coords()
        .filter(|(x, y)| *grid.get(*x, *y).unwrap() == 0)
        .map(|p| node_indicies[&p]);

    let end = node_indicies[&end];
    let mut shortest_path = usize::MAX;
    for start in zero_nodes {
        if let Some(length) = find_shortest_path_between(&graph, start, end) {
            shortest_path = usize::min(length, shortest_path);
        }
    }
    if shortest_path == usize::MAX {
        Err("Unable to find any paths".into())
    } else {
        Ok(shortest_path)
    }
}

#[test]
fn test_part1_sample() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let result = run_part1(input).expect("A path should be found");
    assert_eq!(result, 31);
}

#[test]
fn test_part2_sample() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let result = run_part2(input).expect("A path should be found");
    assert_eq!(result, 29);
}

#[test]
fn test_make_graph() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let (grid, start, end) = parse_to_grid(input).unwrap();
    let (graph, _node_indicies) = grid_to_graph(&grid);
    assert_eq!(start, (0, 0), "start");
    assert_eq!(end, (5, 2), "end");
    assert_eq!(
        graph.node_count(),
        grid.width() * grid.height(),
        "graph node count"
    );
}
