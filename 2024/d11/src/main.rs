fn main() {
    let file = include_str!("../input2.txt");
    let mut stones: Vec<u128> = file.trim().split(" ").into_iter().map(|s| s.parse().unwrap()).collect();
    for _i in 0..25 {
        let mut new_stones: Vec<u128> = Vec::new();
        for stone in stones {
            if stone == 0 { new_stones.push(1); }
            else if stone.to_string().len() % 2 == 0 {
                let left = &stone.to_string()[..stone.to_string().len() / 2];
                let right = &stone.to_string()[stone.to_string().len() / 2..];
                new_stones.push(left.parse().unwrap());
                new_stones.push(right.parse().unwrap());
            }
            else { new_stones.push(stone * 2024); }
        }
        stones = new_stones.clone();
    }
    println!("{}", stones.len());
}
