fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let mut files: Vec<Vec<i32>> = Vec::new();
    for line in file.lines() {
        let mut f: Vec<i32> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if i % 2 == 0 {
                for _j in 0..c.to_string().parse().unwrap() {
                    f.push((i / 2).to_string().parse().unwrap());
                }
            }
            else {
                for _j in 0..c.to_string().parse().unwrap() {
                    f.push(-1);
                }
            }
        }
        files.push(f.clone());
    }

    for f in files.iter_mut() {
        let mut i = f.len() - 1;
        let mut j = 1;
        loop {
            while f[i] == -1 {
                i -= 1;
            }
            while f[j] != -1 {
                j += 1;
            }
            if i <= j { break; }
            f[j] = f[i];
            f[i] = -1;
        }
    }

    let mut sum = 0;
    for f in files.into_iter() {
        for (i, c) in f.into_iter().enumerate() {
            if c == -1 { break; }
            sum += i * c.to_string().parse::<usize>().unwrap();
        }
    }
    println!("{sum}");
}
