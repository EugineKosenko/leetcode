use std::env;

fn find(result: &mut [usize], is_used: &mut [bool]) -> bool {
    if result.is_empty() { return true; }
    if result[0] > 0 { return find(&mut result[1..], is_used); }
    (1..=is_used.len().min(result.len()-1).max(1)).rev()
        .any(|v| {
            if is_used[v-1] { return false; }
            if v > 1 && result[v] > 0 { return false; }
            result[0] = v;
            if v > 1 { result[v] = v; }
            is_used[v-1] = true;
            if find(&mut result[1..], is_used) { return true; }
            result[0] = 0;
            if v > 1 { result[v] = 0; }
            is_used[v-1] = false;
            false
        })
}

fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = vec![0; 2*n-1];
    let mut is_used = vec![false; n];
    find(&mut result, &mut is_used);
    result.into_iter().map(|v| v as i32).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    println!("{:?}", construct_distanced_sequence(n));
}
