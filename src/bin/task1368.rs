use std::env;
use std::collections::HashSet;
use std::collections::BTreeSet;



fn main() {
    let mut result = 0;
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
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = HashSet::from([(0, 0)]);
    let mut queue = BTreeSet::from([(0, (0, 0))]);
    while let Some((cost, (r, c))) = queue.pop_first() {
        if (r, c) == (rows - 1, cols - 1) { result = cost; break; }
        visited.insert((r, c));
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let next = (r as isize + dr, c as isize + dc);
            if next.0 < 0 || next.0 == rows as isize
                || next.1 < 0 || next.1 == cols as isize { continue; }
            let next = (next.0 as usize, next.1 as usize);
            if visited.contains(&next) { continue; }
            let cost = cost + match (dr, dc) {
                (-1,  0) => if grid[r][c] == 4 { 0 } else { 1 },
                ( 0,  1) => if grid[r][c] == 1 { 0 } else { 1 },
                ( 1,  0) => if grid[r][c] == 3 { 0 } else { 1 },
                ( 0, -1) => if grid[r][c] == 2 { 0 } else { 1 },
                d => panic!("Invalid delta ({:?})", d)
            };
            queue.insert((cost, next));
        }
    }
    println!("{}", result);
}
