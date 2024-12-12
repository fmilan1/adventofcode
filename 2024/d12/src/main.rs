use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Garden {
    plant: char,
    count: usize,
    perimeter: usize,
    sides: usize,
    perim_locations: HashSet<(usize, usize)>,
    locations: Vec<(usize, usize)>,
    connects_at: Vec<(usize, usize)>,
}

fn find_garden(map: &Vec<Vec<char>>, i: usize, j: usize, garden: &mut Garden, visited: &mut Vec<Vec<bool>>, _gardens: &mut Vec<Garden>) -> bool {
    if i >= map.len() || j >= map[0].len() || map[i][j] != garden.plant || visited[i][j] { return false }

    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    visited[i][j] = true;
    for dir in &dirs {
        if (i + dir.0 as usize) < map.len() && (j + dir.1 as usize) < map[0].len() && map[i + dir.0 as usize][j + dir.1 as usize] != garden.plant {
            garden.connects_at.push((i + dir.0 as usize, j + dir.1 as usize));
        }
        if find_garden(map, i + dir.0 as usize, j + dir.1 as usize, garden, visited, _gardens) {
            garden.count += 1;
        }
    }
    if map[i][j] == garden.plant {
        garden.perimeter += calc_perimter(map, i, j, garden.plant, garden);
        garden.locations.push((i, j));
        return true;
    }

    false 
}

fn calc_perimter(map: &[Vec<char>], i: usize, j: usize, plant: char, garden: &mut Garden) -> usize {
    let mut perimeter = 0;
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in dirs {
        if i + dir.0 as usize >= map.len() || j + dir.1 as usize >= map[0].len() {
            garden.perim_locations.insert((i + dir.0 as usize, j + dir.1 as usize));
            perimeter += 1;
            continue;
        }
        if map[i + dir.0 as usize][j + dir.1 as usize] != plant {
            garden.perim_locations.insert((i + dir.0 as usize, j + dir.1 as usize));
            perimeter += 1;
        }
    }
    perimeter
}

fn turn_right(dir: &mut (i32, i32)) {
    *dir = (dir.1, -dir.0);
}

fn turn_left(dir: &mut (i32, i32)) {
    *dir = (-dir.1, dir.0);
}

fn calc_sides(map: &[Vec<char>], start_i: usize, start_j: usize, plant: char, garden: &Garden) -> usize {
    let mut tmp = vec![vec!['.'; map[0].len() + 4]; map.len() + 4];
    for l in &garden.locations {
        tmp[l.0 + 2][l.1 + 2] = garden.plant;
    }
    let mut sides = 0;

    let mut i = start_i;
    let mut j = start_j;
    let mut dir: (i32, i32) = (0, 1);
    loop {
        if tmp[i + dir.1 as usize][j - dir.0 as usize] != plant {
            turn_right(&mut dir);
            sides += 1;
        }
        else if tmp[i + dir.0 as usize][j + dir.1 as usize] == plant {
            turn_left(&mut dir);
            sides += 1;
        }

        if tmp[i + dir.0 as usize][j + dir.1 as usize] != plant {
            i += dir.0 as usize;
            j += dir.1 as usize;
        }

        if i == start_i && j == start_j {
            break;
        }
    }
    sides
}

fn main() {
    let map: Vec<Vec<char>> = include_str!("../input2.txt").lines().map(|line| line.chars().collect()).collect();
    let mut gardens: Vec<Garden> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            let mut garden = Garden { plant: *c, count: 1, perimeter: 0, perim_locations: HashSet::new(), locations: Vec::new(), sides: 0, connects_at: Vec::new(), };
            if find_garden(&map, i, j, &mut garden, &mut visited, &mut gardens) {
                let min = garden.locations.iter().min().unwrap();
                garden.sides = calc_sides(&map, min.0 + 1, min.1 + 2, garden.plant, &garden);
                gardens.push(garden);
            }
        }
    }

    let price: usize = gardens.iter().map(|g| g.count * g.perimeter).collect::<Vec<usize>>().iter().sum();
    println!("{}", price);

    let gardens2 = gardens.clone();
    for g in &mut gardens {
        let mut connects_to: Vec<&Garden> = Vec::new();
        for g1 in &gardens2 {
            if g1.locations.iter().any(|loc| g.connects_at.contains(loc)) {
                connects_to.push(g1);
            }
        }
        for con in connects_to.iter_mut() {
            let mut good = true;
            for p in &con.perim_locations {
                if !g.locations.contains(p) {
                    good = false;
                    break;
                }
            }
            if good {
                g.sides += con.sides;
            }
        }
    }
    let price: usize = gardens.iter().map(|g| g.count * g.sides).collect::<Vec<usize>>().iter().sum();
    println!("{}", price);
}
