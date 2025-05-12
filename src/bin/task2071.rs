use std::{env, fs, io::{self, BufRead}};

fn find(tasks: &[i32], mut workers: Vec<(i32, bool)>, mut pills: i32, strength: i32) -> bool {
    let mut i = workers.len();
    for task in tasks.into_iter().rev() {
        while i > 0 && workers[i-1].1 { i -= 1; }
        if workers[i-1].0 >= *task { i -= 1; continue; }
        if pills == 0 { return false; }
        let j = match workers[..i].into_iter().position(|w| !w.1 && w.0 + strength >= *task) {
            None => { return false; },
            Some(j) => j
        };
        workers[j].1 = true;
        pills -= 1;
    }
    true
}

fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
    tasks.sort();
    workers.sort();
    let tn = tasks.len();
    let wn = workers.len();
    let mut l = 0;
    let mut h = tn.min(wn) + 1;
    while l+1 < h {
        let m = (l+h)/2;
        let workers: Vec<_> = workers[wn-m..].into_iter().map(|&w| (w, false)).collect();
        if find(&tasks[..m], workers, pills, strength) { l = m; } else { h = m; }
    }
    l as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let tasks = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();
    let workers = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();
    let pills = lines.next().unwrap().parse().unwrap();
    let strength = lines.next().unwrap().parse().unwrap();
    println!("{}", max_task_assign(tasks, workers, pills, strength));
}
