use std::collections::BTreeMap;
const N: usize = 75;

fn main() {
    let file = include_str!("../input2.txt");

    let mut stones: BTreeMap<usize, usize> = BTreeMap::new();
    for n in file
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
    {
        stones.insert(n, 1);
    }
    for _i in 0..N {
        let mut new_stones: BTreeMap<usize, usize> = BTreeMap::new();
        for (&k, v) in stones.iter() {
            if k == 0 {
                *new_stones.entry(1).or_default() += v;
            } else if k.to_string().len() % 2 == 0 {
                let left = &k.to_string()[..k.to_string().len() / 2].parse().unwrap();
                let right = &k.to_string()[k.to_string().len() / 2..].parse().unwrap();
                *new_stones.entry(*left).or_default() += v;
                *new_stones.entry(*right).or_default() += v;
            } else {
                let new_k = k * 2024;
                *new_stones.entry(new_k).or_default() += v;
            }
        }
        stones = new_stones;
    }

    let count: usize = stones.values().into_iter().sum();
    println!("{count}");
}
