fn is_safe(line: &str) -> bool {
    let nums: Vec<&str> = line.split(" ").collect();
    let mut negatives = 0;
    let mut positives = 0;
    for i in 1..nums.len() {
        let dif: i32 = nums[i - 1].parse::<i32>().unwrap() - nums[i].parse::<i32>().unwrap();
        if dif >= 1 && dif <= 3 {
            positives += 1;
        } else if dif <= -1 && dif >= -3 {
            negatives += 1;
        }
        if i == nums.len() - 1 {
            if (positives == i && negatives == 0) || (negatives == i && positives == 0) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut safes1 = 0;
    let mut safes2 = 0;
    for line in file.lines() {
        if is_safe(line) {
            safes1 += 1;
        }
        else {
            let nums: Vec<&str> = line.split(" ").collect();
            for i in 0..nums.len() {
                let mut tmp = nums.clone();
                tmp.remove(i);
                if is_safe(tmp.join(" ").as_str()) {
                    safes2 += 1;
                    break;
                }
            }
        }
    }
    println!("{safes1}\n{}", safes1 + safes2);
}
