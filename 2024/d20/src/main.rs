use std::collections::{HashMap, HashSet};

const DIRS:[(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {

    let mut queue: Vec<((usize, usize), usize)> = vec![(start, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while queue.len() > 0 {
        let (current, step) = queue.remove(0);
        if current == end {
            return step;
        }
        
        for d in DIRS {
            let (nx, ny) = ((current.0 as isize + d.0) as usize, (current.1 as isize + d.1) as usize);
            if nx >= map[0].len() || ny >= map.len() || map[ny][nx] == '#' { continue; }
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push(((nx, ny), step + 1));
            }
        }
    }

    0
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

    let official_steps = bfs(&map, (sx, sy), (ex, ey));
    let mut saves: HashMap<usize, usize> = HashMap::new();
    for i in 1..map.len() - 1{
        for j in 1..map[0].len() - 1 {
            let mut m = map.clone();
            if m[j][i] == '#' {
                m[j][i] = '.';

                let save = official_steps - bfs(&m, (sx, sy), (ex, ey));
                saves.entry(save).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    println!("{}", saves.iter().filter(|kv| *kv.0 >= 100).map(|kv| *kv.1).sum::<usize>());
}
