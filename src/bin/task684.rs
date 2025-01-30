use std::env;
use std::collections::{HashSet, BTreeSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let edges: Vec<Vec<i32>> = args[1]
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

    let mut nodes = HashSet::from([edges[0][0] as usize]);
    let mut edges = edges.into_iter().enumerate()
        .map(|(i, edge)| (i, (edge[0] as usize, edge[1] as usize)))
        .collect::<BTreeSet<_>>();
    let result = loop {
        let &e @ (_, (f, t)) = edges.iter()
            .find(|&(_, (f, t))| nodes.contains(f) || nodes.contains(t))
            .unwrap();
        println!("Edge: {:?}", e);
        if nodes.contains(&f) && nodes.contains(&t) { break vec![f as i32, t as i32]; }
        edges.remove(&e);
        nodes.insert(f);
        nodes.insert(t);
    };
    println!("{:?}", result);
}
