use std::env;
use std::collections::BTreeSet;

fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    let mut xs: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut ys: BTreeSet<(usize, usize)> = BTreeSet::new();
    for r in rectangles {
        xs.insert((r[0] as usize, r[2] as usize));
        ys.insert((r[1] as usize, r[3] as usize));
    }

    let mut splits = 0;
    let mut e = xs.pop_first().unwrap().1;
    for (b_, e_) in xs {
        if b_ < e {
            e = e.max(e_);
        } else {
            splits += 1;
            if splits == 2 { return true; }
            e = e_;
        }
    }

    let mut splits = 0;
    let mut e = ys.pop_first().unwrap().1;
    for (b_, e_) in ys {
        if b_ < e {
            e = e.max(e_);
        } else {
            splits += 1;
            if splits == 2 { return true; }
            e = e_;
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let rectangles = args[2]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
             .trim_start_matches('[')
             .trim_end_matches(']')
             .split(",")
             .map(|item| item.parse().unwrap())
             .collect())
        .collect();
    println!("{}", check_valid_cuts(n, rectangles));
}
