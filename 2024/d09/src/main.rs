fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let mut files1: Vec<i32> = Vec::new();
    for line in file.lines() {
        for (i, c) in line.chars().enumerate() {
            if i % 2 == 0 {
                for _j in 0..c.to_string().parse().unwrap() {
                    files1.push((i / 2).to_string().parse().unwrap());
                }
            }
            else {
                for _j in 0..c.to_string().parse().unwrap() {
                    files1.push(-1);
                }
            }
        }
    }

    let mut i = files1.len() - 1;
    let mut j = 1;
    loop {
        while files1[i] == -1 {
            i -= 1;
        }
        while files1[j] != -1 {
            j += 1;
        }
        if i <= j { break; }
        files1[j] = files1[i];
        files1[i] = -1;
    }

    let mut sum = 0;
    for (i, c) in files1.into_iter().enumerate() {
        if c == -1 { break; }
        sum += i * c.to_string().parse::<usize>().unwrap();
    }
    println!("{sum}");
}
