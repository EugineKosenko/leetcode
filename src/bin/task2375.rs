use std::env;

fn find(result: &mut [usize], pattern: &[char], is_used: &mut [bool]) -> bool {
    if pattern.is_empty() { return true; }
    let mut range = match pattern[0] {
        'D' => 1..=result[0]-1,
        'I' => result[0]+1..=9,
        c => panic!("Invalid pattern step {}", c)
    };
    range
        .any(|d| {
            if is_used[d-1] { return false; }
            result[1] = d;
            is_used[d-1] = true;
            if find(&mut result[1..], &pattern[1..], is_used) { return true; }
            result[1] = 0;
            is_used[d-1] = false;
            false
        })
}

fn smallest_number(pattern: String) -> String {
    let n = pattern.len();
    let pattern: Vec<_> = pattern.chars().collect();
    let mut result = vec![0; n+1];
    let mut is_used = vec![false; 9];
    for d in 1..=9 {
        result[0] = d;
        is_used[d-1] = true;
        if find(&mut result, &pattern, &mut is_used) { break; }
        result[0] = 0;
        is_used[d-1] = false;
    }
    result.into_iter().map(|d| char::from_digit(d as u32, 10).unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = args[1].to_string();
    println!("{}", smallest_number(pattern));
}
