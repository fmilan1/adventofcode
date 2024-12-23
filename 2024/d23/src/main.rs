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

fn bron_kerbosch(graph: &BTreeMap<String, BTreeSet<String>>, r: &mut BTreeSet<String>, p: &mut BTreeSet<String>, x: &mut BTreeSet<String>, cliques: &mut Vec<BTreeSet<String>>) -> Vec<BTreeSet<String>>{
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }
    for node in p.clone().iter() {
        let mut new_r = r.clone();
        let mut new_p = BTreeSet::new();
        let mut new_x = BTreeSet::new();
        new_r.insert(node.to_string());
        for n in graph[node].iter() {
            if p.contains(n) {
                new_p.insert(n.to_string());
            }
            if x.contains(n) {
                new_x.insert(n.to_string());
            }
        }
        bron_kerbosch(graph, &mut new_r, &mut new_p, &mut new_x, cliques);
        new_p = p.clone();
        new_p.remove(node);
        new_x = x.clone();
        new_x.insert(node.to_string());
        *p = new_p.clone();
        *x = new_x.clone();
    }
    cliques.to_vec()
}

fn main() {
    let file = include_str!("../input2.txt");
    let mut connections: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for line in file.lines() {
        let arr = line.split("-").collect::<Vec<&str>>();
        connections.entry(arr[0].to_string()).and_modify(|set| {set.insert(arr[1].to_string());}).or_insert(BTreeSet::from([arr[1].to_string()]));
        connections.entry(arr[1].to_string()).and_modify(|set| {set.insert(arr[0].to_string());}).or_insert(BTreeSet::from([arr[0].to_string()]));
    }

    let mut sets: BTreeSet<Vec<String>> = BTreeSet::new();
    for pc in connections.keys().into_iter() {
        let set = bfs(&connections, pc.to_string());
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


    let keys = connections.keys().map(|k| k.to_string()).collect::<Vec<String>>();
    let mut set: BTreeSet<String> = BTreeSet::new();
    for k in keys.iter() {
        set.insert(k.to_string());
    }

    let cliques = bron_kerbosch(&connections, &mut BTreeSet::new(), &mut set, &mut BTreeSet::new(), &mut Vec::new());

    let mut max_clique = cliques.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().iter().map(|a| a.to_string()).collect::<Vec<String>>();
    max_clique.sort();
    println!("{}", max_clique.join(","));
}
