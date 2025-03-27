use std::env;

fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut nums: Vec<_> = grid.into_iter().flatten().collect();

    if nums.iter().any(|&num| num % x != nums[0] % x) { return -1; }

    nums.sort();
    let m = nums[nums.len() / 2];
    nums.into_iter().map(|num| (num - m).abs() / x).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let grid = args[1]
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
    let x = args[2].parse().unwrap();
    println!("{}", min_operations(grid, x));
}
