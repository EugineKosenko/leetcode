use std::env;
use std::collections::BTreeSet;
use std::cmp::Ordering;



fn main() {
    let args: Vec<String> = env::args().collect();
    let num1 = args[1].parse::<usize>().unwrap();
    let num2 = args[2].parse::<usize>().unwrap();
    let mut n = num2;
    let mut cnt = 0;
    while n > 0 {
        if n % 2 == 1 { cnt += 1 }
        n /= 2;
    }
    println!("{}", cnt);
    let mut n = num1;
    let mut bits = BTreeSet::new();
    let mut i = 0;
    while n > 0 {
        if n % 2 == 1 { bits.insert(i); }
        i += 1;
        n /= 2;
    }
    println!("{:?}", bits);
    let result = match cnt.cmp(&bits.len()) {
        Ordering::Equal => num1,
        Ordering::Less => {
            while bits.len() > cnt {
                bits.pop_first();
            }
            bits.into_iter().map(|p| 1 << p).sum()
        },
        Ordering::Greater => {
            let mut i = 0;
            while bits.len() < cnt {
                if !bits.contains(&i) { bits.insert(i); }
                i += 1;
            }
            bits.into_iter().map(|p| 1 << p).sum()
        }
    };
    println!("{}", result);
}
