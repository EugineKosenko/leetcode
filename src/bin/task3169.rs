use std::{fs, env, io::{self, BufRead}};

fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort();
    let mut result = meetings[0][0] - 1;
    let mut e = meetings[0][1];
    for m in &meetings[1..] {
        if m[0] <= e + 1 {
            e = e.max(m[1]);
        } else {
            result += m[0] - e - 1;
            e = m[1];
        }
    }
    result + days - e
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let days = lines.next().unwrap().parse().unwrap();
    let meetings = lines.next().unwrap()
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("], [")
        .map(|row| row
             .trim_start_matches('[')
             .trim_end_matches(']')
             .split(", ")
             .map(|item| item.parse().unwrap())
             .collect())
        .collect();
    println!("{}", count_days(days, meetings));
}
