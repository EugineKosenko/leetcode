use std::env;

fn is_near(w1: &str, w2: &str) -> bool {
    if w1.len() == w2.len() {
        w1.chars().zip(w2.chars()).filter(|(c1, c2)| c1 != c2).count() == 1
    } else { false }
}

fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let n = words.len();
    let (js, dp) = (0..n)
        .fold((Vec::new(), Vec::new()), |(mut js, mut dp), i| {
            let (j, d) = dp[..i].into_iter().enumerate()
                .filter_map(|(j, &d)| {
                    if groups[i] != groups[j] && is_near(&words[j], &words[i]) {
                        Some((Some(j), d))
                    } else { None }
                })
                .max_by_key(|&(_, d)| d)
                .unwrap_or((None, -1));
            js.push(j); dp.push(d+1);
            (js, dp)
        });
    let mut result = Vec::new();
    let mut j = dp.into_iter().enumerate().max_by_key(|&(_, d)| d).map(|(j, _)| j);
    while let Some(j_) = j { result.push(words[j_].clone()); j = js[j_]; }
    result.reverse();
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let words = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.to_string())
        .collect();
    let groups = args[2]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", get_words_in_longest_subsequence(words, groups));
}
