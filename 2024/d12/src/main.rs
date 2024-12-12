struct Garden {
    plant: char,
    count: usize,
    perimeter: usize,
}

fn find_garden(map: &Vec<Vec<char>>, i: usize, j: usize, garden: &mut Garden, visited: &mut Vec<Vec<bool>>, gardens: &mut Vec<Garden>) -> bool {
    if i >= map.len() || j >= map[0].len() || map[i][j] != garden.plant || visited[i][j] { return false }

    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    visited[i][j] = true;
    for dir in dirs {
        if find_garden(map, i + dir.0 as usize, j + dir.1 as usize, garden, visited, gardens) {
            garden.count += 1;
        }
    }
    if map[i][j] == garden.plant {
        garden.perimeter += calc_perimter(&map, i, j, garden.plant);
        return true;
    }

    false 
}

fn calc_perimter(map: &Vec<Vec<char>>, i: usize, j: usize, plant: char) -> usize {
    let mut perimeter = 0;
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in dirs {
        if i + dir.0 as usize >= map.len() || j + dir.1 as usize >= map[0].len() {
            perimeter += 1;
            continue;
        }
        if map[i + dir.0 as usize][j + dir.1 as usize] != plant {
            perimeter += 1;
        }
    }
    perimeter
}

fn main() {
    let map: Vec<Vec<char>> = include_str!("../input2.txt").lines().map(|line| line.chars().collect()).collect();
    let mut gardens: Vec<Garden> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            let mut garden = Garden { plant: *c, count: 1, perimeter: 0 };
            if find_garden(&map, i, j, &mut garden, &mut visited, &mut gardens) {
                gardens.push(garden);
            }
        }
    }

    let price: usize = gardens.iter().map(|g| g.count * g.perimeter).collect::<Vec<usize>>().iter().sum();
    println!("{}", price)
}
