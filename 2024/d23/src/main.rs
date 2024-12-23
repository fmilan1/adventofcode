use std::collections::{BTreeMap, BTreeSet};

fn bfs(graph: &BTreeMap<String, BTreeSet<String>>, start: String) -> BTreeSet<Vec<String>> {
    let mut queue: Vec<(String, Vec<String>)> = vec![(start.to_string(), Vec::new())];
    let mut sets = BTreeSet::new();

    while queue.len() > 0 {
        let (current, path) = queue.remove(0);

        if start == current && path.len() == 3 {
            let mut tmp = path.clone();
            tmp.sort();
            sets.insert(tmp);
        }

        for neighbor in graph[&current].iter() {
            if path.contains(&current) { continue; }
            let mut tmp = path.clone();
            tmp.push(current.to_string());
            if tmp.len() > 3 { continue; }
            queue.push((neighbor.to_string(), tmp));
        }
    }
    
    sets
}

fn main() {
    let file = include_str!("../input2.txt");
    let mut conns: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for line in file.lines() {
        let arr = line.split("-").collect::<Vec<&str>>();
        conns.entry(arr[0].to_string()).and_modify(|set| {set.insert(arr[1].to_string());}).or_insert(BTreeSet::from([arr[1].to_string()]));
        conns.entry(arr[1].to_string()).and_modify(|set| {set.insert(arr[0].to_string());}).or_insert(BTreeSet::from([arr[0].to_string()]));
    }

    let mut sets: BTreeSet<Vec<String>> = BTreeSet::new();
    for pc in conns.keys().into_iter() {
        let mut set = bfs(&conns, pc.to_string());
        for s in set.iter() {
            sets.insert(s.to_vec());
        }
    }

    let mut count = 0;
    for set in sets.iter() {
        for pc in set.iter() {
            if pc.starts_with("t") {
                count += 1;
                break;
            }
        }
    }
    println!("{count}");

}
