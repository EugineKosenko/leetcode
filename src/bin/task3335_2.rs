use std::env;
use std::ops::{Deref, DerefMut};

const MOD: u64 = 10u64.pow(9) + 7;

#[derive(Clone, Debug)]
struct Matrix(Vec<Vec<u64>>);

impl Matrix {
    fn new(n: usize) -> Self { Self(vec![vec![0; 26]; n]) }
    fn row(row: Vec<u64>) -> Self { Self(vec![row]) }
    fn pow(&self, n: i32) -> Self {
        if n == 1 { return self.clone(); }
        let y = self.pow(n/2);
        let y = mul(&y, &y);
        if n % 2 == 0 { y } else { mul(&y, self) }
    }
}
impl Deref for Matrix {
    type Target = Vec<Vec<u64>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
fn mul(lhs: &Matrix, rhs: &Matrix) -> Matrix {
    let m = lhs.len();
    let mut result = Matrix::new(m);
    for i in 0..m { for j in 0..26 { for k in 0..26 {
        result[i][j] = (result[i][j] + (lhs[i][k] * rhs[k][j]) % MOD) % MOD;
    } } }
    result
}

fn length_after_transformations(s: String, t: i32) -> i32 {
    let cs = Matrix::row(
        s.chars().fold(vec![0; 26], |mut cs, c| { cs[(c as u8 - b'a') as usize] += 1; cs }));
    let tr = (0..26).fold(Matrix::new(26), |mut tr, i| {
        if i == 25 { tr[i][0] = 1; tr[i][1] = 1; } else { tr[i][i+1] = 1; }
        tr });
    mul(&cs, &tr.pow(t))[0].iter().fold(0, |a, b| (a+*b) % MOD) as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = args[1].to_string();
    let t = args[2].parse().unwrap();
    println!("{}", length_after_transformations(s, t));
}
