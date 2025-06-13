#![recursion_limit="1024"]

use std::env;

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
            (626615575, 366297423),  // Перша пара
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
    let mut n = n as u32;
    let mut k = k as u32;
    let ln = (n as f64).log10().floor() as u32 + 1;
    let mut s = (10u32.pow(ln) - 1) / 9;
    let mut m = n;
    let mut b = 1;
    let mut d = 0;
    let mut result = 1;
    k -= 1;
    while k > 0 {
        if d > 9 {
            result += k/s;
            k -= (k/s)*s;
            if k == 0 { return result as i32; }
            result *= 10;
            s /= 10;
            k -= 1;
            continue;
        } else {
            let p = s - s/10;
            d = (n / p) % 10;
        }
        if s*(b + (d.max(1) - 1)) >= k {
            d = 10;
            result += k/s;
            k -= (k/s)*s;
            if k == 0 { return result as i32; }
            result *= 10;
            s /= 10;
            k -= 1;
            continue;
        }               
        if d > b {
            result += d-b;
            k -= (d-b)*s;
        }
        m = m - (d.max(1) - 1)*s - (9-d)*(s/10);
        println!("m={}", m);
        if m > k { result *= 10; k -= 1; s /= 10; m -= 1; continue; }
        b = 0;
    }
    result as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    let k = args[2].parse().unwrap();
    println!("{}", find_kth_number(n, k));
}
