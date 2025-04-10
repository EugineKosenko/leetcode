use std::env;

fn count(x: i64, l: i64, s: i64, sl: u32) -> i64 {
    let mut t = 1;
    let mut result = if x % 10_i64.pow(sl) < s { 0 } else { 1 };
    let xp = (x / 10_i64.pow(sl)).to_string();
    let xp = xp.chars().map(|c| c.to_digit(10).unwrap() as i64);
    for x in xp.rev() { if x > l { t *= l+1; result = t; } else { result += x * t; t *= l+1 } }
    result
}

fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
    let sl = s.len() as u32;
    let s = s.parse().unwrap();
    let l = limit as i64;
    count(finish, l, s, sl) - count(start-1, l, s, sl)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let start = args[1].parse().unwrap();
    let finish = args[2].parse().unwrap();
    let limit = args[3].parse().unwrap();
    let s = args[4].to_string();
    println!("{}", number_of_powerful_int(start, finish, limit, s));
}
