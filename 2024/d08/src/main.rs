use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let map: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();
    let mut antinodes1: Vec<Vec<char>> = vec![vec!['.'; map[0].len()]; map.len()];
    let mut antinodes2 = antinodes1.clone();
    let mut freqs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, line) in map.clone().into_iter().enumerate() {
        for (j, c) in line.into_iter().enumerate() {
            if c == '.' { continue; }
            if !freqs.contains_key(&c) {
                freqs.insert(c, vec![(i, j)]);
            }
            else {
                freqs.get_mut(&c).unwrap().push((i, j));
            }
        }
    }

    for positions in freqs.into_values() {
        for i in 0..positions.clone().into_iter().len() - 1 {
            for j in i + 1..positions.clone().into_iter().len() {
                let dis: (usize, usize) = (positions[j].0 - positions[i].0, positions[j].1 - positions[i].1);
                let new_i: (usize, usize) = (positions[i].0 - dis.0, positions[j].0 + dis.0);
                let new_j: (usize, usize) = (positions[i].1 - dis.1, positions[j].1 + dis.1);
                if new_i.0 < map.len() && new_j.0 < map[0].len() {
                    antinodes1[new_i.0][new_j.0] = '#';
                }
                if new_i.1 < map.len() && new_j.1 < map[0].len() {
                    antinodes1[new_i.1][new_j.1] = '#';
                }
                for k in 0..map[0].clone().len() {
                    let dis: (usize, usize) = (positions[j].0 - positions[i].0, positions[j].1 - positions[i].1);
                    let new_i: (usize, usize) = (positions[i].0 - dis.0 * k, positions[j].0 + dis.0 * k);
                    let new_j: (usize, usize) = (positions[i].1 - dis.1 * k, positions[j].1 + dis.1 * k);
                    if new_i.0 < map.len() && new_j.0 < map[0].len() {
                        antinodes2[new_i.0][new_j.0] = '#';
                    }
                    if new_i.1 < map.len() && new_j.1 < map[0].len() {
                        antinodes2[new_i.1][new_j.1] = '#';
                    }
                }
            }
        }
    }
    let mut count1 = 0;
    let mut count2 = 0;
    for i in 0..antinodes1.len() {
        for j in 0..antinodes1[i].len() {
            if antinodes1[i][j] == '#' { count1 += 1; }
            if antinodes2[i][j] == '#' { count2 += 1; }
        }
    }
    println!("{count1}");
    println!("{count2}");
}
