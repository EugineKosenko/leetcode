use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;



fn close_strings(word1: String, word2: String) -> bool {
    let mut s1: HashMap<char, usize> = HashMap::new();
    for c in word1.chars() { *s1.entry(c).or_default() += 1; }
    let mut s2: HashMap<char, usize> = HashMap::new();
    for c in word2.chars() { *s2.entry(c).or_default() += 1; }
    let ks1: HashSet<_> = s1.keys().collect();
    let ks2: HashSet<_> = s2.keys().collect();
    if ks1 != ks2 { return false; }
    let mut s1: BinaryHeap<_> = s1.into_values().collect();
    let mut s2: BinaryHeap<_> = s2.into_values().collect();
    while !s1.is_empty() && !s2.is_empty() {
        if s1.pop().unwrap() != s2.pop().unwrap() { return false; }
    }
    s1.is_empty() && s2.is_empty()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let word1 = args[1].to_string();
    let word2 = args[2].to_string();
    println!("{}", close_strings(word1, word2));
}
