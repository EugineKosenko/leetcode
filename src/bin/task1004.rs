use std::env;



fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    let n = nums.len();
    for i in 0..n {
        let mut j = i;
        let mut c = 0;
        while j < n && c < k {
            if nums[j] == 0 { c += 1; }
            j += 1;
        }
        if c == k { while j < n && nums[j] == 1 { j += 1; } }
        if c == k { while j < n && nums[j] == 1 { j += 1; } }
        result = result.max(j - i);
        if j == n { break; }
    }
    result as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums: Vec<i32> = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let k = args[2].parse().unwrap();
    println!("{}", longest_ones(nums, k));
}
