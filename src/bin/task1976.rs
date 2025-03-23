use std::env;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn find(node: usize,
        roads: &HashMap<usize, Vec<(usize, usize)>>,
        times: &Vec<usize>,
        dp: &mut Vec<usize>) -> usize {
    if dp[node] > 0 { return dp[node]; }
    for (next, weight) in &roads[&node] {
        if times[*next] + weight == times[node] {
            dp[node] = (dp[node] + find(*next, roads, times, dp)) % (10usize.pow(9) + 7);
        }
    }
    dp[node]
}

fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    if roads.is_empty() { return 1; }
    let n = n as usize;
    let mut roads_: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for road in roads {
        let from = road[0] as usize;
        let to = road[1] as usize;
        let time = road[2] as usize;
        roads_.entry(from).or_default().push((to, time));
        roads_.entry(to).or_default().push((from, time));
    }
    let roads = roads_;

    let mut is_visited = vec![false; n];
    let mut times = vec![usize::MAX; n]; times[0] = 0;
    let mut queue = BinaryHeap::from([Reverse((0, 0))]);
    while let Some(Reverse((_, node))) = queue.pop() {
        if is_visited[node] { continue; }
        is_visited[node] = true;
        for (next, weight) in &roads[&node] {
            if times[*next] >= times[node] + weight {
                times[*next] = times[node] + weight;
                queue.push(Reverse((times[*next], *next)));
            }
        }
    }
    
    let mut dp = vec![0; n]; dp[0] = 1;
    find(n-1, &roads, &times, &mut dp) as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let roads = if args[2] == "[]" { vec![] } else {
        args[2]
            .strip_prefix('[').unwrap()
            .strip_suffix(']').unwrap()
            .split("],[")
            .map(|row| row
                 .trim_start_matches('[')
                 .trim_end_matches(']')
                 .split(',')
                 .map(|item| item.parse().unwrap())
                 .collect())
            .collect()
    };
    println!("{}", count_paths(n, roads));
}
