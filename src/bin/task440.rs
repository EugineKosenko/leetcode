#![recursion_limit="1024"]

use std::env;

fn find(n: i32, m: i32, k: i32, i: &mut i32, d: usize) -> i32 {
    // println!("{}", d);
    if m > n { return 0; }
    *i += 1;
    if *i == k { return m; }
    assert!(*i < k);
    (0..=9).map(|j| find(n, 10*m+j, k, i, d+1)).find(|&m| m>0).unwrap_or(0)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_1() {
        assert_eq!(find_kth_number(13, 2), 10);
    }
    
    #[test]
    fn test_basic_2() {
        assert_eq!(find_kth_number(13, 5), 13);
    }
    #[test]
    fn test_first_number() {
        assert_eq!(find_kth_number(20, 1), 1);
    }
    
    #[test]
    fn test_last_number() {
        assert_eq!(find_kth_number(20, 20), 9);
    }
    #[test]
    fn test_middle_number() {
        assert_eq!(find_kth_number(100, 10), 17);
    }
    
    #[test]
    fn test_small_range() {
        assert_eq!(find_kth_number(2, 2), 2);
    }
    
    #[test]
    fn test_single_number() {
        assert_eq!(find_kth_number(1, 1), 1);
    }
    #[test]
    fn test_larger_k() {
        assert_eq!(find_kth_number(100, 90), 9);
    }
    #[test]
    fn test_large_numbers() {
        let test_cases = vec![
            //(626615575, 366297423),  // Перша пара
            (724201123, 419746578),  // Друга пара
            (212276511, 161357227),  // Третя пара
            (760772781, 749696464),  // Четверта пара
            (174314809, 101386962),  // П'ята пара
            (156187893, 4429469),    // Шоста пара
            (862411591, 494068084),  // Сьома пара
            (12901357, 201607)       // Восьма пара
        ];
    
        for (n, k) in test_cases {
            let result = find_kth_number(n, k);
            assert!(result > 0 && result <= n,
                "Помилка для n={}, k={}, результат={}", n, k, result);
        }
    }
}

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut i = 0;
    (1..=9).map(|m| find(n, m, k, &mut i, 1)).find(|&m| m>0).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let k = args[2].parse().unwrap();
    println!("{}", find_kth_number(n, k));
}
