use std::env;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

fn find(alice: usize, step: usize,
        forward: &HashMap<usize, HashSet<usize>>,
        amount: &Vec<i32>, bob_path: &HashMap<usize, usize>) -> i32 {
    let result = match bob_path.get(&alice) {
        None => amount[alice],
        Some(bob_step) => match bob_step.cmp(&step) {
            Ordering::Less => 0,
            Ordering::Equal => amount[alice]/2,
            Ordering::Greater => amount[alice]
        }
    };
    result + forward.get(&alice).unwrap().iter()
        .map(|&alice| find(alice, step + 1, forward, amount, bob_path))
        .max().unwrap_or(0)
}

fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
    let mut forward: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut back = HashMap::new();
    let mut pool: HashMap<_, HashSet<_>> = HashMap::new();
    for edge in edges {
        let n1 = edge[0] as usize;
        let n2 = edge[1] as usize;
        pool.entry(n1).or_default().insert(n2);
        pool.entry(n2).or_default().insert(n1);
    }
    let mut queue = vec![0];
    while let Some(node) = queue.pop() {
        if let Some(nexts) = pool.remove(&node) {
            for next in nexts.iter() {
                back.insert(*next, node);
                queue.push(*next);
                pool.get_mut(next).unwrap().remove(&node);
            }
            forward.insert(node, nexts);
        }
    }
    let mut bob = bob as usize;
    let mut bob_path = HashMap::from([(bob, 0)]);
    let mut step = 0;
    while bob != 0 {
        bob = *back.get(&bob).unwrap();
        step += 1;
        bob_path.insert(bob, step);
    }
    find(0, 0, &forward, &amount, &bob_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let edges = args[1]
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
    let bob = args[2].parse().unwrap();
    let amount = args[3]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", most_profitable_path(edges, bob, amount));
}
