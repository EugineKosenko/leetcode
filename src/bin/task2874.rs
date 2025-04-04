use std::env;

fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let pm: Vec<i32> = nums.iter().scan(i32::MIN, |m, &n| { *m = n.max(*m); Some(*m) }).collect();
    let mut sm: Vec<i32> = nums.iter().rev().scan(i32::MIN, |m, &n| { *m = n.max(*m); Some(*m) }).collect();
    sm.reverse();
    (1..nums.len()-1).map(|i| (pm[i-1] - nums[i]) as i64 * sm[i+1] as i64).max().unwrap_or(0).max(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", maximum_triplet_value(nums));
}
