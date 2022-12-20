use std::collections::HashMap;

use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;

fn create_graph(input: &str) -> DiGraph<char, usize> {
    let mut g: DiGraph<char, usize> = DiGraph::new();
    let mut rows = 0;
    let mut cols = 0;
    for (row, line) in input.split("\n").collect::<Vec<&str>>().iter().enumerate() {
        rows += 1;
        cols = 0;
        for (col, c) in line.chars().enumerate() {
            cols += 1;
            g.add_node(c);
        }
    }
    for row in 0..rows {
        for col in 0..cols {
            let ni: i32 = row * cols + col;
            let cur_node_index = NodeIndex::new(ni as usize);
            let mut cur_char = g.node_weight(cur_node_index).unwrap().to_owned();
            if cur_char == 'S' {
                cur_char = 'a';
            }

            let directions: Vec<i32> = vec![
                ni - cols, // up
                ni + cols, // down
                ni - 1, // left
                ni + 1  // right
            ];

            for direction in directions {
                if direction < 0
                || direction > (rows * cols) as i32 - 1 {
                    continue;
                } else {
                    let target_node_index = NodeIndex::new(direction as usize);
                    let mut target_char = g.node_weight(target_node_index).unwrap().to_owned();
                    if target_char == 'E' { target_char = 'z' }
                    if target_char == 'S' { target_char = 'a' }
                    if target_char as u32 <= cur_char as u32 + 1 {
                        g.add_edge(cur_node_index, target_node_index, 1);
                    }
                }
            }
        }
    }
    g
}

pub fn shortest_path(input: &str) -> usize {
    let g = create_graph(input);
    let start = get_start_nodeindex(input);
    let end = get_end_nodeindex(input);
    println!("Start: {:?}\tEnd: {:?}", start, end);
    let node_map = dijkstra(&g, start, Some(end), |_| 1);
    node_map.get(&end).unwrap().to_owned() as usize
}

pub fn get_hiking_trail_start_nodeindexes(input: &str) -> Vec<NodeIndex> {
    let mut v: Vec<NodeIndex> = vec![];
    let mut ni = 0;
    for line in input.split("\n").collect::<Vec<&str>>() {
        for c in line.chars() {
            ni += 1;
            if c == 'a'
            || c == 'S' {
                v.push(NodeIndex::new(ni - 1));
            }
        }
    }
    v
}

pub fn hiking_trail_shortest_path(input: &str) -> usize {
    let g = create_graph(input);
    let hiking_trail_start_nodeindexes = get_hiking_trail_start_nodeindexes(input);
    let end = get_end_nodeindex(input);
    let mut min_distance = 9999;
    for node in hiking_trail_start_nodeindexes {
        let node_map = dijkstra(&g, node, Some(end), |_| 1);
        let distance_to_end = node_map.get(&end).unwrap_or(&9999).to_owned() as usize;
        min_distance = if distance_to_end < min_distance {distance_to_end} else {min_distance}
    }
    min_distance
}

fn get_start_nodeindex(input: &str) -> NodeIndex {
    let mut ni: usize = 0;
    for line in input.split("\n").collect::<Vec<&str>>() {
        for c in line.chars() {
            ni += 1;
            if c == 'S' {
                println!("{}", ni);
                return NodeIndex::new(ni - 1)
            }
        }
    }
    panic!("Could not find start nodeindex!");
}

fn get_end_nodeindex(input: &str) -> NodeIndex {
    let mut ni: i32 = -1;
    for line in input.split("\n").collect::<Vec<&str>>() {
        for c in line.chars() {
            ni += 1;
            if c == 'E' {
                return NodeIndex::new(ni as usize)
            }
        }
    }
    panic!("Could not find end nodeindex!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
        let expected = 31;
        let actual = shortest_path(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_hiking_trail_shortest_path() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
        let expected = 29;
        let actual = hiking_trail_shortest_path(input);
        assert_eq!(actual, expected)
    }
}