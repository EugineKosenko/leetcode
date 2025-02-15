use std::env;

fn is_parted(ds: &[usize], a: usize, n: usize) -> bool {
    if ds.is_empty() { return a == n; }
    ds.iter().enumerate()
        .scan(0, |la, (i, d)| {
            *la = 10 * *la + d;
            Some((&ds[i+1..], *la))
        })
        .any(|(ds, la)| is_parted(ds, a + la, n))
}

fn punishment_number(n: i32) -> i32 {
    (1..=n)
        .filter(|&i| {
            let ds: Vec<_> = (i*i).to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            is_parted(&ds, 0, i as usize)
        })
        .map(|i| i*i)
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    println!("{}", punishment_number(n));
}
