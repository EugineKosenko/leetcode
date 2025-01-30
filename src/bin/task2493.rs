use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;



fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].parse().unwrap();
    let edges: Vec<Vec<i32>> = args[2]
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
    let mut arcs: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in edges {
        arcs.entry(edge[0] as usize).or_default().push(edge[1] as usize);
        arcs.entry(edge[1] as usize).or_default().push(edge[0] as usize);
    }
    let mut units: Vec<HashSet<usize>> = Vec::new();
    let mut queue1: HashSet<usize> = (1..=n as usize).collect();
    while let Some(node) = queue1.iter().cloned().next() {
        let mut unit: HashSet<usize> = HashSet::new();
        let mut queue2: HashSet<usize> = HashSet::from([node]);
        while let Some(node) = queue2.iter().cloned().next() {
            queue2.remove(&node);
            if queue1.contains(&node) {
                queue1.remove(&node);
                unit.insert(node);
                for node in arcs.entry(node).or_default() {
                    queue2.insert(*node);
                }
            }
        }
        units.push(unit);
    }
    let is_bipartite = units.iter()
        .all(|unit| {
            let mut colors: HashMap<usize, bool> = HashMap::new();
            let node = unit.iter().next().unwrap();
            colors.insert(*node, true);
            let mut queue = HashSet::from([node]);
            loop {
                let Some(node) = queue.iter().next().cloned() else { break true; };
                queue.remove(&node);
                let color1 = *colors.get(node).unwrap();
                if !arcs.get(node).unwrap().iter()
                    .all(|node| {
                        match colors.get(node) {
                            None => { // вузол не пофарбовано
                                colors.insert(*node, !color1);
                                queue.insert(node);
                                true
                            },
                            Some(color2) => { // вузол пофарбовано
                                color1 ^ color2 // кольори вузлів мают бути протилежними
                            }
                        }
                    }) { break false; }
            }
        });
    let result = if is_bipartite {
        units.into_iter()
            .map(|unit| {
                unit.into_iter()
                    .map(|node| {
                        let mut dists: HashSet<usize> = HashSet::new();
                        let mut map: HashMap<usize, usize> = HashMap::new();
                        let mut queue = BTreeSet::from([(0, node)]);
                        let mut visited = HashSet::new();
                        while let Some(item @ (dist, node)) = queue.iter().cloned().next() {
                            map.insert(node, dist);
                            dists.insert(dist);
                            queue.remove(&item);
                            visited.insert(node);
                            for node in arcs.get(&node).unwrap() {
                                if !visited.contains(node) { queue.insert((dist + 1, *node)); }
                            }
                        }
                        dists.len()
                    })
                    .max().unwrap()
            })
            .sum::<usize>() as i32
    } else {
        -1
    };
    println!("Result: {}", result);
}
