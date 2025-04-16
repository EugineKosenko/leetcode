use std::env;
use std::collections::HashMap;

fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut result = 0;
    let (mut i, mut j) = (0, 1);
    let mut cs: HashMap<i32, i32> = HashMap::from([(nums[0], 1)]);
    let mut pc = 0;
    while j < n {
        while j < n && pc < k {
            pc += *cs.entry(nums[j]).or_default();
            *cs.entry(nums[j]).or_default() += 1;
            j += 1;
        }
        if pc >= k { result += n - j + 1; }
        loop {
            *cs.entry(nums[i]).or_default() -= 1;
            pc -= *cs.entry(nums[i]).or_default();
            i += 1;
            if pc < k { break; } else { result += n - j + 1; }
        }
    }
    result as i64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let k = args[2].parse().unwrap();
    println!("{}", count_good(nums, k));
}
