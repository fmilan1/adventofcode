use std::collections::HashSet;

const N: usize = 71;
const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(mem: &[[i32; N]; N], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<((usize, usize), usize)> = vec![(start, 0)];

    while queue.len() > 0 {
        let (current, step) = queue.remove(0);
        if current == end {
            return Some(step);
        }
        for d in DIRS {
                let neighbor = ((current.0 as isize + d.0) as usize, (current.1 as isize + d.1) as usize);
            if !visited.contains(&neighbor) {
                if neighbor.0 >= N || neighbor.1 >= N || mem[neighbor.1][neighbor.0] == 1 { continue; }
                visited.insert(neighbor);
                queue.push((neighbor, step + 1));
            }
        }
    }
    None
}

fn main() {
    let file = include_str!("../input2.txt");
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for line in file.lines() {
        let arr = line.split(",").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
        coords.push((arr[0], arr[1]));
    }
    let mut mem = [[0; N]; N];
    for i in 0..1024 {
        mem[coords[i].1][coords[i].0] = 1;
    }
    if let Some(s) = bfs(&mem, (0, 0), (N - 1, N - 1)) {
        println!("{s}");
    }
    let mut mem = [[0; N]; N];
    let mut i = 0;
    loop {
        mem[coords[i].1][coords[i].0] = 1;
        if let None = bfs(&mem, (0, 0), (N - 1, N - 1)) {
            println!("{:?}", coords[i]);
            break;
        }
        i += 1;
    }
}
