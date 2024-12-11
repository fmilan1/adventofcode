fn main() {
    let file = include_str!("../input2.txt");
    let mut stones: Vec<u64> = file.trim().split(" ").into_iter().map(|s| s.parse().unwrap()).collect();
    for _j in 0..25 {
        let mut new_stones: Vec<u64> = Vec::new();
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 { new_stones.push(1); }
            else if stones[i].to_string().len() % 2 == 0 {
                let left = &stones[i].to_string()[..stones[i].to_string().len() / 2];
                let right = &stones[i].to_string()[stones[i].to_string().len() / 2..];
                new_stones.push(left.parse().unwrap());
                new_stones.push(right.parse().unwrap());
            }
            else { new_stones.push(stones[i] * 2024); }
            i += 1;
        }
        stones = new_stones.clone();
    }
    println!("{}", stones.len());
}
