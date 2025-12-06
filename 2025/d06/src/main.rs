use std::{env::args, fs::read_to_string, usize, vec};

fn main() {
    let args: Vec<String> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();
    let mut columns: Vec<Vec<String>> = vec![Vec::new(); 2];
    for (i, l) in file.lines().enumerate() {
        let line = l.split_whitespace().collect::<Vec<&str>>().join(" ");
        let split: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        if i == 0 {
            columns = vec![Vec::new(); split.len()];
        }
        for (j, sp) in split.iter().enumerate() {
            columns[j].push(sp.to_string());
        }
    }
    let mut sum = 0;
    let mut operator = "+";
    for c in columns.iter() {
        let mut result = 0;
        for sp in c.iter().rev() {
            if sp == "*" || sp == "+" {
                operator = sp;
                continue;
            }
            if operator == "+" {
                result += sp.parse::<usize>().unwrap();
            } else if operator == "*" {
                if result == 0 {
                    result = 1;
                }
                result *= sp.parse::<usize>().unwrap();
            }
        }
        sum += result;
    }
    println!("{sum}");

    let mut transposed: String = String::from("");
    for x in 0..file.lines().nth(0).unwrap().len() {
        let mut row: String = String::from("");
        for y in 0..file.lines().count() {
            row.push(file.lines().nth(y).unwrap().chars().nth(x).unwrap());
        }
        row.push('\n');
        transposed.push_str(&row);
    }
    let mut sum2 = 0;
    let mut result2 = 0;
    for l in transposed.lines() {
        let line;
        if l.contains("*") {
            operator = "*";
            line = l.split(operator).nth(0).unwrap();
        } else if l.contains("+") {
            operator = "+";
            line = l.split(operator).nth(0).unwrap();
        } else {
            line = l;
        }
        let line = line.trim();
        if line == "" {
            sum2 += result2;
            result2 = 0;
        } else {
            if operator == "*" {
                if result2 == 0 {
                    result2 = 1;
                }
                result2 *= line.parse::<usize>().unwrap();
            } else {
                result2 += line.parse::<usize>().unwrap();
            }
        }
    }
    sum2 += result2;
    println!("{sum2}");
}
