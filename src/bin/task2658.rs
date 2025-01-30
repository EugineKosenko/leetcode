use std::env;
use std::collections::HashMap;



fn main() {
    let args: Vec<String> = env::args().collect();
    let grid: Vec<Vec<i32>> = args[1]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
              .trim_start_matches('[')
              .trim_end_matches(']')
              .split(',')
              .map(|item| item.parse().unwrap())
              .collect())
        .collect();
    let (m, n) = (grid.len(), grid[0].len());
    let mut pool: HashMap<(usize, usize), usize> = grid.into_iter().enumerate()
        .flat_map(|(r, row)| {
            row.into_iter().enumerate()
                .filter_map(move |(c, d)| if d == 0 { None } else { Some(((r, c), d as usize)) })
        })
        .collect();
    let mut result = 0;
    while let Some(start) = pool.keys().cloned().next() {
        let mut count = 0;
        let mut queue = vec![start];
        count += *pool.get(&start).unwrap();
        pool.remove(&start);
        while let Some((r, c)) = queue.pop() {
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (r1, c1) = (r as isize + dr, c as isize + dc);
                if r1 >= 0 && r1 < m as isize && c1 >= 0 && c1 < n as isize {
                    let next = (r1 as usize, c1 as usize);
                    if pool.contains_key(&next) {
                        count += *pool.get(&next).unwrap();
                        pool.remove(&next);
                        queue.push(next);
                    }
                }
            }
        }
        result = result.max(count);
    }
    println!("Result: {}", result);
}
