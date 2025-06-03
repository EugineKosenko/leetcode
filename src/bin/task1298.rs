use std::{env, fs, io::{self, BufRead}};
use std::collections::HashSet;



pub fn max_candies(
    status: Vec<i32>,
    candies: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
    initial_boxes: Vec<i32>
) -> i32 {
    let mut kset = HashSet::new();
    let (mut closed, mut opened) = initial_boxes.into_iter()
        .fold((Vec::new(), Vec::new()), |(mut closed, mut opened), bx| {
            if status[bx as usize] == 1 { opened.push(bx as usize) } else { closed.push(bx as usize) };
            (closed, opened)
        });
    let mut result = 0;
    while let Some(bx) = opened.pop() {
        result += candies[bx];
        for &k in &keys[bx] { kset.insert(k as usize); }
        for &b in &contained_boxes[bx] {
            if status[b as usize] == 1 { opened.push(b as usize); } else { closed.push(b as usize); }
        }
        closed = closed.into_iter()
            .filter_map(|b| if kset.contains(&b) { opened.push(b); None } else { Some(b) })
            .collect();
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    
    let status = lines.next().unwrap()
        .trim_matches(|c| c == '[' || c == ']')
        .split(", ")
        .map(|n| n.parse().unwrap())
        .collect();
    
    let candies = lines.next().unwrap()
        .trim_matches(|c| c == '[' || c == ']')
        .split(", ")
        .map(|n| n.parse().unwrap())
        .collect();
    
    let keys = lines.next().unwrap()
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("], [")
        .map(|list| {
            let list = list.trim_matches(|c| c == '[' || c == ']');
            if list.is_empty() { Vec::new() } else {
                list.split(", ")
                    .map(|item| item.parse().unwrap())
                    .collect()
            }
        })
        .collect();
    
    let contained_boxes = lines.next().unwrap()
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("], [")
        .map(|list| {
            let list = list.trim_matches(|c| c == '[' || c == ']');
            if list.is_empty() { Vec::new() } else {
                list.split(", ")
                    .map(|item| item.parse().unwrap())
                    .collect()
            }
        })
        .collect();
    
    let initial_boxes = lines.next().unwrap()
        .trim_matches(|c| c == '[' || c == ']')
        .split(", ")
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", max_candies(status, candies, keys, contained_boxes, initial_boxes));
}
