fn main() {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let file = include_str!("../input2.txt");
    for block in file.split("\n\n") {
        let mut heights = vec![-1; 5];
        let mut lock = false;
        for (i, line) in block.lines().enumerate() {
            if i == 0 && line == "#####" { lock = true; }
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[j] += 1;
                }
            }
        }
        if lock { locks.push(heights); }
        else { keys.push(heights); }
    }
    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut good = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    good = false;
                    break;
                }
            }
            if good { count += 1; }
        }
    }
    println!("{count}");
}
