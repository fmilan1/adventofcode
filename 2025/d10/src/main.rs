use std::{env::args, fs::read_to_string, usize};

fn main() {
    let file = read_to_string(args().collect::<Vec<String>>()[1].clone()).unwrap();
    let mut sum = 0;
    for line in file.lines() {
        let tokens: Vec<String> = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut diagram = tokens.first().unwrap().chars().map(|c| if c == '#' { true } else { false }).collect::<Vec<bool>>();
        diagram.remove(0);
        diagram.pop();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for t in tokens.iter().skip(1).take(tokens.len() - 2) {
            let tokens2 = t.replace("(", "").replace(")", "").split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
            buttons.push(tokens2);
        }
        let mut map: Vec<(Vec<bool>, Vec<Vec<usize>>)> = Vec::new();

        for btn in buttons.iter() {
            let mut lights = vec![false; tokens[0].len() - 2];
            for b in btn.iter() {
                lights[*b] = !lights[*b];
            }
            map.push((lights, vec![btn.clone()]));
        }

        for btn in buttons.iter() {
            for m in map.clone().iter() {
                let (mut tmp_vec, mut tmp_btns) = m.clone();
                for b in btn.iter() {
                    tmp_vec[*b] = !tmp_vec[*b];
                }
                tmp_btns.push(btn.clone());
                map.push((tmp_vec, tmp_btns));
            }
        }
        map.sort_by_key(|m| m.1.len());
        for m in map.iter() {
            if m.0 == diagram {
                sum += m.1.len();
                break;
            }
        }
    }
    println!("{sum}");
}
