use std::env;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn find(result: &mut BinaryHeap<Reverse<String>>, n: usize, happy: &mut String) {
    if n == 0 { result.push(Reverse(happy.clone())); return; }
    for c in "abc".chars() {
        if c != happy.chars().last().unwrap() {
            happy.push(c); find(result, n-1, happy); happy.pop();
        }
    }
}

fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let mut result = BinaryHeap::new();
    let mut happy = String::new();
    for c in "abc".chars() {
        happy.push(c); find(&mut result, n-1, &mut happy); happy.pop();
    }
    for _ in 0..k-1 { if result.pop().is_none() { return String::default(); } }
    result.pop().unwrap_or_default().0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let k = args[2].parse().unwrap();
    println!("{}", get_happy_string(n, k));
}
