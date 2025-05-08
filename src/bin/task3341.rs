use std::env;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let n = move_time.len();
    let m = move_time[0].len();
    let mut visited = HashMap::from([((0, 0), 0)]);
    let mut queue = BinaryHeap::from([(Reverse(0), (0, 0))]);
    while let Some((Reverse(time), point)) = queue.pop() {
        if point.0 == n-1 && point.1 == m-1 { return time; }
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let next: (usize, usize) = (
                match (point.0 as isize + dr).try_into() {
                    Ok(r) if r < n => r,
                    _ => { continue; }
                },
                match (point.1 as isize + dc).try_into() {
                    Ok(c) if c < m => c,
                    _ => { continue; }
                });
            let time = time.max(move_time[next.0][next.1]) + 1;
            match visited.get(&next) {
                None => { visited.insert(next, time); queue.push((Reverse(time), next)); },
                Some(&t) => { if time < t {
                    visited.insert(next, time); queue.push((Reverse(time), next));
                }}
            }
        }
    }
    unreachable!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let move_time = args[1]
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
    println!("{}", min_time_to_reach(move_time));
}

