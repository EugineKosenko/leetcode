use std::env;

fn tails<T: Clone>(v: &[T]) -> impl Iterator<Item = &[T]> { (0..v.len()-1).map(move |i| &v[i..]) }

fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let nums = nums.into_iter()
        .fold(vec![0], |mut nums, num| {
            nums.push(nums.last().unwrap() + num as i64); nums
        });
    tails(&nums).into_iter()
        .map(|tail| {
            // Lexically best, but $O(n)$ in time
            // tail.into_iter().enumerate()
            //     .position(|(i, num)| i as i64 * (num - tail[0]) >= k)
            //     .unwrap_or(tail.len()) as i64 - 1

            let (mut l, mut h) = (0, tail.len());
            while l+1 < h {
                let m = (l+h)/2;
                if m as i64 * (tail[m] - tail[0]) < k { l = m; } else { h = m; }
            }
            (if l as i64 * (tail[l] - tail[0]) >= k { h } else { l }) as i64
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
    let k = args[2].parse().unwrap();
    println!("{}", count_subarrays(nums, k));
}
