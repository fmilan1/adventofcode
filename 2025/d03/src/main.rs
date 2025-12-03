use std::{env::args, fs::read_to_string, isize};

fn main() {
    let args: Vec<_> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();

    let mut sum = 0;
    for line in file.lines() {
        let mut max = 0;
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                let n: isize = line.chars().nth(i).unwrap().to_string().parse::<isize>().unwrap() * 10 + line.chars().nth(j).unwrap().to_string().parse::<isize>().unwrap();
                if n > max {
                    max = n;
                }
            }
        }
        sum += max;
    }

    println!("{sum}");
}
