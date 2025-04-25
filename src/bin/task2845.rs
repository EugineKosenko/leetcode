use std::env;
use std::collections::HashMap;

fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let m = modulo;

    nums.into_iter()
        .scan(0, |c, num| { *c += if num % m == k { 1 } else { 0 }; Some(*c)x })
        .scan(HashMap::from([(0, 1)]), |map, c| {
            let result: i64 = *map.entry((c + m - k) % m).or_default();
            *map.entry(c % m).or_default() += 1;
            Some(result)
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let modulo = args[2].parse().unwrap();
    let k = args[3].parse().unwrap();
    println!("{}", count_interesting_subarrays(nums, modulo, k));
}
