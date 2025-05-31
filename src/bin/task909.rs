use std::env;
use std::collections::BinaryHeap;
use std::cmp::Reverse;



pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let jumps = board.into_iter().rev().enumerate()
        .fold(Vec::new(), |jumps, (i, row)| {
            if i % 2 == 0 {
                row.into_iter().fold(jumps, |mut jumps, target| { jumps.push(target-1); jumps })
            } else {
                row.into_iter().rev().fold(jumps, |mut jumps, target| { jumps.push(target-1); jumps })
            }
        });
    let mut dists = vec![-1; jumps.len()];
    let mut queue = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(dist), cell)) = queue.pop() {
        for next in cell+1..=(cell+6).min(jumps.len()-1) {
            let next = if jumps[next] >= 0 { jumps[next] as usize } else { next };
            if dists[next] == -1 || dist + 1 < dists[next] {
                dists[next] = dist + 1;
                queue.push((Reverse(dist+1), next));
            }
        }
    }
    *dists.last().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let board = args[1]
        .trim_matches(|c| c == '[' || c == ']')
        .split("],[")
        .map(|s| s.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect())
        .collect();
    println!("{}", snakes_and_ladders(board));
}
