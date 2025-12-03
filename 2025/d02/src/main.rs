use std::{env::args, fs::read_to_string};

fn count(str: String, substring: String) -> (bool, isize) {
    if str.len() < substring.len() {
        return (true, 0);
    }

    let (head, tail) = str.split_at(substring.len());
    if tail.len() > 0 && !tail.contains(&substring) {
        return (false, 0);
    }

    if head == substring {
        let asd = count(tail.to_string(), substring);
        return (asd.0, 1 + asd.1);
    }

    (false, 0)
}

fn main() {
    let args: Vec<String> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();
    let ranges = file.split(",").collect::<Vec<_>>();
    let mut sum = 0;
    let mut sum2 = 0;
    for r in ranges.clone() {
        let ids = r.split("-").collect::<Vec<_>>();
        let first = ids[0].trim();
        let last = ids[1].trim();

        for i in first.parse::<isize>().unwrap()..last.parse::<isize>().unwrap() + 1 {
            let s = i.to_string();
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                sum += i;
            }

            // second part
            let l = s.len();
            for j in 1..l {
                let substring = s.split_at(j).0.to_string();
                let asd = count(s.to_string(), substring.to_string());
                if asd.0 {
                    sum2 += i;
                    break;
                }
            }
        }
    }
    println!("{sum}");
    println!("{sum2}");
}
