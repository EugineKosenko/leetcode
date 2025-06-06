use std::env;

fn find(n: i32, i: i32, result: &mut Vec<i32>) {
    if i > n { return; }
    result.push(i);
    for j in 0..=9 { find(n, 10*i+j, result); }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_1() {
        let n = 13;
        let result = lexical_order(n);
        assert_eq!(result, vec![1,10,11,12,13,2,3,4,5,6,7,8,9]);
    }

    #[test]
    fn test_basic_2() {
        let n = 2;
        let result = lexical_order(n);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn test_single() {
        let n = 1;
        let result = lexical_order(n);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_twenty() {
        let n = 20;
        let result = lexical_order(n);
        assert_eq!(result, vec![1,10,11,12,13,14,15,16,17,18,19,2,20,3,4,5,6,7,8,9]);
    }

    #[test]
    fn test_hundred() {
        let n = 100;
        let result = lexical_order(n);
        let expected: Vec<i32> = vec![
            1,10,100,11,12,13,14,15,16,17,18,19,2,20,21,22,23,24,25,26,27,28,29,
            3,30,31,32,33,34,35,36,37,38,39,4,40,41,42,43,44,45,46,47,48,49,
            5,50,51,52,53,54,55,56,57,58,59,6,60,61,62,63,64,65,66,67,68,69,
            7,70,71,72,73,74,75,76,77,78,79,8,80,81,82,83,84,85,86,87,88,89,
            9,90,91,92,93,94,95,96,97,98,99
        ];
        assert_eq!(result, expected);
    }

    // fn test_from_file_impl(filename: &str) {
    //     use std::fs::File;
    //     use std::io::{BufRead, BufReader};

    //     let file = File::open(filename).unwrap();
    //     let mut lines = BufReader::new(file).lines();
        
    //     let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    //     let expected: Vec<i32> = lines.next().unwrap().unwrap()
    //         .split(',')
    //         .map(|s| s.parse().unwrap())
    //         .collect();
        
    //     let result = lexical_order(n);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn test_from_file_1() {
    //     test_from_file_impl("./task386_1.txt");
    // }

    // #[test]
    // fn test_from_file_2() {
    //     test_from_file_impl("./task386_2.txt");
    // }
}

pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 1..=9 { find(n, i, &mut result); }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse().unwrap();
    println!("{:?}", lexical_order(n));
}
