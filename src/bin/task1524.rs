use std::{env, fs, io::{self, BufRead}};



fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut even = 0;
    let mut odd = 0;
    for v in arr {
        (even, odd) = if v % 2 == 0 { (even + 1, odd) } else { (odd, even + 1) };
        result = (result + odd) % (10i32.pow(9) + 7);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let arr = io::BufReader::new(file)
        .lines().next().unwrap().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", num_of_subarrays(arr));
}
