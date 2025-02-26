use std::env;



fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut best1 = i32::MIN;
    let mut sum = 0;
    for n in &nums {
        let n = *n;
        sum = n.max(sum + n);
        best1 = best1.max(sum);
    }
    let mut best2 = i32::MAX;
    let mut sum = 0;
    for n in nums {
        sum = n.min(sum + n);
        best2 = best2.min(sum);
    }
    best2 = -best2;
    best1.max(best2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", max_absolute_sum(nums));
}
