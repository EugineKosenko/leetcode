use std::env;

fn min_operations(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut result = 0;
    for i in 0..n-2 {
        if nums[i] == 0 {
            nums[i] = 1;
            nums[i+1] = 1 - nums[i+1];
            nums[i+2] = 1 - nums[i+2];
            result += 1;
        }
    }
    if nums[n-2] == 0 || nums[n-1] == 0 { return -1; }
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
    println!("{}", min_operations(nums));
}
