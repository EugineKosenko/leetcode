use std::env;

struct UnionFind { parent: Vec<usize> }

impl UnionFind {
    fn new(size: usize) -> Self { UnionFind { parent: (0..size).collect() } }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x { self.parent[x] = self.find(self.parent[x]); }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px < py { self.parent[py] = px; } else { self.parent[px] = py; }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let s1 = "parker".to_string();
        let s2 = "morris".to_string();
        let base_str = "parser".to_string();
        let result = smallest_equivalent_string(s1, s2, base_str);
        assert_eq!(result, "makkek");
    }

    #[test]
    fn test_empty() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let base_str = "".to_string();
        let result = smallest_equivalent_string(s1, s2, base_str);
        assert_eq!(result, "");
    }

    #[test]
    fn test_complex() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let base_str = "hold".to_string();
        let result = smallest_equivalent_string(s1, s2, base_str);
        assert_eq!(result, "hdld");
    }
}

pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut uf = s1.chars().zip(s2.chars())
        .fold(UnionFind::new(26), |mut uf, (c1, c2)| {
            uf.union((c1 as u8 - b'a') as usize, (c2 as u8 - b'a') as usize); uf
        });
    base_str.chars().map(|c| (uf.find((c as u8 - b'a') as usize) as u8 + b'a') as char).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s1 = args[1].clone();
    let s2 = args[2].clone();
    let base_str = args[3].clone();
    println!("{}", smallest_equivalent_string(s1, s2, base_str));
}
