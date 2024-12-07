use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let mut eqs: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in file.lines() {
        let arr: Vec<&str> = line.split(':').collect();
        eqs.insert(arr[0].parse().unwrap(), arr[1].split_ascii_whitespace().map(|a| a.parse::<u64>().unwrap()).collect());
    }
    let mut sum1 = 0;
    let mut sum2 = 0;
    for eq in eqs.iter() {
        let mut e = eq.1[0];
        let mut o: usize = 0;
        let mut i = 1;
        let mut saves: Vec<u64> = vec![e];
        let mut stack: Vec<usize> = Vec::new();
        loop {
            e = *saves.last().unwrap();

            let n = eq.1[i];
            e = if o == 0 { e + n } else { e * n };
            saves.push(e);
            stack.push(o);
            i += 1;
            o = 0;

            if e == *eq.0 && i == eq.1.len() {
                sum1 += eq.0;
                break;
            }
            else if e != *eq.0 && i == eq.1.len() {
                while stack.len() > 0 && *stack.last().unwrap() == 1 {
                    i -= 1;
                    stack.pop();
                    saves.pop();
                }
                if stack.len() == 0 { break; }
                i -= 1;
                o = 1;
                stack.pop();
                saves.pop();
            }
        }

        e = eq.1[0];
        o = 0;
        i = 1;
        saves = vec![e];
        stack = Vec::new();
        loop {
            e = *saves.last().unwrap();

            let n = eq.1[i];
            let mut tmp = saves.last().unwrap().to_string();
            tmp.push_str(&n.to_string());
            e = if o == 0 { e + n } else if o == 1 { e * n } else { tmp.parse().unwrap() };
            saves.push(e);
            stack.push(o);
            i += 1;
            o = 0;

            if e == *eq.0 && i == eq.1.len() {
                sum2 += eq.0;
                break;
            }
            else if e != *eq.0 && i == eq.1.len() {
                while stack.len() > 0 && *stack.last().unwrap() == 2 {
                    i -= 1;
                    stack.pop();
                    saves.pop();
                }
                if stack.len() == 0 { break; }
                i -= 1;
                o += stack.last().unwrap() + 1;
                stack.pop();
                saves.pop();
            }
        }
    }
    println!("{sum1}");
    println!("{sum2}");
}
