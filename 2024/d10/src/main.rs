use std::{collections::{HashMap, HashSet}};

fn track1(map: &Vec<Vec<i32>>, i: usize, j: usize, start: (usize, usize), prev: i32, scores: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>, visited: &mut  Vec<Vec<bool>>) -> bool {
    if i >= map.len() || j >= map[i].len() || visited[i][j] { return false; }
    if prev + 1 != map[i][j] { return false; }

    visited[i][j] = true;
    if map[i][j] == 9 { 
        if !scores.contains_key(&start) {
            scores.insert(start, HashSet::from([(i, j)]));
        }
        else {
            scores.get_mut(&start).unwrap().insert((i, j));
        }
        return true;
    }
    let mut tmp = false;
    let dirs: Vec<(i8, i8)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in dirs {
        if track1(map, i + dir.0 as usize, j + dir.1 as usize, start, map[i][j], scores, visited) { tmp = true; }
    }
    tmp
}


fn track2(map: &Vec<Vec<i32>>, i: usize, j: usize, start: (usize, usize), prev: i32, scores: &mut HashMap<(usize, usize), i32>) -> bool {
    if i >= map.len() || j >= map[i].len() { return false; }
    if prev + 1 != map[i][j] { return false; }

    if map[i][j] == 9 { 
        if !scores.contains_key(&start) {
            scores.insert(start, 1);
        }
        else {
            *scores.get_mut(&start).unwrap() += 1;
        }
        return true;
    }
    let mut tmp = false;
    let dirs: Vec<(i8, i8)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in dirs {
        if track2(map, i + dir.0 as usize, j + dir.1 as usize, start, map[i][j], scores) { tmp = true; }
    }
    tmp
}

fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let map: Vec<Vec<i32>> = file.lines().map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect();
    let mut scores1: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut scores2: HashMap<(usize, usize), i32> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
            track1(&map, i, j, (i, j), -1, &mut scores1, &mut visited);
            track2(&map, i, j, (i, j), -1, &mut scores2);
        }
    }
    let mut sum1 = 0;
    for score in scores1 {
        sum1 += score.1.len();
    }
    println!("{sum1}");
    let mut sum2 = 0;
    for score in scores2 {
        sum2 += score.1;
    }
    println!("{sum2}");
}
