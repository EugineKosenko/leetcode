use std::env;
use std::collections::HashMap;
use std::collections::HashSet;



pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let mut links: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        let weight = edge[2] as usize;
        links.entry(from).or_default().push((to, weight));
        links.entry(to).or_default().push((from, weight));
    }
    let mut weights: Vec<usize> = Vec::new();
    let mut cnodes: HashMap<usize, usize> = HashMap::new();
    let mut nodes: HashSet<usize> = (0..n as usize).collect();
    while let Some(node) = nodes.iter().next().copied() {
        let mut weight = usize::MAX;
        let mut queue = vec![node];
        while let Some(node) = queue.pop() {
            if !nodes.contains(&node) { continue; }
            nodes.remove(&node);
            cnodes.insert(node, weights.len());
            for (node, weight_) in links.entry(node).or_default() {
                queue.push(*node);
                weight &= *weight_;
            }
        }
        weights.push(weight);
    }
    query.into_iter()
        .map(|query| {
            let from = *cnodes.get(&(query[0] as usize)).unwrap();
            let to = *cnodes.get(&(query[1] as usize)).unwrap();
            if from == to { weights[from] as i32 } else { -1 }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let edges = args[2]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
             .trim_start_matches('[')
             .trim_end_matches(']')
             .split(',')
             .map(|item| item.parse().unwrap())
             .collect())
        .collect();
    let query = args[3]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
             .trim_start_matches('[')
             .trim_end_matches(']')
             .split(',')
             .map(|item| item.parse().unwrap())
             .collect())
        .collect();
    println!("{:?}", minimum_cost(n, edges, query));
}
