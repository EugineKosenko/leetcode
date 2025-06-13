use std::{env, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![10, 1, 2, 7, 1, 3];
        let p = 2;
        assert_eq!(minimize_max(nums, p), 1);
    }
    
    #[test]
    fn test_example2() {
        let nums = vec![4, 2, 1, 2];
        let p = 1;
        assert_eq!(minimize_max(nums, p), 0);
    }
    #[test]
    fn test_edge_cases() {
        assert_eq!(minimize_max(vec![1], 0), 0);
        assert_eq!(minimize_max(vec![1, 2, 3], 0), 0);
    }
    use std::fs;
    
    fn read_test_data(filename: &str) -> (Vec<i32>, i32) {
        let content = fs::read_to_string(filename)
            .expect("Error reading file");
        let lines: Vec<&str> = content.lines().collect();
        
        let nums = lines[0]
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();      
        
        let p = lines[1].parse().unwrap();
        
        (nums, p)
    }
    
    #[test]
    fn test_large_dataset() {
        let (nums, p) = read_test_data("./task2616_1.txt");
        let _result = minimize_max(nums, p);
    }
}

pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    let n = nums.len();
    nums.sort();
    let mut l = 0;
    let mut h = nums[n-1] - nums[0];
    while l < h {
        let m = (l+h)/2;
        let mut c = 0;
        let mut i = 0;
        while i < n-1 && c < p {
            if nums[i+1] - nums[i] <= m { c += 1; i += 2; } else { i += 1; }
        }
        if c < p { l = m+1 } else { h = m }
    }
    h
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    
    let nums = lines[0]
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();      
    let p = lines[1].parse().unwrap();
    println!("{}", minimize_max(nums, p));
}
