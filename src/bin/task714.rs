use std::env;

fn find(prices: &[i32], fee: i32, profit: i32, is_stocked: bool) -> i32 {
    if prices.is_empty() { return profit; }
    find(&prices[1..], fee, profit, is_stocked)
        .max(if is_stocked {
            find(&prices[1..], fee, profit + prices[0], !is_stocked)
        } else {
            find(&prices[1..], fee, profit - prices[0] - fee, !is_stocked)
        })
}

fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    find(&prices, fee, 0, false)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let prices = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let fee = args[2].parse().unwrap();
    println!("{}", max_profit(prices, fee));
}
