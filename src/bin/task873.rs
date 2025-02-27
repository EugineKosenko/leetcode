use std::env;
use std::collections::HashSet;
use std::collections::HashMap;

fn find(v1: i32, v2: i32, values: &HashSet<i32>, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if let Some(result) = memo.get(&(v1, v2)) { return *result; }
    let result = 1 + if values.contains(&v2) { find(v2, v1 + v2, values, memo) } else { 0 };
    memo.insert((v1, v2), result);
    result
}

fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let values = arr.iter().copied().collect();
    let n = arr.len();
    let mut memo = HashMap::new();
    let mut result = 0;
    for i in 0..n-1 { for j in i+1..n {
        result = result.max(1 + find(arr[j], arr[i] + arr[j], &values, &mut memo));
    }}
    if result < 3 { 0 } else { result }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arr = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{}", len_longest_fib_subseq(arr));
}
