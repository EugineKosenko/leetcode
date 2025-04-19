use std::env;

fn count(nums: &[i32], limit: i32) -> i64 {
    let mut result = 0;
    for j in (1..nums.len()).rev() {
        if let Some(i) = search(&nums[..j], nums[j], limit) {
            if i+1 == j {
                return (result + j * (j + 1) / 2) as i64;
            } else {
                result += i + 1;
            }
        }
    }
    result as i64
}
fn search(nums: &[i32], num: i32, limit: i32) -> Option<usize> {
    let (mut l, mut h) = (0, nums.len() - 1);
    while l+1 < h {
        let m = (l + h) / 2;
        if nums[m] + num <= limit { l = m; } else { h = m; }
    }
    if nums[h] + num <= limit { return Some(h); }
    if nums[l] + num <= limit { Some(l) } else { None }
}

fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort();
    count(&nums, upper) - count(&nums, lower - 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let lower = args[2].parse().unwrap();
    let upper = args[3].parse().unwrap();
    println!("{}", count_fair_pairs(nums, lower, upper));
}
