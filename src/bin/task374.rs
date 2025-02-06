use std::env;
use std::cmp::Ordering;

fn guess(num: i32, pick: i32) -> i32 {
    match num.cmp(&pick) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1
    }
}

fn guessNumber(n: i32, pick: i32) -> i32 {
    let mut low = 0;
    let mut high = n;
    if guess(high, pick) == 0 { return high; }
    loop {
        let result = ((high as u32 + low as u32) / 2) as i32;
        match guess(result, pick) {
            0 => { return result; },
            -1 => { high = result; },
            1 => { low = result; }
            c => panic!("Invalid guess result {}", c)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let pick = args[2].parse().unwrap();
    println!("{} {}", n, pick);
    println!("{}", guessNumber(n, pick));
}
