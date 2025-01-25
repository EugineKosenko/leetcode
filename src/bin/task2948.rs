use std::env;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::HashMap;



fn main() {
    let args: Vec<String> = env::args().collect();
    let nums: Vec<i32> = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    let limit: i32 = args[2].parse::<i32>().unwrap();
    println!("{:?} {}", nums, limit);
    let mut queue = BinaryHeap::from(nums.iter().copied().map(Reverse).collect::<Vec<_>>());
    println!("Queue: {:?}", queue);
    let Reverse(mut v1) = queue.pop().unwrap();
    let mut dsu = vec![BinaryHeap::from([Reverse(v1)])];
    println!("DSU: {:?}", dsu);
    let mut index = HashMap::from([(v1, dsu.len() - 1)]);
    println!("Index: {:?}", index);
    while let Some(Reverse(v2)) = queue.pop() {
        let l = dsu.len() - 1;
        if v1 + limit >= v2 {
            dsu[l].push(Reverse(v2));
        } else {
            dsu.push(BinaryHeap::from([Reverse(v2)]));
        }
        index.insert(v2, dsu.len() - 1);
        v1 = v2;
    }
    println!("DSU: {:?}", dsu);
    println!("Index: {:?}", index);
    let result: Vec<i32> = nums.into_iter()
        .map(|v| {
            let Reverse(v) = dsu[*index.get(&v).unwrap()].pop().unwrap();
            v
        })
        .collect();
    println!("{:?}", result);
}
