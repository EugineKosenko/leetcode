use std::env;

fn find(qs: &[Vec<i32>], dp: &mut [i64]) -> i64 {
    match () {
        _ if qs.is_empty() => 0,
        _ if dp[0] >= 0 => dp[0],
        _ => {
            let q = qs[0][0] as i64;
            let c = 1 + qs[0][1] as usize;
            dp[0] = find(&qs[1..], &mut dp[1..])
                .max(q + if c >= qs.len() { 0 } else { find(&qs[c..], &mut dp[c..]) });
            dp[0]
        }
    }
}

fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut dp = vec![-1; questions.len()];
    find(&questions, &mut dp)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let questions = args[1]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
              .trim_start_matches('[')
              .trim_end_matches(']')
              .split(',')
              .map(|item| item.parse().unwrap())
              .collect())
        .collect();
    println!("{:?}", most_points(questions));
}
