#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    vel_x: isize,
    vel_y: isize,
}

fn main() {
    let file = include_str!("../input2.txt");
    let mut robots: Vec<Robot> = Vec::new();
    for line in file.lines() {
        let arr: Vec<&str> = line.split(" ").collect();
        let x= arr[0].split(",").nth(0).unwrap().split("p=").nth(1).unwrap().parse().unwrap();
        let y= arr[0].split(",").nth(1).unwrap().parse().unwrap();
        let vel_x= arr[1].split(",").nth(0).unwrap().split("v=").nth(1).unwrap().parse().unwrap();
        let vel_y= arr[1].split(",").nth(1).unwrap().parse().unwrap();

        robots.push(Robot{x, y, vel_x, vel_y});
    }
    let tall: usize = 103;
    let wide: usize = 101;
    for _i in 0..100 {
        for r in robots.iter_mut() {
            let x = (r.x as isize + r.vel_x) % wide as isize;
            let y = (r.y as isize + r.vel_y) % tall as isize;
            r.x = if x < 0 {(wide as isize + x) as usize} else {x as usize};
            r.y = if y < 0 {(tall as isize + y) as usize} else {y as usize};
        }
    }

    let mut tmp = vec![vec![0; wide as usize]; tall as usize];
    for r in robots {
        tmp[r.y][r.x] += 1;
    }
    let mut counts = vec![0; 4];
    for i in 0..tall / 2 {
        for j in 0..wide / 2 {
            counts[0] += tmp[i][j];
        }
    }
    for i in tall / 2 + 1..tall {
        for j in 0..wide / 2 {
            counts[1] += tmp[i][j];
        }
    }
    for i in 0..tall / 2 {
        for j in wide / 2 + 1..wide {
            counts[2] += tmp[i][j];
        }
    }
    for i in tall / 2 + 1..tall {
        for j in wide / 2 + 1..wide {
            counts[3] += tmp[i][j];
        }
    }
    println!("{}", counts[0] * counts[1] * counts[2] * counts[3]);
}
