use std::env;

fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let n = weights.len();
    let k = k as usize;

    let mut weights_ = Vec::new();
    for i in 0..n-1 { weights_.push(weights[i] as i64 + weights[i+1] as i64); }
    weights_.sort();
    let weights = weights_;

    (0..k-1).map(|i| weights[n-2-i] - weights[i]).sum()
}

fn main() {
        let args: Vec<String> = env::args().collect();
    let weights = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let k = args[2].parse().unwrap();
    println!("{:?}", put_marbles(weights, k));
}
