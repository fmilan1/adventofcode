use std::collections::{BinaryHeap, HashMap};

fn dijsktra(map: &Vec<Vec<char>>, start: Pos, end: Pos) -> isize {
                            // cost - position - direction
    let mut table: BinaryHeap<(isize, Pos, (isize, isize))> = BinaryHeap::new(); 
    let mut visited: HashMap<Pos, isize> = HashMap::new();

    table.push((0, start, (0, 1)));

    while let Some((cost, current_pos, dir)) = table.pop() {
        if visited.contains_key(&current_pos) {
            continue;
        }
        visited.insert(current_pos.clone(), cost);
        
        if current_pos == end {
            return -cost;
        }

        for (neighbor, move_cost, new_dir) in get_neighbors(map, current_pos, dir) {
            let new_cost = move_cost + cost;
            if visited.get(&neighbor).map_or(true, |&v| new_cost < -v) {
                table.push((new_cost, neighbor, new_dir));
            }
        }

    }
    -1
}

fn get_neighbors(map: &Vec<Vec<char>>, position: Pos, dir: (isize, isize)) -> Vec<(Pos, isize, (isize, isize))>{
    let mut neighbors: Vec<(Pos, isize, (isize, isize))> = Vec::new();
    for new_dir in DIRS {
        let neighbor: Pos = Pos{ x: (position.x as isize + new_dir.1) as usize, y: (position.y as isize + new_dir.0) as usize};
        if neighbor.x >= map[0].len() || neighbor.y >= map.len() || map[neighbor.y][neighbor.x] == '#' { continue; }
        let cost = if new_dir != dir {1001} else {1};
        neighbors.push((neighbor, -cost, new_dir));
    }
    neighbors
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
fn main() {
    let map: Vec<Vec<char>> = include_str!("../input2.txt").lines().map(|l| l.chars().collect()).collect();
    println!("{}", dijsktra(&map, Pos { x: 1, y: map[0].len() - 2 }, Pos { x: map.len() - 2, y: 1 }));
}
