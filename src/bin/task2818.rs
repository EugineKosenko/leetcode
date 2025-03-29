use std::{fs, env, io::{self, BufRead}};
use std::cmp::Reverse;

const MOD: u64 = 10_u64.pow(9) + 7;
fn pow(x: u64, n: u64) -> u64 {
    let x = x as u64;
    if n == 0 { return 1; }
    let y = pow(x, n/2);
    let y = (y * y) % MOD;
    if n % 2 == 0 { y } else { (y * x) % MOD }
}

fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let scores = nums.iter()
        .map(|&num| {
            let mut num = num;
            let mut score = 0;
            for d in 2..(num as f64).sqrt().ceil() as i32 {
                if num % d == 0 {
                    score += 1;
                    while num % d == 0 { num /= d; }
                }                  
            }
            if num > 1 { score += 1; }
            score
        });
    let lefts = scores.clone().enumerate()
        .scan(Vec::new(), |idxs, score@(_, s)| {
            while let Some(&(_, ls)) = idxs.last() {
                if ls < s { idxs.pop(); } else { break; }
            }
            let result = if idxs.is_empty() { -1 } else { idxs.last().unwrap().0 as i32 };
            idxs.push(score);
            Some(result)
        });
    let rights: Vec<_> = scores.enumerate().rev()
        .scan(Vec::new(), |idxs, score@(_, s)| {
            while let Some(&(_, rs)) = idxs.last() {
                if rs <= s { idxs.pop(); } else { break; }
            }
            let result = if idxs.is_empty() { n } else { idxs.last().unwrap().0 } as i32;
            idxs.push(score);
            Some(result)
        })
        .collect();
    let rights = rights.into_iter().rev();
    let mut rcounts: Vec<_> = lefts.zip(rights).enumerate()
        .map(|(i, (l, r))| (i, (i as i32 - l) as u64 * (r - i as i32) as u64))
        .collect();
    rcounts.sort_by_key(|&(i, _)| (Reverse(nums[i]), i));
    let mut result = 1;
    let mut k = k as u64;
    for (i, rc) in rcounts {
        let rc = k.min(rc);
        result = (result * pow(nums[i] as u64, rc)) % MOD;
        k -= rc;
    }
    result as i32
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
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();
    let k = lines.next().unwrap().parse().unwrap();
    println!("{:?}", maximum_score(nums, k));
}
