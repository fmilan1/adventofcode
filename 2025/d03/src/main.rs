use std::{char, env::args, fs::read_to_string};

fn max_kdigit_string(line: String, k: usize) -> usize {
    let mut remaining = k;
    let mut str_n = String::new();
    let mut max_idx = 0;
    let mut max = line.chars().nth(max_idx).unwrap().to_digit(10).unwrap();
    while remaining > 0 {
        for i in max_idx + 1..line.len() - remaining + 1 {
            let digit = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if digit > max {
                max_idx = i;
                max = digit;
            }
        }
        str_n.push(char::from_digit(max, 10).unwrap());
        remaining -= 1;
        max = 0;
    }
    str_n.parse::<usize>().unwrap()
}

fn main() {
    let args: Vec<_> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();

    let mut sum = 0;
    let mut sum2 = 0;
    for line in file.lines() {
        sum += max_kdigit_string(line.to_string(), 2);
        sum2 += max_kdigit_string(line.to_string(), 12);
    }

    println!("{sum}");
    println!("{sum2}");
}
