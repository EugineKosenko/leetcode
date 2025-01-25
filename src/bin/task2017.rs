use std::{env, fs, io::{self, BufRead}};



fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let grid = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .next().unwrap();
    let grid: Vec<Vec<i32>> = grid
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
    println!("{} {}", grid.len(), grid[0].len());
    let mut sum1: i64 = grid[0].iter().map(|&i| i as i64).sum::<i64>() - grid[0][0] as i64;
    let mut sum2: i64 = 0;
    let mut result = sum1;
    for i in 0..grid[0].len() - 1 {
        sum1 -= grid[0][i + 1] as i64;
        sum2 += grid[1][i] as i64;
        result = result.min(sum1.max(sum2));
    }
    println!("{}", result);
    //println!("{}", result);
}
