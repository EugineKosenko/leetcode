use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max_by_key;

fn find(str1: &[u8], str2: &[u8],
        i: usize, j: usize,
        dp: &mut Vec<Vec<Option<Rc<RefCell<Vec<u8>>>>>>) -> Rc<RefCell<Vec<u8>>> {
    if let Some(result) = &dp[i][j] { return result.clone(); }
    dp[i][j] = Some(Rc::new(RefCell::new(
        if str1[i-1] == str2[j-1] {
            let mut result = find(str1, str2, i-1, j-1, dp).borrow().clone();
            result.push(str1[i-1]);
            result
        } else {
            max_by_key(find(str1, str2, i-1, j, dp), find(str1, str2, i, j-1, dp), |s| s.borrow().len())
                .borrow().clone()
        })));
    dp[i][j].as_ref().unwrap().clone()
}

fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let (n1, n2) = (str1.len(), str2.len());
    let (str1, str2) = (str1.as_bytes(), str2.as_bytes());
    let mut dp = vec![vec![None; n2+1]; n1+1];
    for i in 0..=n1 { dp[i][0] = Some(Rc::new(RefCell::new(Vec::new()))); }
    for j in 1..=n2 { dp[0][j] = Some(Rc::new(RefCell::new(Vec::new()))); }
    let sstr = find(&str1, &str2, n1, n2, &mut dp);
    let sstr = sstr.borrow();
    let n = sstr.len();
    let mut result = String::new();
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < n1 || j < n2 {
        if i < n1 && (k >= n || str1[i] != sstr[k]) { result.push(str1[i] as char); i += 1; }
        if j < n2 && (k >= n || str2[j] != sstr[k]) { result.push(str2[j] as char); j += 1; }
        if i < n1 && j < n2 && k < n
            && str1[i] == sstr[k] && str2[j] == sstr[k] { result.push(str1[i] as char); (i, j, k) = (i+1, j+1, k+1); }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let str1 = args[1].to_string();
    let str2 = args[2].to_string();
    println!("{}", shortest_common_supersequence(str1, str2));
}
