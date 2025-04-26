use std::env;
use std::collections::VecDeque;

fn count(nums: &[i32], min_k: i32, max_k: i32) -> i64 {
    let n = nums.len();
    let mut result = 0;
    let mut q = VecDeque::new();
    let (mut i, mut j) = (0, 0);
    while j < n {
        while j < n {
            if q.is_empty() {
                q.push_back(nums[j]);
            } else if *q.back().unwrap() <= nums[j] {
                q.push_back(nums[j]);
            } else if nums[j] <= *q.front().unwrap() {
                q.push_front(nums[j]);
            }

            j += 1;
            
            if min_k == *q.front().unwrap() && *q.back().unwrap() == max_k { break; }
        }

        while !q.is_empty() && min_k == *q.front().unwrap() && *q.back().unwrap() == max_k {
            result += n - j + 1;
            if *q.back().unwrap() == nums[i] {
                q.pop_back();
            } else if *q.front().unwrap() == nums[i] {
                q.pop_front();
            }
            i += 1;
        }
    }
    result as i64
}

fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let n = nums.len();
    let mut result = 0;
    let mut i = 0;
    loop {
        while i < n && (nums[i] < min_k || nums[i] > max_k) { i += 1; }
        if i == n { break; } else {
            let mut j = i + 1;
            while j < n && min_k <= nums[j] && nums[j] <= max_k { j += 1; }
            result += count(&nums[i..j], min_k, max_k);
            i = j;
        }
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
    let min_k = args[2].parse().unwrap();
    let max_k = args[3].parse().unwrap();
    println!("{}", count_subarrays(nums, min_k, max_k));
}
