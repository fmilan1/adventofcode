use std::{collections::BTreeSet, env::args, fs::read_to_string, isize, usize};

fn count_neighbours_3d(actives: BTreeSet<(isize, isize, isize)>, x: isize, y: isize, z: isize) -> usize {
    let mut counter = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                let new_x = dx + x;
                let new_y = dy + y;
                let new_z = dz + z;
                if actives.contains(&(new_x, new_y, new_z)) {
                    counter += 1;
                }
            }
        }
    }

    counter
}

fn count_neighbours_4d(actives: BTreeSet<(isize, isize, isize, isize)>, x: isize, y: isize, z: isize, w: isize) -> usize {
    let mut counter = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                for dw in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        continue;
                    }
                    let new_x = dx + x;
                    let new_y = dy + y;
                    let new_z = dz + z;
                    let new_w = dw + w;
                    if actives.contains(&(new_x, new_y, new_z, new_w)) {
                        counter += 1;
                    }
                }
            }
        }
    }

    counter
}

fn main() {
    let args: Vec<String> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();

    let mut actives_3d = BTreeSet::<(isize, isize, isize)>::new();
    let mut actives_4d = BTreeSet::<(isize, isize, isize, isize)>::new();
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                actives_3d.insert((x as isize, y as isize, 0));
                actives_4d.insert((x as isize, y as isize, 0, 0));
            }
        }
    }

    for _i in 0..6 {
        let mut new_actives_3d = BTreeSet::<(isize, isize, isize)>::new();
        let mut new_actives_4d = BTreeSet::<(isize, isize, isize, isize)>::new();
        for cube in actives_3d.iter() {
            let neighbours_3d = count_neighbours_3d(actives_3d.clone(), cube.0, cube.1, cube.2);
            if neighbours_3d >= 2 && neighbours_3d <= 3 {
                new_actives_3d.insert(cube.clone());
            }
        }
        for cube in actives_4d.iter() {
            let neighbours_4d = count_neighbours_4d(actives_4d.clone(), cube.0, cube.1, cube.2, cube.3);
            if neighbours_4d >= 2 && neighbours_4d <= 3 {
                new_actives_4d.insert(cube.clone());
            }
        }
        let mut min_x_3d: isize = 0;
        let mut min_y_3d: isize = 0;
        let mut min_z_3d: isize = 0;
        let mut max_x_3d: isize = 0;
        let mut max_y_3d: isize = 0;
        let mut max_z_3d: isize = 0;
        for a in actives_3d.iter() {
            if a.0 < min_x_3d {
                min_x_3d = a.0;
            }
            if a.1 < min_y_3d {
                min_y_3d = a.1;
            }
            if a.2 < min_z_3d {
                min_z_3d = a.2;
            }

            if a.0 > max_x_3d {
                max_x_3d = a.0;
            }
            if a.1 > max_y_3d {
                max_y_3d = a.1;
            }
            if a.2 > max_z_3d {
                max_z_3d = a.2;
            }
        }
        let mut min_x_4d: isize = 0;
        let mut min_y_4d: isize = 0;
        let mut min_z_4d: isize = 0;
        let mut min_w_4d: isize = 0;
        let mut max_x_4d: isize = 0;
        let mut max_y_4d: isize = 0;
        let mut max_z_4d: isize = 0;
        let mut max_w_4d: isize = 0;
        for a in actives_4d.iter() {
            if a.0 < min_x_4d {
                min_x_4d = a.0;
            }
            if a.1 < min_y_4d {
                min_y_4d = a.1;
            }
            if a.2 < min_z_4d {
                min_z_4d = a.2;
            }
            if a.3 < min_w_4d {
                min_w_4d = a.3;
            }

            if a.0 > max_x_4d {
                max_x_4d = a.0;
            }
            if a.1 > max_y_4d {
                max_y_4d = a.1;
            }
            if a.2 > max_z_4d {
                max_z_4d = a.2;
            }
            if a.3 > max_w_4d {
                max_w_4d = a.3;
            }
        }
        for z in min_z_3d - 1..max_z_3d + 2 {
            for y in min_y_3d - 1..max_y_3d + 2 {
                for x in min_x_3d - 1..max_x_3d + 2 {
                    if !actives_3d.contains(&(x, y, z)) {
                        let neighbours = count_neighbours_3d(actives_3d.clone(), x, y, z);
                        if neighbours == 3 {
                            new_actives_3d.insert((x, y, z));
                        }
                    }
                }
            }
        }
        for w in min_w_4d - 1..max_w_4d + 2 {
            for z in min_z_4d - 1..max_z_4d + 2 {
                for y in min_y_4d - 1..max_y_4d + 2 {
                    for x in min_x_4d - 1..max_x_4d + 2 {
                        if !actives_4d.contains(&(x, y, z, w)) {
                            let neighbours = count_neighbours_4d(actives_4d.clone(), x, y, z, w);
                            if neighbours == 3 {
                                new_actives_4d.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        actives_3d = new_actives_3d;
        actives_4d = new_actives_4d;
    }
    println!("{}", actives_3d.len());
    println!("{}", actives_4d.len());
}
