use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;

#[derive(Debug)]
struct Bag(BTreeMap<char, usize>);

impl Bag {
    fn new(word: &String) -> Self {
        let mut result = BTreeMap::new();
        for c in word.chars() {
            *result.entry(c).or_default() += 1;
        }
        Bag(result)
    }

    fn is_subset(&self, other: &Self) -> bool {
        0 == self.0.iter()
            .filter(|&(ch, cnt)| cnt > other.0.get(&ch).unwrap_or(&0))
            .count()
    }

    fn expand(&mut self, other: Self) {
        for (ch, cnt) in other.0 {
            *self.0.get_mut(&ch).unwrap() = cnt.max(*self.0.entry(ch).or_default());
        }
    }
}

pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let bags1: Vec<(Bag, String)> = words1.into_iter().map(|word| (Bag::new(&word), word)).collect();
    let mut sup2 = Bag(BTreeMap::new());
    for word in words2 {
        sup2.expand(Bag::new(&word));
    }
    
    bags1.into_iter()
        .filter_map(|(bag1, word)| {
            if sup2.is_subset(&bag1) {
                Some(word)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let s1 = lines.next().unwrap();
    let w1: Vec<String> = s1.trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|word| word.trim_matches('"').to_string())
        .collect();
    let s2 = lines.next().unwrap();
    let w2: Vec<String> = s2.trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|word| word.trim_matches('"').to_string())
        .collect();
    println!("{}", word_subsets(w1, w2).len());
}
