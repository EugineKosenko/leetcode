use std::env;

fn graph(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let n = edges.iter().map(|edge| edge[0].max(edge[1])).max().unwrap() as usize + 1;
    edges.into_iter()
        .fold(vec![Vec::new(); n], |mut graph, edge| {
            let (from, to) = (edge[0] as usize, edge[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
            graph
        })
}
fn degree(graph: &Vec<Vec<usize>>, root: usize, k: i32) -> i32 {
    let mut result = 1;
    let mut is_visited = vec![false; graph.len()];
    is_visited[root] = true;
    let mut queue = vec![(root, 0)];
    while let Some((node, d)) = queue.pop() {
        let nodes: Vec<_> = graph[node].iter().filter(|&&node| !is_visited[node]).collect();
        result += nodes.len();
        if d < k-1 {
            for &node in nodes.into_iter() { queue.push((node, d+1)); is_visited[node] = true; }
        } else {
            for &node in nodes.into_iter() { is_visited[node] = true; }
        }
    }
    result as i32
}

pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    if k == 0 {
        let n = edges1.into_iter().map(|edge| edge[0].max(edge[1])).max().unwrap() as usize + 1;
        return vec![1; n];
    }
    let d = if k == 1 { 1 } else {
        let g = graph(edges2);
        (0..g.len()).map(|n| degree(&g, n, k-1)).max().unwrap()
    };
    let g = graph(edges1);
    (0..g.len()).map(|n| degree(&g, n, k) + d).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let edges1: Vec<Vec<i32>> = args[1]
        .trim_matches(|c| c == '[' || c == ']')
        .split("],[")
        .map(|s| s.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect())
        .collect();
    let edges2: Vec<Vec<i32>> = args[2]
        .trim_matches(|c| c == '[' || c == ']')
        .split("],[")
        .map(|s| s.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect())
        .collect();
    let k: i32 = args[3].parse().unwrap();
    println!("{:?}", max_target_nodes(edges1, edges2, k));
}
