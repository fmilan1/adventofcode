use std::collections::{HashMap};

fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let mut guard: Vec<i32> = vec![0, 0];
    let mut dir: Vec<i32>= vec![-1, 0];
    let mut map: Vec<Vec<char>> = Vec::new();
    for (i, line) in file.lines().enumerate() {
        map.push(line.chars().collect());
        if line.contains('^') { 
            guard[0] = i as i32;
            guard[1] = line.chars().position(|c| c == '^').unwrap() as i32;
        }
    }
    let default = guard.clone();
    
    let mut path = map.clone();
    loop {
        if guard[0] == 0 || guard[0] as usize == map.len() - 1 || guard[1] == 0 || guard[1] as usize == map[0].len() - 1 { break; }
        if map[(guard[0] + dir[0]) as usize][(guard[1] + dir[1]) as usize] != '#' {
            guard[0] += dir[0];
            guard[1] += dir[1];
            path[guard[0] as usize][guard[1] as usize] = 'X';
        }
        else { 
            dir = vec![dir[1], dir[0] * -1];
        }
    }
    let mut c = 0;
    for a in path.iter() {
        for b in a.iter() {
            if *b == 'X' { c += 1; }
        }
    }
    println!("{c}");
    
    c = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            guard = default.clone();
            dir = vec![-1, 0];
            path = map.clone();
            path[i][j] = 'O';
            let mut visited: HashMap<Vec<i32>, Vec<Vec<i32>>> = HashMap::new();
            loop {
                if guard[0] == 0 || guard[0] as usize == map.len() - 1 || guard[1] == 0 || guard[1] as usize == map[0].len() - 1 { break; }
                let ahead = path[(guard[0] + dir[0]) as usize][(guard[1] + dir[1]) as usize];
                if ahead != '#' && ahead != 'O' {
                    guard[0] += dir[0];
                    guard[1] += dir[1];
                    path[guard[0] as usize][guard[1] as usize] = if dir == vec![-1, 0] || dir == vec![1, 0] {'|'} else {'-'};
                }
                else { 
                    dir = vec![dir[1], dir[0] * -1];
                    path[guard[0] as usize][guard[1] as usize] = '+';
                }
                if visited.contains_key(&guard.clone()) {
                    let mut tmp= visited.get(&guard.clone()).unwrap().clone();
                    if tmp.contains(&dir.clone()) {
                        c += 1;
                        break;
                    }
                    else {
                        tmp.push(dir.clone());
                        visited.insert(guard.clone(), tmp);
                    }
                }
                else {
                    visited.insert(guard.clone(), vec![dir.clone()]);
                }
            }
        }
    }
    println!("{c}");
}
