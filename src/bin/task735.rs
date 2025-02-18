use std::env;



fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for a in asteroids {
        if result.is_empty() || a > 0 { result.push(a); continue; }
        let a = -a;
        while !result.is_empty()
            && *result.last().unwrap() > 0
            && *result.last().unwrap() < a {
                result.pop();
            }
        if result.is_empty() || *result.last().unwrap() < 0 { result.push(-a); continue; }
        if *result.last().unwrap() == a { result.pop(); continue; }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let asteroids: Vec<i32> = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", asteroid_collision(asteroids));
}
