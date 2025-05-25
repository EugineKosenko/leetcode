use std::env;
use std::collections::HashMap;
#[cfg(test)]
use std::vec;

#[test]
fn test_longest_palindrome() {
    // Тест 1: базовий випадок з симетричним словом посередині
    assert_eq!(
        longest_palindrome(
            vec!["lc","cl","gg"]
                .into_iter()
                .map(String::from)
                .collect()
        ),
        6  // "lcggcl" - довжина 6 символів
    );

    // Тест 2: випадок з повторами та кількома парами
    assert_eq!(
        longest_palindrome(
            vec!["ab","ty","yt","lc","cl","ab"]
                .into_iter()
                .map(String::from)
                .collect()
        ),
        8  // можна скласти "ablcclba" або "abyttyba"
    );

    // Тест 3: всі слова симетричні
    assert_eq!(
        longest_palindrome(
            vec!["cc","ll","xx"]
                .into_iter()
                .map(String::from)
                .collect()
        ),
        6  // можна скласти "ccllxx" або будь-яку іншу комбінацію
    );
}

fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut cs: HashMap<_, i32> = words.into_iter()
        .fold(HashMap::new(), |mut cs, w| {
            let bs = w.as_bytes();
            *cs.entry((bs[0], bs[1])).or_default() += 1; cs
        });
    let mut result = 0;
    let mut has_center = false;
    while let Some(w) = cs.keys().next().cloned() {
        let ((b1, b2), c) = cs.remove_entry(&w).unwrap();
        if b1 == b2 {
            result += c;
            if c % 2 == 1 { result -= 1; has_center = true; } 
        } else {
            result += 2 * cs.remove(&(b2, b1)).unwrap_or_default().min(c);
        }
    }
    if has_center { result += 1; }
    2*result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let words = args[1]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split(',')
        .map(|s| s.trim_matches('"'))
        .map(String::from)
        .collect();
    println!("{}", longest_palindrome(words));
}
