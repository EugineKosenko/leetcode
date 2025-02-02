use std::{env, fs, io::{self, BufRead}};
use std::collections::BinaryHeap;



fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let n = nums1.len();
    println!("{}", n);
    let k = k as usize;
    let mut pairs = (0..n)
        .map(|i| (nums1[i], nums2[i]))
        .collect::<Vec<_>>();
    pairs.sort_by_key(|pair| pair.1);
    println!("{}", pairs.len());
    (0..=n-k)
        .inspect(|i| if i % 1000 == 0 { println!("{}", i); })
        .map(|i| {
            let mut result = 0;
            let mut rest = pairs[i..].iter().map(|pair| pair.0).collect::<BinaryHeap<_>>();
            for _ in 0..k {
                result += rest.pop().unwrap() as i64;
            }
            result * pairs[i].1 as i64
        })
        .max().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let nums1 = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let nums2 = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let k = lines.next().unwrap().parse().unwrap();
    println!("{}", max_score(nums1, nums2, k));
}
