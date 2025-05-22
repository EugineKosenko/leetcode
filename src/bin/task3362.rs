use std::{env, fs, io::{self, BufRead}};
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;



fn max_removal(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let m = queries.len();
    let mut qs: BTreeMap<_, BinaryHeap<_>> = queries.into_iter()
        .fold(BTreeMap::new(), |mut qs, q| {
            qs.entry(q[0] as usize).or_default().push(q[1] as usize); qs
        });
    nums.push(0);
    for i in (1..=n).rev() { nums[i] -= nums[i-1]; }
    let mut result = m as i32;
    let mut i = 0;
    loop {
        while i < n && nums[i] <= 0 { i += 1; nums[i] += nums[i-1]; }
        if i == n { return result; }
        let mut rs = loop {
            let Some((l, mut rs)) = qs.pop_first() else { return -1; };
            match l.cmp(&i) {
                Ordering::Greater => { return -1; },
                Ordering::Equal => { break rs; },
                Ordering::Less => { qs.entry(i).or_default().append(&mut rs); }
            }
        };
        while nums[i] > 0 {
            let Some(r) = rs.pop() else { return -1; };
            if r < i { return -1; } else {
                nums[i] -= 1; nums[r+1] += 1;
                result -= 1;
            }
        }
        qs.insert(i, rs);
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
