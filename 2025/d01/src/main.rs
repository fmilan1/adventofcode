use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_to_string(args[1].clone()).unwrap();
    let mut dial: isize = 50;
    let mut c: isize = 0;
    let mut c2: isize = 0;
    for line in file.lines() {
        let change_by: isize;
        let mut left = false;
        if line.chars().collect::<Vec<_>>()[0] == 'R' {
            change_by = line.split("R").collect::<Vec<_>>()[1]
                .parse::<isize>()
                .unwrap();
        } else {
            change_by = line.split("L").collect::<Vec<_>>()[1]
                .parse::<isize>()
                .unwrap();
            left = true;
        }

        let mut clicks = 0;
        let mut d = dial;
        for _i in 0..change_by {
            if left {
                d -= 1;
            } else {
                d += 1;
            }
            if d > 99 {
                d -= 100;
            } else if d < 0 {
                d += 100;
            }
            if d == 0 {
                clicks += 1;
            }
        }
        c2 += clicks;

        if left {
            dial -= change_by % 100;
        } else {
            dial += change_by % 100;
        }

        if dial > 99 {
            dial -= 100;
        } else if dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            c += 1;
        }
    }
    println!("{c}\n{}", c2);
}
