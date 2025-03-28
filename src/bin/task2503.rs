use std::{fs, env, io::{self, BufRead}};
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::BTreeSet;



fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let (m, n, k) = (grid.len(), grid[0].len(), queries.len());
    let mut result = vec![0; k];
    let mut queries: BinaryHeap<(Reverse<i32>, usize)> = queries.into_iter().enumerate()
        .map(|(i, q)| (Reverse(q), i))
        .collect();
    let mut visited = HashSet::new();
    let mut queue = BTreeSet::from([(grid[0][0], (0, 0))]);
    while let Some(&(pv, p@(r, c))) = queue.first() {
        let Some(&(Reverse(qv), i)) = queries.peek() else { break; };
        if pv < qv {
            visited.insert(p);
            queue.pop_first();
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let r = r as isize + dr; if r < 0 || r == m as isize { continue; }
                let c = c as isize + dc; if c < 0 || c == n as isize { continue; }
                let p@(r, c) = (r as usize, c as usize);
                if !visited.contains(&p) { queue.insert((grid[r][c], p)); }
            }
        } else {
            result[i] = visited.len() as i32;
            queries.pop();
        }
    }
    for (_, i) in queries { result[i] = visited.len() as i32; }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let grid = lines.next().unwrap()
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
             .trim_start_matches('[')
             .trim_end_matches(']')
             .split(",")
             .map(|item| item.parse().unwrap())
             .collect())
        .collect();
    let queries = lines.next().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", max_points(grid, queries));
}
