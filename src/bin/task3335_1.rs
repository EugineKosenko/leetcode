use std::env;

const MOD: u64 = 10_u64.pow(9) + 7;

fn pow(x: u64, n: usize) -> u64 {
    if n == 0 { return 1; }
    let y = pow(x, n/2);
    let y = (y * y) % MOD;
    if n % 2 == 0 { y } else { (y * x) % MOD }
}

fn length_after_transformations(s: String, t: i32) -> i32 {
    s.chars()
        .fold(vec![0; 26], |mut cs, c| {
            cs[(c as u8 - b'a') as usize] += 1; cs
        })
        .into_iter().enumerate()
        .filter(|&(_, c)| c > 0)
        .map(|(i, c)| (c * pow(2, (t as usize + i) / 26)) % MOD)
        .reduce(|a, b| (a+b) % MOD).unwrap() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].to_string();
    let t = args[2].parse().unwrap();
    println!("{}", length_after_transformations(s, t));
}
