use std::env;

fn tails<T: Clone>(v: &[T]) -> impl Iterator<Item = &[T]> { (0..v.len()).map(move |i| &v[i..]) }

fn maximum_triplet_value_short(nums: Vec<i32>) -> i64 {
    tails(&nums).flat_map(|ns1| tails(&ns1).skip(1).flat_map(|ns2| tails(&ns2).skip(1).filter_map(|ns3| {
        if ns1[0] - ns2[0] <= 0 { None } else { Some((ns1[0] - ns2[0]) as i64 * ns3[0] as i64) }
    }))).max().unwrap_or(0)
}

fn maximum_triplet_value_long(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut result = 0;
    for i in 0..n-2 { for j in i+1..n-1 { for k in j+1..n {
        if nums[i] - nums[j] > 0 { result = result.max((nums[i] - nums[j]) as i64 * nums[k] as i64); }
    }}}
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.parse().unwrap())
        .collect();
    println!("{:?}", maximum_triplet_value_short(nums));
}
