use std::env;
use std::collections::HashSet;

fn tail_len(eminions: &HashSet<usize>, minions: &Vec<HashSet<usize>>, queue: &mut HashSet<usize>) -> usize {
    if eminions.is_empty() { return 0; }
    1 + eminions.iter()
        .map(|&m| { queue.remove(&m); tail_len(&minions[m], minions, queue) })
        .max().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let favorite: Vec<i32> = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let n = favorite.len();
    let mut cyclic_size = 0;
    let mut chained_size = 0;
    let mut minions = vec![HashSet::new(); n];
    for (m, f) in favorite.iter().enumerate() {
        minions[*f as usize].insert(m);
    }
    let mut queue: HashSet<usize> = (0..n).collect();
      while !queue.is_empty() {
          let mut se = *queue.iter().next().unwrap();
          let mut fast = favorite[se] as usize;
          while fast != se {
              se = favorite[se] as usize;
              fast = favorite[favorite[fast] as usize] as usize;
          }
          let mut cycle_len = 0;
          let mut tails_len = 0;
          let mut pe = se;
          let mut e = favorite[pe] as usize;
          loop {
              queue.remove(&e);
              cycle_len += 1;
              let mut emignions = minions[e].clone();
              emignions.remove(&pe);
              tails_len += tail_len(&emignions, &minions, &mut queue);
              pe = e;
              e = favorite[pe] as usize;
              if pe == se { break; }
          }
          cyclic_size = cyclic_size.max(cycle_len);
          if cycle_len == 2 { chained_size += cycle_len + tails_len; }
    
    }
    let result = cyclic_size.max(chained_size);
    println!("Result: {}", result);
}
