use std::collections::{BTreeSet, BinaryHeap};

fn bfs(patterns: &BTreeSet<String>, design: &String) -> bool {
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut heap: BinaryHeap<(usize, String)> = BinaryHeap::new();
    heap.push((0, String::new()));

    while heap.len() > 0 {
        let (_, path) = heap.pop().unwrap();

        for pattern in patterns.iter() {
            let new_path = path.to_owned() + pattern;
            if visited.contains(&new_path) { continue; }
            if new_path == *design {
                return true;
            }
            if new_path.len() >= design.len() { continue; }
            if !design.starts_with(&new_path) { continue; }
            visited.insert(new_path.to_owned());
            heap.push((new_path.len(), new_path));
        }
    }
    false
}

fn main() {
    let file = include_str!("../input2.txt");
    let patterns: Vec<String> = file.split("\n\n").nth(0).unwrap().split(", ").map(|a| a.to_string()).collect();
    let designs: Vec<String> = file.split("\n\n").nth(1).unwrap().lines().map(|a| a.to_string()).collect();

    let mut count = 0;
    for  design in designs.iter() {
        let mut new_patterns = BTreeSet::new();
        for p in patterns.iter() {
            if design.contains(p) {
                new_patterns.insert(p.to_owned());
            }
        }
        if bfs(&new_patterns, &design) {
            count += 1;
        }
    }
    println!("{count}");
}
