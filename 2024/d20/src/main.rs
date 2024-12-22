use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, path};

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

fn bfs_cheat(map: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<((usize, usize), usize)>{
    let mut queue: Vec<((usize, usize), usize)> = vec![(start, 1)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut cheats: Vec<((usize, usize), usize)> = Vec::new();
    while queue.len() > 0 {
        let (current, step) = queue.remove(0);
        if map[current.1][current.0] == '.' /*|| map[current.1][current.0] == 'E'*/{
            if step > 20 { continue; }
            cheats.push((current, step));
            // println!("{:?} {}", current, step);
            continue;
            // return step;
        }
        
        for d in DIRS {
            let (nx, ny) = ((current.0 as isize + d.0) as usize, (current.1 as isize + d.1) as usize);
            if nx >= map[0].len() - 1 || ny >= map.len() - 1 || nx == 0 || ny == 0 || map[ny][nx] == 'S' /*|| map[ny][nx] == 'E'*/ { continue; }
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push(((nx, ny), step + 1));
            }
        }
    }
    
    cheats
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

    println!("{:?}", bfs(&map, (sx, sy), (ex, ey)).len());

    let mut path = bfs(&map, (sx, sy), (ex, ey));
    path.push((ex ,ey));


    let mut saves: BTreeMap<usize, usize> = BTreeMap::new();
    let mut tmp = path.clone();
    tmp.reverse();
    for i in 0..path.len(){
        for j in 0..tmp.len() / 2{
            let manhattan = (path[i].0 as isize - tmp[j].0 as isize).abs() + (path[i].1 as isize - tmp[j].1 as isize).abs();
            if manhattan <= 20 {
                let save = path.len() as isize - 1 - (i as isize + j as isize + manhattan);
                // if save != 76 { continue; }
                // println!("{} {:?} {:?} {}", save, path[i], tmp[j], manhattan);
                if save >= 100 {

                saves.entry(save as usize).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }

    dbg!(&saves.iter().map(|kv| kv.1).sum::<usize>());
    // dbg!(&saves.iter().filter(|kv| *kv.0 >= 100).map(|kv| kv.1).sum::<usize>());

        // 85709 low
        // 85878 low
        // 88898 low
        // 657911
        // 544309

    // dbg!(&saves);


    // let mut saves: BTreeMap<usize, BTreeSet<((usize, usize), (usize, usize))>> = BTreeMap::new();
    // 'asd: for (i, p) in official_path.iter().enumerate() {
    //     'dsa: for d in DIRS {
    //         let (nx, ny) = ((p.0 as isize + d.0) as usize, (p.1 as isize + d.1) as usize);
    //         if nx >= map[0].len() - 1 || ny >= map.len() - 1 || nx == 0 || ny == 0 { continue; }
    //         if map[ny][nx] == '#' {
    //             let cheats = bfs_cheat(&map, (nx, ny));
    //             // println!("{:?}\n", cheats);
    //             let mut m: Vec<Vec<char>> = map.iter().map(|l| l.iter().map(|c| if *c == '.' {' '}else {*c}).collect()).collect();
    //             let mut asd = false;
    //             for (cheat_end, cheat_step) in cheats.iter() {
    //                 // if *cheat_end != (3, 7) { continue; }
    //                 // println!("{:?} {:?}", i, cheat_end);
    //                 let path_after_cheat = bfs(&map, *cheat_end, (ex, ey));
    //                 let all_steps_with_cheat = i + path_after_cheat.len() + cheat_step;
    //                 if all_steps_with_cheat < official_path.len() && official_path.len() - all_steps_with_cheat >= 50 {
    //                     // if i != 0 { continue; }
    //                     let save = official_path.len() - all_steps_with_cheat;
    //                     // if save != 74 { continue; }
    //                     // saves.entry(save).and_modify(|v| *v += 1).or_insert(1);
    //                     // saves.entry(save).and_modify(|v| {v.insert(((p.0, p.1), (cheat_end.0, cheat_end.1)));}).or_insert(BTreeSet::from([((p.0, p.1), (cheat_end.0, cheat_end.1))]));
    //                     saves.entry(save).and_modify(|v| {v.insert(((nx, ny), (cheat_end.0, cheat_end.1)));}).or_insert(BTreeSet::from([((nx, ny), (cheat_end.0, cheat_end.1))]));
    //                     m[cheat_end.1][cheat_end.0] = 'e';
    //                     m[ny][nx] = 's';
    //                     asd = true;
    //                     println!("save: {}\ti: {}\tch step: {}\tch e: {:?}\tch sta: {:?} {:?}", save, i, cheat_step, cheat_end, (nx, ny), p);
    //                 }

    //             }
    //                     // continue 'dsa;
    //             if asd {

    //             for line in m.iter() {
    //                 for c in line.iter() {
    //                     print!("{c}");
    //                 }
    //                 println!();
    //             }
    //             println!();
    //             }
    //             // for line in map.iter() {
    //             //     for c in line.iter() {
    //             //         print!("{c}");
    //             //     }
    //             //     println!();
    //             // }
    //         }
    //         // break;
    //     }
    //     // break;
    // }
    // for s in saves {
    //     println!("{}: {}", s.0, s.1.iter().count());
    // }



    // let official_steps = bfs(&map, (sx, sy), (ex, ey));
    // let mut saves: HashMap<usize, usize> = HashMap::new();
    // for i in 1..map.len() - 1{
    //     for j in 1..map[0].len() - 1 {
    //         let mut m = map.clone();
    //         if m[j][i] == '#' {
    //             m[j][i] = '.';

    //             let save = official_steps - bfs(&m, (sx, sy), (ex, ey));
    //             saves.entry(save).and_modify(|v| *v += 1).or_insert(1);
    //         }
    //     }
    // }
    // println!("{}", saves.iter().filter(|kv| *kv.0 >= 100).map(|kv| *kv.1).sum::<usize>());

    // println!("{:?}", bfs_cheat(&map, (2, 3)));
}
