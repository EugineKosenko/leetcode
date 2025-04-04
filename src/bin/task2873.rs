use std::env;

fn tails<T: Clone>(v: &[T]) -> impl Iterator<Item = &[T]> { (0..v.len()).map(move |i| &v[i..]) }

fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    tails(&nums).flat_map(|ns1| tails(&ns1).skip(1).flat_map(|ns2| tails(&ns2).skip(1).map(|ns3| {
        (ns1[0] - ns2[0]) as i64 * ns3[0] as i64
    }))).max().unwrap_or(0).max(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", maximum_triplet_value(nums));
}
