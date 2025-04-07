use std::{env, fs, io::{self, BufRead}};



fn can_partition(nums: Vec<i32>) -> bool {
    let t = nums.iter().sum::<i32>() as usize;
    if t % 2 == 1 { return false; }
    let t = t / 2;
    let mut dp = vec![false; t+1];
    dp[0] = true;
    for num in nums {
        let num = num as usize;
        for i in (num..=t).rev() {
            dp[i] |= dp[i-num];
        }
    }
    dp[t]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let nums = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", can_partition(nums));
}
