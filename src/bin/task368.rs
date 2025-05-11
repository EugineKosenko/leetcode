use std::env;

fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    nums.sort();
    let mut dp = vec![(0, None); n];
    for i in 0..n {
        dp[i] = (0..i)
            .filter(|&j| nums[i] % nums[j] == 0)
            .max_by_key(|&j| dp[j].0)
            .map(|j| (dp[j].0 + 1, Some(j))).unwrap_or((1, None));
    }
    let mut result = Vec::new();
    let mut i = (0..n).max_by_key(|&i| dp[i].0).unwrap();
    loop {
        result.push(nums[i]);
        let (_, Some(i_)) = dp[i] else { break; };
        i = i_;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", largest_divisible_subset(nums));
}
