use std::{env, fs, io::{self, BufRead}};
use std::collections::BinaryHeap;
use std::cmp::Reverse;



fn max_removal(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut queries: BinaryHeap<_> = queries.into_iter()
        .map(|q| Reverse((q[0] as usize, q[1] as usize))).collect();
    nums.push(0);
    for i in (1..=n).rev() { nums[i] -= nums[i-1]; }
    let mut result = queries.len() as i32;
    let mut i = 0;
    loop {
        while i < n && nums[i] <= 0 { i += 1; nums[i] += nums[i-1]; }
        if i == n { return result; }
        let mut r = loop {
            let Some(Reverse((l, r))) = queries.pop() else { return -1; };
            if l > i { return -1; }
            if r < i { continue; }
            break r;
        };
        
        let mut rs = Vec::new();
        while let Some(&Reverse((l, r_))) = queries.peek() {
            if l > i { break; } else { queries.pop(); }
            if r_ < i { continue; }
            if r_ > r { rs.push(r); r = r_; } else { rs.push(r_); }
        }
        for r in rs { queries.push(Reverse((i, r))); }
        nums[i] -= 1; nums[r+1] += 1;
        result -= 1;
    }
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
    let queries = lines.next().unwrap()
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
    println!("{}", max_removal(nums, queries));
}
