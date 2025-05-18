use std::env;

const MOD: u64 = 10u64.pow(9) + 7;

fn count_balanced_permutations(num: String) -> i32 {
    let n = num.len();
    let (mut fact, mut inv, mut inv_fact) = (vec![1; n+1], vec![1; n+1], vec![1; n+1]);
    for i in 2..=n {
        fact[i] = fact[i-1] * i as u64 % MOD;
        inv[i] = MOD - MOD / i as u64 * inv[(MOD % i as u64) as usize] % MOD;
        inv_fact[i] = inv_fact[i-1] * inv[i] % MOD;
    }
    let ts: usize = num.chars().map(|c| c.to_digit(10).unwrap() as usize).sum();
    let ts = if ts % 2 == 1 { return 0; } else { ts / 2 };
    let tl = n / 2;
    let mut dp = vec![vec![0; tl+1]; ts+1];
    dp[0][0] = 1;
    let mut cs = vec![0; 10];
    for d in num.chars().map(|c| c.to_digit(10).unwrap() as usize) {
        cs[d] += 1;
        for s in (d..=ts).rev() { for l in (1..=tl).rev() { dp[s][l] = (dp[s][l] + dp[s-d][l-1]) % MOD; }}
    } 
    let result = dp[ts][tl] * fact[tl] % MOD * fact[n-tl] % MOD;
    cs.into_iter().fold(result, |result, c| result * inv_fact[c] % MOD) as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].to_string();
    println!("{}", count_balanced_permutations(num));
}
