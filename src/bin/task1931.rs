use std::env;

fn is_valid(v: u16, m: i32) -> bool { (0..m).all(|i| (v >> (2*i)) & 0b11 != 0) }
fn is_proper(v: u16, m: i32) -> bool {
    (0..m).all(|i| {
        let v1 = v >> (2*i) & 0b11;
        let v2 = v >> (2*i+2) & 0b11;
        v1 != 0 && v1 != v2
    })
}
const MOD: u64 = 10u64.pow(9) + 7;

fn color_the_grid(m: i32, n: i32) -> i32 {
    let cols: Vec<_> = (0..2u16.pow(2*m as u32))
        .filter(|&v| is_proper(v, m))
        .collect();
    let l = cols.len();
    (0..n-1)
        .fold(vec![1; l], |dp, _| {
            (0..l).map(|i| {
                cols.iter().zip(dp.iter())
                    .filter_map(|(&c, &d)| if is_valid(c ^ cols[i], m) { Some(d) } else { None })
                    .reduce(|a, b| (a + b) % MOD).unwrap()
            }).collect()
        })
        .into_iter().reduce(|a, b| (a + b) % MOD).unwrap() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let m = args[1].parse().unwrap();
    let n = args[2].parse().unwrap();
    println!("{}", color_the_grid(m, n));
}
