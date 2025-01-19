use std::env;
use std::collections::BTreeSet;

fn can_be_valid(s: String, locked: String) -> bool {
    if s.len() %2 == 1 { return false; }
    let mut lefts = BTreeSet::new();
    let mut rights = BTreeSet::new();
    let mut unlockeds = BTreeSet::new();
    for (p, (c, l)) in s.chars()
        .zip(locked.chars())
        .enumerate() {
            match l {
                '1' => match c {
                    '(' => lefts.insert(p),
                    ')' => rights.insert(p),
                    c => panic!("Invalid char {}", c)
                },
                '0' => unlockeds.insert(p),
                l => panic!("Invalid flag {}", l)
            };
        }
    for right in rights {
        match lefts.range(..right).next_back().copied() {
            None => match unlockeds.range(..right).next_back().copied() {
                None => { return false; },
                Some(unlocked) => { unlockeds.remove(&unlocked); }
            },
            Some(left) => { lefts.remove(&left); }
        }
    }
    for left in lefts {
        match unlockeds.range(left..).next().copied() {
            None => { return false; },
            Some(unlocked) => { unlockeds.remove(&unlocked); }
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = args[1].to_string();
    let locked = args[2].to_string();
    println!("{}", can_be_valid(source, locked));
}
