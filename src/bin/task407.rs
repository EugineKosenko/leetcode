use std::{env, fs, io::{self, BufRead}};

#[derive(Debug, Clone)]
enum Cell {
    Undefined, Leak(usize), Keep(usize)
}
type Cells = Vec<Vec<Cell>>;
fn show_cells(cells: &Cells) {
    for row in cells {
        for cell in row {
            print!(
                "{} ", match cell {
                    Cell::Undefined => "U ".to_string(),
                    Cell::Keep(n) => format!("K{}", n),
                    Cell::Leak(n) => format!("L{}", n)
                });

        }
        println!();
    }
}

fn main() {
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
    println!("{} {}", rows, cols);
    let mut cells = vec![vec![Cell::Undefined; cols]; rows];
    for row in cells.iter_mut() {
        row[0] = Cell::Leak(0);
        row[cols - 1] = Cell::Leak(0);
    }
    
    for col in 0..cols {
        cells[0][col] = Cell::Leak(0);
        cells[rows - 1][col] = Cell::Leak(0);
    }
    //show_cells(&cells);
    let minl = *grid.iter().map(|row| row.iter().min().unwrap()).min().unwrap();
    let maxl = *grid.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    println!("{} {}", minl, maxl);
    for l in minl..=maxl {
        //if l % 1000 == 0 { println!("Level {}", l); }
        let mut queue = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] <= l {
                    if let Cell::Leak(_) = cells[r][c] {
                        queue.push((r, c));
                    }
                }
            }
        }
        //println!("{:?}", queue);
        while let Some((r, c)) = queue.pop() {
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (r1, c1) = (r as isize + dr, c as isize + dc);
                if r1 >= 0 && r1 < rows as isize
                    && c1 >= 0 && c1 < cols as isize
                    && grid[r1 as usize][c1 as usize] <= l {
                        let (r1, c1) = (r1 as usize, c1 as usize);
                        match cells[r1][c1] {
                            Cell::Undefined => {
                                cells[r1][c1] = Cell::Leak(0);
                                queue.push((r1, c1));
                            },
                            Cell::Keep(n) => {
                                cells[r1][c1] = Cell::Leak(n);
                                queue.push((r1, c1));
                            },
                            Cell::Leak(_) => { /* do nothing */ }
                        }       
                    }
            }
        }
        //show_cells(&cells);
        //println!();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] <= l {
                    match cells[r][c] {
                        Cell::Undefined => { cells[r][c] = Cell::Keep(1); },
                        Cell::Keep(n) => { cells[r][c] = Cell::Keep(n + 1); },
                        Cell::Leak(_) => { /* do nothing */ }
                    }
                }
            }
        }
        //show_cells(&cells);
    }
    let result = cells.into_iter()
        .map(|row| row.into_iter()
             .map(move |cell| match cell {
                 Cell::Undefined => panic!("Empty cell"),
                 Cell::Leak(n) => n,
                 Cell::Keep(n) => n
             })
             .sum::<usize>() as i32)
        .sum::<i32>();
    println!("{}", result);
}
