use std::env;

fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();

    nums.push(0);
    for i in (1..=n).rev() {
        nums[i] -= nums[i-1];
    }

    let mut result = 0;
    let mut i = 0;
    while i < n {
        while i < n && nums[i] <= 0 { i += 1; nums[i] += nums[i-1]; }
        if i == n { break; }

        if result == queries.len() { return -1; }
        
        let l = queries[result][0] as usize;
        let r = queries[result][1] as usize;
        let v = queries[result][2];
        nums[i.max(l)] -= v;
        nums[i.max(r+1)] += v;
        result += 1;
    }
    result as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let queries = args[2]
      .strip_prefix('[').unwrap()
      .strip_suffix(']').unwrap()
      .split("],[")
      .map(|row| row
           .trim_start_matches('[')
           .trim_end_matches(']')
           .split(',')
           .map(|item| item.parse().unwrap())
           .collect())
      .collect();
    println!("{}", min_zero_array(nums, queries));
}
