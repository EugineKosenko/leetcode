use std::env;
use std::cmp::Ordering;



pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 { return nums.len() as i32; }
    let mut result = 1;
    let mut dir = nums[0].cmp(&nums[1]);
    let mut run = if dir == Ordering::Equal { 0 } else { 1 };
    for i in 1..nums.len()-1 {
        run += 1;
        let ndir = nums[i].cmp(&nums[i+1]);
        if ndir == Ordering::Equal || ndir != dir {
            result = result.max(run);
            dir = ndir;
            run = if dir == Ordering::Equal { 0 } else { 1 };
        }
    }
    result.max(run + 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums: Vec<i32> = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", longest_monotonic_subarray(nums));
}
