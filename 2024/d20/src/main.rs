use std::collections::{BTreeMap, HashSet};

const DIRS:[(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {

    let mut queue: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![(start, vec![])];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while queue.len() > 0 {
        let (current, path) = queue.remove(0);
        if current == end {
            return path;
        }
        
        for d in DIRS {
            let (nx, ny) = ((current.0 as isize + d.0) as usize, (current.1 as isize + d.1) as usize);
            if nx >= map[0].len() || ny >= map.len() || map[ny][nx] == '#' { continue; }
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                let mut tmp = path.clone();
                tmp.push(current);
                queue.push(((nx, ny), tmp ));
            }
        }
    }

    Vec::new()
}

fn main() {
    let map: Vec<Vec<char>> = include_str!("../input2.txt").lines().map(|l| l.chars().collect()).collect();
    let (mut ex, mut ey): (usize, usize) = (0, 0);
    let (mut sx, mut sy): (usize, usize) = (0, 0);
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' { (sx, sy) = (j, i); }
            else if *c == 'E' { (ex, ey) = (j, i); }
        }
    }

    let mut path = bfs(&map, (sx, sy), (ex, ey));
    path.push((ex ,ey));

    let mut saves1: BTreeMap<isize, usize> = BTreeMap::new();
    let mut saves2: BTreeMap<isize, usize> = BTreeMap::new();
    let mut tmp = path.clone();
    tmp.reverse();
    for i in 0..path.len(){
        for j in 0..tmp.len() {
            let manhattan = (path[i].0 as isize - tmp[j].0 as isize).abs() + (path[i].1 as isize - tmp[j].1 as isize).abs();
            let save = path.len() as isize - 1 - (i as isize + j as isize + manhattan);
            if save < 100 { continue; }

            if manhattan == 2 {
                saves1.entry(save).and_modify(|v| *v += 1).or_insert(1);
            }
            
            if manhattan <= 20 {

                saves2.entry(save).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }

    println!("{}", &saves1.iter().map(|kv| kv.1).sum::<usize>());
    println!("{}", &saves2.iter().map(|kv| kv.1).sum::<usize>());
}
