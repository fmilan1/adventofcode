use std::{env::args, fs::read_to_string, usize};

fn evaluate(input: String) -> usize {
    let mut _left: String = String::from("");
    let mut _right: String = String::from("");
    let mut new_input = input.clone();
    if input.parse::<usize>().is_ok() {
        return input.parse::<usize>().unwrap();
    } else {
        while new_input.contains('(') {
            let mut opened = 0;
            let first_idx = new_input.find("(").unwrap();
            let mut last_idx = 0;
            for i in first_idx..new_input.len() {
                let c = new_input.chars().nth(i).unwrap();
                if c == '(' {
                    opened += 1;
                } else if c == ')' {
                    opened -= 1;
                }
                if i != 0 && opened == 0 {
                    last_idx = i;
                    break;
                }
            }
            let s: String = new_input.chars().skip(first_idx + 1).take(last_idx - first_idx - 1).collect();
            let res = evaluate(s.clone());
            let rest: String = new_input.clone().chars().skip(last_idx + 1).collect();
            new_input = new_input.chars().take(first_idx).collect();
            new_input.push_str(&res.to_string());
            new_input.push_str(&rest);
        }
    }

    let mut operator = '+';
    for c in new_input.chars() {
        if c == '+' {
            operator = '+';
            break;
        } else if c == '*' {
            operator = '*';
            break;
        }
    }
    (_left, _right) = new_input.split_once(operator).map(|a| (a.0.to_string(), a.1.to_string())).unwrap();
    let left_parsed = _left.parse::<usize>();
    let right_parsed = _right.parse::<usize>();
    if left_parsed.is_ok() && right_parsed.is_ok() {
        if operator == '+' {
            return left_parsed.unwrap() + right_parsed.unwrap();
        } else {
            return left_parsed.unwrap() * right_parsed.unwrap();
        }
    } else {
        if operator == '+' {
            return evaluate(_left) + evaluate(_right);
        } else {
            return evaluate(_left) * evaluate(_right);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut file = read_to_string(args[1].clone()).unwrap();
    file = file.split(" ").collect();
    let mut sum = 0;
    for line in file.lines() {
        let mut reversed: String = line.chars().rev().collect();
        reversed = reversed.replace("(", "]");
        reversed = reversed.replace(")", "(");
        reversed = reversed.replace("]", ")");
        sum += evaluate(reversed);
    }
    println!("{sum}");
}
