use std::{env, fs, io::{self, BufRead}};
use std::collections::HashMap;
use std::collections::HashSet;



fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let _num_courses: i32 = lines.next().unwrap().parse().unwrap();
    let prerequisites = lines.next().unwrap();
    let prerequisites = prerequisites
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap();
    
    let prerequisites: Vec<Vec<i32>> = if prerequisites.is_empty() {
        vec![]
    } else {
        prerequisites
            .split("],[")
            .map(|row| row
                 .trim_start_matches('[')
                 .trim_end_matches(']')
                 .split(',')
                 .map(|item| item.parse().unwrap())
                 .collect())
            .collect()
    };
    let queries: Vec<Vec<i32>> = lines.next().unwrap()
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
    let mut deps: HashMap<usize, Vec<usize>> = HashMap::new();
    for dep in prerequisites.iter() { deps.entry(dep[0] as usize).or_default().push(dep[1] as usize); }
    let mut closures = HashMap::new();
    for from in deps.keys() {
        let mut closure: HashSet<usize> = HashSet::new();
        let mut queue = vec![*from];
        while let Some(to) = queue.pop() {
            closure.insert(to);
            for to in deps.get(&to).unwrap_or(&vec![]) { if !closure.contains(to) { queue.push(*to); } }
        }
        closures.insert(from, closure);
    }
    let result: Vec<bool> = queries.into_iter()
        .map(|query| match closures.get(&(query[0] as usize)) {
            None => false,
            Some(closure) => closure.contains(&(query[1] as usize))
        })
        .collect();
    println!("{:?}", result);
}
