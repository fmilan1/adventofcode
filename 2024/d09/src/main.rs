fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let mut files1: Vec<i64> = Vec::new();
    let mut files2: Vec<(i64, i64)> = Vec::new();
    for line in file.lines() {
        for (i, c) in line.chars().enumerate() {
            if i % 2 == 0 {
                let id = (i / 2).to_string().parse().unwrap(); 
                let length = c.to_string().parse().unwrap();
                for _j in 0..length {
                    files1.push(id);
                }
                files2.push((id, length));
            }
            else {
                let length = c.to_string().parse().unwrap();
                for _j in 0..c.to_string().parse().unwrap() {
                    files1.push(-1);
                }
                if length == 0 { continue; }
                files2.push((-1, length));
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
        sum += i * c as usize;
    }
    println!("{sum}");

    let mut i = files2.len() - 1;
    let mut j = 1;
    let mut moved_ids: Vec<i64> = Vec::new();
    loop {
        if i == 0 { break; }
        while files2[i].0 == -1 {
            i -= 1;
        }
        while files2[j].0 != -1 {
            j += 1;
        }
        if i <= j {
            i -= 1;
            j = 1;
            continue;
        }
        if moved_ids.contains(&files2[i].0) { i -= 1; j = 1; continue; }
        if files2[j].1 >= files2[i].1 {
            files2[j].1 -= files2[i].1;
            let tmp = files2[i];
            files2[i].0 = -1;
            if files2[j].1 <= 0 { files2.remove(j); i -= 1; }
            files2.insert(j, tmp);
            moved_ids.push(tmp.0);
            j = 1;
        }
        else {
            j += 1;
        }
    }

    let mut sum2 = 0;
    let mut place = 0;
    for tuple in files2.iter() {
        if tuple.0 == -1 { 
            place += tuple.1;
            continue; 
        }
        for _j in 0..tuple.1 {
            sum2 += tuple.0 * place;
            place += 1;
        }
    }
    println!("{sum2}");
}
