use std::env;



pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut right = nums.iter().skip(1).sum::<i32>();
    for i in 0..n-1 {
        if left == right { return i as i32; }
        left += nums[i];
        right -= nums[i+1];
    }
    if left == right { (n-1) as i32 } else { -1 }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums: Vec<i32> = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", pivot_index(nums));
}
