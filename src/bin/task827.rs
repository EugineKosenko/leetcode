use std::env;
use std::collections::HashSet;
use std::collections::HashMap;



fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut land = grid.iter().enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate()
                .filter_map(move |(c, &p)| if p == 1 { Some((r, c)) } else { None })
        })
        .collect::<HashSet<_>>();
    if land.len() == n * n { return (n * n) as i32; }
    let mut shelf: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut sizes: Vec<usize> = Vec::new();
    while let Some(point) = land.iter().cloned().next() {
        let mut island = HashSet::new();
        let mut queue = HashSet::from([point]);
        while let Some(point) = queue.iter().cloned().next() {
            queue.remove(&point);
            land.remove(&point);
            island.insert(point);
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (r, c) = (point.0 as isize + dr, point.1 as isize + dc);
                if r >= 0 && r < n as isize && c >= 0 && c < n as isize {
                    let point = (r as usize, c as usize);
                    if grid[point.0][point.1] == 0 {
                        shelf.entry(point).or_default().insert(sizes.len());
                    } else if !island.contains(&point) {
                        island.insert(point);
                        queue.insert(point);
                    }
                }
            }
        }
        sizes.push(island.len());
    }
    1 + shelf.values()
        .map(|islands| islands.into_iter().map(|&i| sizes[i]).sum())
        .max().unwrap_or(0) as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let grid = args[1]
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
    println!("{}", largest_island(grid));
}
