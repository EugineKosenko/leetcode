use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(max_diff(555), 888);
    }
    
    #[test]
    fn test_example2() {
        assert_eq!(max_diff(9), 8);
    }
    #[test]
    fn test_edge_cases() {
        assert_eq!(max_diff(1), 8);
        assert_eq!(max_diff(111), 888);
        assert_eq!(max_diff(1000), 8000);
    }
    #[test]
    fn test_large_numbers() {
        assert_eq!(max_diff(123456), 820000);
        assert_eq!(max_diff(999999), 888888);
    }
}

pub fn max_diff(num: i32) -> i32 {
    let s = num.to_string();
    let d = s.chars().next().unwrap();
    let (mn, mx) = ('0'..='9').map(|d1| ('0'..='9').filter_map(move |d2| {
        if d1 == d && d2 == '0' { None } else { Some((d1, d2)) }
    })).flatten()
        .map(|(d1, d2)|
             s.chars().map(|d| if d == d1 { d2 } else { d })
             .collect::<String>().parse::<i32>().unwrap())
        .fold((i32::MAX, 0), |(mn, mx), n| (mn.min(n), mx.max(n)));
    mx - mn
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse().unwrap();
    println!("{}", max_diff(num));
}
