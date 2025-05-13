use std::env;
use std::collections::VecDeque;

const MOD: u64 = 10_u64.pow(9) + 7;

fn length_after_transformations(s: String, t: i32) -> i32 {
    let cs = s.chars()
        .fold(VecDeque::from([0; 26]), |mut cs, c| {
            cs[(c as u8 - b'a') as usize] += 1; cs
        });
    (0..t).fold(cs, |mut cs, _| {
        let c = cs.pop_back().unwrap();
        cs[0] = (cs[0] + c) % MOD;
        cs.push_front(c);
        cs
    }).into_iter().reduce(|a, b| (a+b) % MOD).unwrap() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].to_string();
    let t = args[2].parse().unwrap();
    println!("{}", length_after_transformations(s, t));
}
