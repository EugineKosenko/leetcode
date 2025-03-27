use std::{fs, env, io::{self, BufRead}};

fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let cars = cars as i64;

    let mut l = 0;
    let mut h = (*ranks.iter().min().unwrap() as i64) * cars * cars;
    while l < h {
        let m = (l + h) / 2;

        let mut cc = 0;
        for r in &ranks {
            cc += ((m / *r as i64) as f64).sqrt().floor() as i64;
            if cc >= cars { break; }
        }
        
        if cc < cars { l = m + 1; } else { h = m; }
    }
    l
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let ranks = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let cars = lines.next().unwrap().parse().unwrap();
    println!("{}", repair_cars(ranks, cars));
}
