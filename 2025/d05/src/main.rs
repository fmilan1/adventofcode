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

    let mut counter2 = 0;
    let mut fresh_list: Vec<(usize, usize)> = Vec::new();
    for fresh_line in fresh_ids.lines() {
        let (from, to) = fresh_line.split_once("-").map(|a| (a.0.parse::<usize>().unwrap(), a.1.parse::<usize>().unwrap())).unwrap();
        fresh_list.push((from, to));
    }

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut end = false;
    while !end {
        ranges.clear();
        end = true;
        for (from, to) in fresh_list.iter() {
            if ranges.is_empty() {
                ranges.push((*from, *to));
            } else {
                let mut need_new_range = true;
                for (r_from, r_to) in ranges.iter_mut() {
                    if from >= r_from && from <= r_to {
                        if to > r_to {
                            *r_to = *to;
                        }
                        need_new_range = false;
                        end = false;
                        break;
                    }
                }
                if need_new_range {
                    ranges.push((*from, *to));
                }
            }
        }
        fresh_list = ranges.clone();
        fresh_list.sort_by(|a, b| a.cmp(b));
    }

    for (from, to) in ranges {
        counter2 += to - from + 1;
    }

    println!("{counter}");
    println!("{counter2}");
}
