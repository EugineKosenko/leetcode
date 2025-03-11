use std::{env, fs, io::{self, BufRead}};
use std::collections::HashMap;



fn count_of_substrings(word: String, k: i32) -> i64 {
    let word = word.as_bytes();
    let mut vcounts: HashMap<u8, usize> = "aeiou".chars().map(|c| (c as u8, 0)).collect();
    let mut result = 0;
    if k == 0 {
        for word in word.split(|c| !"aeiou".as_bytes().contains(c)).filter(|s| !s.is_empty()) {
            let n = word.len();
            for vcount in vcounts.values_mut() { *vcount = 0; }
            let (mut i, mut j) = (0, 0);
            while j < n {
                while j < n && vcounts.values().any(|&c| c == 0) {
                    if let Some(c) = vcounts.get_mut(&word[j]) { *c += 1; }
                    j += 1;
                }
                while vcounts.values().all(|&c| c > 0) {
                    result += (1 + n - j) as i64;
                    i += 1;
                    if let Some(c) = vcounts.get_mut(&word[i-1]) { *c -= 1; }
                }
            }
        }
    } else {
        let n = word.len();
        let k = k as usize;
        let mut l = 0;
        let mut ccount = 0;
        let mut m = 0;
        loop {
            if let Some(vcount) = vcounts.get_mut(&word[m]) { *vcount += 1; } else { ccount += 1; }
            if ccount == k { break; }
            m += 1;
            if m == n { return 0; }
        }
        while m < n {
            let mut r = m + 1;
            while r < n && vcounts.contains_key(&word[r]) { r += 1; }
            while m < r {
                while vcounts.values().all(|&vcount| vcount > 0)  {
                    result += (r - m) as i64;
                    l += 1;
                    if let Some(vcount) = vcounts.get_mut(&word[l-1]) {
                        *vcount -= 1;
                    } else {
                        loop {
                            m += 1;
                            if m == r { break; } else { *vcounts.get_mut(&word[m]).unwrap() += 1; }
                        }
                        break;
                    }
                }
                if m < r {
                    m += 1;
                    if m == r {
                        loop {
                            l += 1;
                            if let Some(vcount) = vcounts.get_mut(&word[l-1]) { *vcount -= 1; } else { break; }
                        }
                    } else {
                        *vcounts.get_mut(&word[m]).unwrap() += 1;
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let word = lines.next().unwrap().unwrap();
    let k = lines.next().unwrap().unwrap().parse().unwrap();
    println!("{}", count_of_substrings(word, k));
}
