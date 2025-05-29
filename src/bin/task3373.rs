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
fn degree(graph: &Vec<Vec<usize>>) -> ([i32; 2], Vec<isize>) {
    let mut degree = [0, 0];
    let mut colors = vec![-1; graph.len()];
    colors[0] = 0;
    let mut queue = vec![0];
    while let Some(node) = queue.pop() {
        degree[colors[node] as usize] += 1;
        for &next in &graph[node] { if colors[next] == -1 {
            colors[next] = 1 - colors[node];
            queue.push(next)
        }}
    }
    (degree, colors)
}

pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
    let (d, _) = degree(&graph(edges2));
    let d2 = d[0].max(d[1]);
    let g = graph(edges1);
    let (d, cs) = degree(&g);
    (0..g.len()).map(|n| d[cs[n] as usize] + d2).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let edges1 = args[1]
        .trim_matches(|c| c == '[' || c == ']')
        .split("],[")
        .map(|s| s.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect())
        .collect();
    let edges2 = args[2]
        .trim_matches(|c| c == '[' || c == ']')
        .split("],[")
        .map(|s| s.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect())
        .collect();
    println!("{:?}", max_target_nodes(edges1, edges2));
}
