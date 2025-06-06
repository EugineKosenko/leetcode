use std::env;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_1() {
        let s = "leet**cod*e".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "ltode");
    }

    #[test]
    fn test_basic_2() {
        let s = "erase*****".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_stars() {
        let s = "abcde".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "abcde");
    }

    #[test]
    fn test_only_stars() {
        let s = "****".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "");
    }

    #[test]
    fn test_alternating() {
        let s = "a*b*c*d*e*".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "");
    }

    #[test]
    fn test_consecutive_stars() {
        let s = "abc***de**f".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "f");
    }

    #[test]
    fn test_stars_at_start() {
        let s = "**abc".to_string();
        let result = clear_stars(s);
        assert_eq!(result, "abc");
    }

    // fn test_from_file_impl(filename: &str) {
    //     use std::fs::File;
    //     use std::io::{BufRead, BufReader};

    //     let file = File::open(filename).unwrap();
    //     let mut lines = BufReader::new(file).lines();
        
    //     let input = lines.next().unwrap().unwrap();
    //     let expected = lines.next().unwrap().unwrap();
        
    //     let result = clear_stars(input);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn test_from_file_1() {
    //     test_from_file_impl("./task3170_1.txt");
    // }

    // #[test]
    // fn test_from_file_2() {
    //     test_from_file_impl("./task3170_2.txt");
    // }

    // #[test]
    // fn test_from_file_3() {
    //     test_from_file_impl("./task3170_3.txt");
    // }
}

pub fn clear_stars(s: String) -> String {
    let mut queue = BinaryHeap::new();
    for (i, c) in s.bytes().enumerate() {
        if c == b'*' {
            queue.pop();
        } else {
            queue.push((Reverse(c), i));
        }
    }
    let mut result = vec![0; s.len()];
    for (Reverse(c), i) in queue { result[i] = c; }
    let result = result.into_iter().filter(|&c| c != 0).collect();
    String::from_utf8(result).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].clone();
    println!("{}", clear_stars(s));
}
