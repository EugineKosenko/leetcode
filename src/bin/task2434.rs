use std::env;
use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_1() {
        let s = "zza".to_string();
        let result = robot_with_string(s);
        assert_eq!(result, "azz");
    }

    #[test]
    fn test_basic_2() {
        let s = "bac".to_string();
        let result = robot_with_string(s);
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_basic_3() {
        let s = "bdda".to_string();
        let result = robot_with_string(s);
        assert_eq!(result, "addb");
    }

    #[test]
    fn test_complex() {
        let s = "bydizfve".to_string();
        let result = robot_with_string(s);
        assert_eq!(result, "bdevfziy");
    }

    fn test_from_file_impl(filename: &str) {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(filename).unwrap();
        let mut lines = BufReader::new(file).lines();
        
        let input = lines.next().unwrap().unwrap();
        let expected = lines.next().unwrap().unwrap();
        
        let result = robot_with_string(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_file_1() {
        test_from_file_impl("./task2434_1.txt");
    }

    #[test] 
    fn test_from_file_2() {
        test_from_file_impl("./task2434_2.txt");
    }
    
    #[test] 
    fn test_from_file_3() {
        test_from_file_impl("./task2434_3.txt");
    }
}

pub fn robot_with_string(s: String) -> String {
    let mut cnts: BTreeMap<_, usize> = s.chars()
        .fold(BTreeMap::new(), |mut cnts, c| { *cnts.entry(c).or_default() += 1; cnts });
    let mut result = String::new();
    let mut t = Vec::new();
    for c in s.chars() {
        t.push(c);
        *cnts.get_mut(&c).unwrap() -= 1;
        if *cnts.get_mut(&c).unwrap() == 0 { cnts.remove(&c); }
        while !t.is_empty() && t.last().unwrap() <= cnts.first_key_value().map(|k| k.0).unwrap_or(&'z') {
            result.push(t.pop().unwrap());
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].clone();
    println!("{}", robot_with_string(s));
}
