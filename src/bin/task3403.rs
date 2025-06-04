use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_string() {
        let word = "a".repeat(5000);
        let num_friends = 2500;
        let result = answer_string(word, num_friends);
        assert_eq!(result.len(), 2501);
        assert!(result.chars().all(|c| c == 'a'));
    }
    #[test]
    fn test_pair() {
        let word = "aann".to_string();
        let num_friends = 2;
        let result = answer_string(word, num_friends);
        assert_eq!(result.as_str(), "nn");
    }
    #[test]
    fn test_solo() {
        let word = "gh".to_string();
        let num_friends = 1;
        let result = answer_string(word, num_friends);
        assert_eq!(result.as_str(), "gh");
    }
}

pub fn answer_string(word: String, num_friends: i32) -> String {
    let n = word.len();
    let m = num_friends as usize;
    if m == 1 { return word; }
    (0..n).map(|i| &word[i..n.min(i+n-m+1)]).max().unwrap().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = args[1].clone();
    let num_friends = args[2].parse().unwrap();
    println!("{}", answer_string(word, num_friends));
}
