use std::{env, fs, io::{self, BufRead}};



fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let grid: Vec<Vec<i32>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .next().unwrap()
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
    let mut is_kept = vec![vec![true; cols]; rows];
    let mut leaks = Vec::new();
    let minl = *grid.iter().map(|row| row.iter().min().unwrap()).min().unwrap();
    let maxl = *grid.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    for l in minl..=maxl {
        for r in 0..rows {
            if grid[r][0] == l { is_kept[r][0] = false; leaks.push((r, 0)); }
            if grid[r][cols - 1] == l { is_kept[r][cols - 1] = false; leaks.push((r, cols - 1)); }
        }
        
        for c in 1..(cols - 1) {
            if grid[0][c] == l { is_kept[0][c] = false; leaks.push((0, c)); }
            if grid[rows - 1][c] == l { is_kept[rows - 1][c] = false; leaks.push((rows - 1, c)); }
        }
        let mut queue = leaks;
        leaks = Vec::new();
        while let Some((r, c)) = queue.pop() {
            leaks.push((r, c));
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (r1, c1) = (r as isize + dr, c as isize + dc);
                if r1 >= 0 && r1 < rows as isize
                    && c1 >= 0 && c1 < cols as isize
                    && grid[r1 as usize][c1 as usize] <= l {
                        let (r1, c1) = (r1 as usize, c1 as usize);
                        if is_kept[r1][c1] {
                            is_kept[r1][c1] = false;
                            result += (l - grid[r1 as usize][c1 as usize]) as usize;
                            queue.push((r1, c1));
                        }   
                    }
            }
        }
    }
    println!("{}", result);
}
