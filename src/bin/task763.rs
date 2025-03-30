use std::env;
use std::collections::HashMap;

fn partition_labels(s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let n = s.len();

    let mut last = HashMap::new();
    for i in 0..n { last.insert(s[i], i); }
    
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        let mut k = last[&s[j]] + 1;
        j += 1;
        while j < k { if last[&s[j]] < k { j += 1; } else { k = last[&s[j]] + 1; } }
        result.push((j - i) as i32);
        i = j;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].to_string();
    println!("{:?}", partition_labels(s));
}
