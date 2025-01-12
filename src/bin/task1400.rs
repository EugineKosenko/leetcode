use std::env;
use std::collections::BTreeMap;

fn can_construct(s: String, k: i32) -> bool {
    let k = k as usize;
    if s.len() < k { return false; }
    if s.len() == k { return true; }
    let mut stats = BTreeMap::new();
    for ch in s.chars() {
        *stats.entry(ch).or_default() += 1;
    }
    k >= stats.values().filter(|&cnt| cnt % 2 == 1).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = args[1].to_string();
    let k = args[2].parse::<i32>().unwrap();
    println!("{}", can_construct(source, k));
}
