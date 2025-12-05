use std::{env::args, fs::read_to_string, usize};

fn main() {
    let args: Vec<String> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();
    let (fresh_ids, ids) = file.split_once("\n\n").unwrap();
    let mut counter = 0;
    for line in ids.lines() {
        let n = line.parse::<usize>().unwrap();
        for fresh_line in fresh_ids.lines() {
            let (from, to) = fresh_line.split_once("-").map(|a| (a.0.parse::<usize>().unwrap(), a.1.parse::<usize>().unwrap())).unwrap();
            if n >= from && n <= to {
                counter += 1;
                break;
            }
        }
    }
    println!("{counter}");
}
