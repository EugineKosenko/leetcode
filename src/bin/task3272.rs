use std::env;
use std::collections::BTreeSet;



fn count_good_integers(n: i32, k: i32) -> i64 {
    let n = n as u32;
    let k = k as u64;
    if n == 1 { return (1..=9).filter(|d| d % k == 0).count() as i64; }
    let n_is_odd = n % 2 == 1;
    let n2 = n / 2;
    let goods: BTreeSet<Vec<_>> = (10_usize.pow(n2-1)..10_usize.pow(n2))
        .flat_map(|x| {
            let l = x.to_string();
            let r: String = l.chars().rev().collect();
            if n_is_odd {
                ('0'..='9')
                    .filter_map(|d| {
                        let s = l.clone() + &d.to_string() + &r;
                        if s.parse::<u64>().unwrap() % k > 0 { None } else {
                            Some(s.chars().collect())
                        }
                    })
                    .collect()
            } else {
                let s = l.clone() + &r;
                if s.parse::<u64>().unwrap() % k == 0 { vec![s.chars().collect()] } else { vec![] }
            }
        })
        .map(|s: Vec<_>| { let mut s = s; s.sort(); s })
        .collect();
    goods.into_iter()
        .map(|s| {
            let mut cs = s.iter().skip(1)
                .fold((s[0], vec![1]), |(pv, cs), &v| {
                    let mut cs = cs;
                    if v == pv { *cs.last_mut().unwrap() += 1; } else { cs.push(1); }
                    (v, cs)
                }).1;
            let n = n as u64;
            let mut result = cs.iter().filter(|&&c| c > 1)
                .fold((1..=n).product::<u64>(), |result, &c| result / (1..=c).product::<u64>());
            if s[0] == '0' {
                cs[0] -= 1;
                result -= cs.into_iter().filter(|&c| c > 1)
                    .fold((1..=n-1).product::<u64>(), |result, c| result / (1..=c).product::<u64>());
            }
            result
        })
        .sum::<u64>() as i64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let k = args[2].parse().unwrap();
    println!("{}", count_good_integers(n, k));
}
