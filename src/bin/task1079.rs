use std::env;
use std::collections::HashMap;

fn find(stats: &mut Vec<usize>) -> i32 {
    (0..stats.len())
        .map(|i| {
            if stats[i] == 0 { return 0; }
            stats[i] -= 1;
            let result = find(stats) + 1;
            stats[i] += 1;
            result
        })
        .sum()
}

fn num_tile_possibilities(tiles: String) -> i32 {
    let mut stats = HashMap::new();
    for c in tiles.chars() { *stats.entry(c).or_default() += 1; }
    let mut stats = stats.into_values().collect();
    find(&mut stats)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let tiles = args[1].to_string();
    println!("{}", num_tile_possibilities(tiles));
}
