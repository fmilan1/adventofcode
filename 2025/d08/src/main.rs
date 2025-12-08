use std::{env::args, fs::read_to_string, isize, vec};

fn distance(b1: (isize, isize, isize), b2: (isize, isize, isize)) -> f64 {
    return (((b1.0 - b2.0).pow(2) + (b1.1 - b2.1).pow(2) + (b1.2 - b2.2).pow(2)) as f64).sqrt();
}

fn find_connection(b: (isize, isize, isize), connections: &Vec<Vec<(isize, isize, isize)>>) -> isize {
    for (i, c) in connections.iter().enumerate() {
        if c.contains(&b) {
            return i as isize;
        }
    }
    -1
}

fn main() {
    let file = read_to_string(args().collect::<Vec<String>>()[1].clone()).unwrap();
    let mut boxes: Vec<(isize, isize, isize)> = Vec::new();
    for line in file.lines() {
        let coords = line.split(",").map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        boxes.push((coords[0], coords[1], coords[2]));
    }

    let mut connections: Vec<Vec<(isize, isize, isize)>> = Vec::new();
    let mut sorted: Vec<(f64, (isize, isize, isize), (isize, isize, isize))> = Vec::new();
    for i in 0..boxes.len() {
        for j in i+1..boxes.len() {
            sorted.push((distance(boxes[i], boxes[j]), boxes[i], boxes[j]));
        }
    }
    sorted.sort_by_key(|a| a.0 as isize);
    for (i, pairs) in sorted.iter().enumerate() {
        if i == 1000 {
            break;
        }
        let a = pairs.1;
        let b = pairs.2;
        let con1 = find_connection(a, &connections);
        let con2 = find_connection(b, &connections);
        if con1 == -1 && con2 == -1 {
            connections.push(vec![a, b]);
        } else if con1 == -1 {
            connections[con2 as usize].push(a);
        } else if con2 == -1 {
            connections[con1 as usize].push(b);
        }
        if con1 != con2 && con1 != -1 && con2 != -1 {
            for b in connections[con2 as usize].clone().iter() {
                connections[con1 as usize].push(*b);
            }
            connections.remove(con2 as usize);
        }
    }

    connections.sort_by_key(|c| -(c.len() as isize));
    let mut res = 1;
    for (i, c) in connections.iter().enumerate() {
        if i == 3 {
            break;
        }
        res *= c.len();
    }
    println!("{res}");
}
