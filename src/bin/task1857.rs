use std::{env, fs, io::{self, BufRead}};
use std::collections::HashSet;

struct Node { color: usize, nodes: Vec<usize> }

fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    if edges.is_empty() { return 1; }
    let nodes: Vec<_> = colors.chars()
        .map(|c| Node { color: (c as u8 - b'a') as usize, nodes: Vec::new() })
        .collect();
    let mut indegrees = vec![0; nodes.len()];
    let edges: HashSet<_> = edges.into_iter().collect();
    let nodes = edges.into_iter()
        .fold(nodes, |mut nodes, edge| {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            nodes[from].nodes.push(to);
            indegrees[to] += 1;
            nodes
        });
    let mut dp: Vec<Vec<_>> = nodes.iter()
        .map(|node| (0..26).map(|j| if node.color == j { 1 } else { 0 }).collect())
        .collect();
    let mut result = -1;
    let mut visited_count = 0;
    let mut queue: Vec<_> = (0..nodes.len())
        .filter_map(|i| if indegrees[i] == 0 { Some(i) } else { None })
        .collect();
    while let Some(i) = queue.pop() {
        visited_count += 1;
        for &j in &nodes[i].nodes {
            for c in 0..26 {
                dp[j][c] = dp[j][c].max(dp[i][c] + if nodes[j].color == c { 1 } else { 0 });
                result = result.max(dp[j][c]);
            }
            indegrees[j] -= 1;
            if indegrees[j] == 0 { queue.push(j); }
        }
    }
    if visited_count < nodes.len() { return -1; }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let colors = lines.next().unwrap().trim_matches('"').to_string();
    let edges = lines.next().unwrap()
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("], [")
        .map(|s| s.trim_matches('[').trim_matches(']')
            .split(", ")
            .map(|n| n.trim().parse().unwrap())
            .collect())
        .collect();
    println!("{}", largest_path_value(colors, edges));
}
