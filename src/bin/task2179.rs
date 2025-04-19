use std::env;

struct FenwickTree(Vec<i64>);

impl FenwickTree {
    fn new(n: usize) -> Self { FenwickTree(vec![0; n + 1]) }
    fn len(&self) -> usize { self.0.len() - 1 }
    fn inc(&mut self, index: usize) {
        let mut index = index as isize + 1;
        while index as usize <= self.len() {
            self.0[index as usize] += 1;
            index += index & -index;
        }
    }
    fn get(&self, index: usize) -> i64 {
        let mut index = index as isize + 1;
        let mut result = 0;
        while index > 0 {
            result += self.0[index as usize];
            index -= index & -index;
        }
        result
    }
}

fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {        
    let n = nums1.len();
    let idcs = nums1.into_iter().enumerate()
        .fold(vec![0; n], |mut idcs, (i, num)| {
            idcs[num as usize] = i;
            idcs
        });
    let nums: Vec<_> = nums2.into_iter().map(|num| idcs[num as usize]).collect();
    let lcs = nums.iter()
        .scan(FenwickTree::new(n), |fw, &num| {
            fw.inc(num+1);
            Some(fw.get(num))
        });
    let rcs = nums.iter().rev()
        .scan(FenwickTree::new(n), |fw, &num| {
            fw.inc(n-num);
            Some(fw.get(n-1-num))
        })
        .collect::<Vec<_>>().into_iter().rev();
    lcs.zip(rcs).map(|(lc, rc)| lc * rc).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums1 = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let nums2 = args[2]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", good_triplets(nums1, nums2));
}
