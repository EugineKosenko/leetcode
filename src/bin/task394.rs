use std::env;
use std::str::Chars;
use std::iter::Peekable;

fn find(cs: &mut Peekable<Chars<'_>>) -> String {
    let mut result = String::new();
    while let Some(c) = cs.next() {
        if c == ']' { break; }
        if c.is_ascii_digit() {
            let mut k = c.to_digit(10).unwrap();
            loop {
                let c = cs.next().unwrap();
                if c == '[' { break; }
                k = 10 * k + c.to_digit(10).unwrap();
            }
            let pattern = find(cs);
            for _ in 0..k { result += &pattern; }
        } else {
            result.push(c);
        }
    }
    result
}

fn decode_string(s: String) -> String {
    find(&mut s.chars().peekable())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].to_string();
    println!("{}", decode_string(s));
}
