use std::{env, fs, io::{self, BufRead}};
use std::collections::BinaryHeap;
use std::cmp::Reverse;



fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let n = nums1.len();
    let k = k as usize;
    let mut pairs = (0..n)
        .map(|i| (nums1[i], nums2[i]))
        .collect::<Vec<_>>();
    pairs.sort_by_key(|pair| pair.1);
    let (nums1, nums2): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    let mut queue: BinaryHeap<_> = nums1[n-k..].iter().map(|&i| Reverse(i as i64)).collect();
    let mut sum: i64 = nums1[n-k..].iter().map(|&i| i as i64).sum();
    let mut result = nums2[n-k] as i64 * sum;
    for i in (0..n-k).rev() {
        sum += nums1[i] as i64 - queue.pop().unwrap().0;
        queue.push(Reverse(nums1[i] as i64));
        result = result.max(nums2[i] as i64 * sum);
    }
    result
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
