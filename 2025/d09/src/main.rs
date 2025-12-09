use std::{env::args, fs::read_to_string};

fn main() {
    let file = read_to_string(args().collect::<Vec<String>>()[1].clone()).unwrap();
    let mut reds: Vec<(isize, isize)> = Vec::new();
    for line in file.lines() {
        let (x, y) = line.split_once(",").map(|s| (s.0.parse::<isize>().unwrap(), s.1.parse::<isize>().unwrap())).unwrap();
        reds.push((x, y));
    }

    let mut max_distance = 0;
    let mut a = (0, 0);
    let mut b = (0, 0);
    for i in 0..reds.len() {
        for j in i + 1..reds.len() {
            let new_max = (reds[i].0 - reds[j].0).pow(2) + (reds[i].1 - reds[j].1).pow(2);
            if new_max > max_distance {
                max_distance = new_max;
                a = reds[i];
                b = reds[j];
            }
        }
    }
    let dist_x = if a.0 > b.0 { a.0 - b.0 + 1} else { b.0 - a.0 + 1 };
    let dist_y = if a.1 > b.1 { a.1 - b.1 + 1} else { b.1 - a.1 + 1 };
    println!("{}", dist_x * dist_y);
}
