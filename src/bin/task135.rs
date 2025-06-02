use std::env;

pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut idcs: Vec<_> = (0..n).collect();
    idcs.sort_by_key(|&i| ratings[i]);
    idcs.into_iter()
        .scan(vec![1; n], |cnts, i| {
            if i > 0 && ratings[i-1] < ratings[i] && cnts[i-1] >= cnts[i] { cnts[i] = cnts[i-1] + 1; }
            if i < n-1 && ratings[i+1] < ratings[i] && cnts[i+1] >= cnts[i] { cnts[i] = cnts[i+1] + 1; }
            Some(cnts[i])
        }).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let ratings = args[1]
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", candy(ratings));
}
