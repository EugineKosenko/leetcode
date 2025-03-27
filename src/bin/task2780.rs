use std::env;
use std::collections::HashMap;

fn minimum_index(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut fs: HashMap::<i32, usize> = HashMap::new();
    for num in nums.iter() { *fs.entry(*num).or_default() += 1; }
    let (x, f) = fs.into_iter().max_by_key(|&(_, f)| f).unwrap();

    let mut f1 = 0;
    for i in 0..n-1 {
        if nums[i] == x { f1 += 1; }
        if 2 * f1 > (i+1) && 2 * (f - f1) > (n-1-i) { return i as i32; }
    }
    -1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", minimum_index(nums));
}
