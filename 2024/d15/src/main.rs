#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    dir: (isize, isize),
}

fn print(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

fn move_object(map: &mut Vec<Vec<char>>, x: usize, y:usize, dir: (isize, isize), robot: &mut Robot) -> bool {
    if map[y][x] == '#' { return false; }
    if map[y][x] == '.' { return true; }

    if move_object(map, (x as isize + dir.1) as usize, (y as isize + dir.0) as usize, dir, robot) {
        map[(y as isize + dir.0) as usize][(x as isize + dir.1) as usize] = map[y][x];
        map[y][x] = '.';
        if robot.x == x && robot.y == y {
            robot.x = (robot.x as isize + dir.1) as usize;
            robot.y = (robot.y as isize + dir.0) as usize;
        }
    }
    if map[y][x] == '.' { return true; }

    false
    // true
}

fn move_object2(map: &mut Vec<Vec<char>>, x: usize, y:usize, dir: (isize, isize), robot: &mut Robot) -> bool {
                   // print(map);
    if map[y][x] == '#' { return false; }
    // if map[y][x] == '.' { return true; }
    
    if dir == (-1, 0) || dir == (1, 0) {
        // if map[y][x] == '.' { return true; }
        // let d: isize = if map[(y as isize + dir.0) as usize][x] == '[' {1} else if map[(y as isize + dir.0) as usize][x] == ']' {-1} else {0};
        // dbg!(x, y, d);
        //     dbg!("itten na", d);
        // if d == 0 {
        //     if map[y][x] == '@' {

        //         move_object(map, x, y, dir, robot);
        //     }
        //     // else if map[y][x] == '.' { return true; }
        //     // else { return true; }
        // }
            let d: isize = if map[(y as isize + dir.0) as usize][x] == '[' {1} else if map[(y as isize + dir.0) as usize][x] == ']' {-1} else {0};
        if map[y][x] == ']' || map[y][x] == '[' {

        }
        else if move_object2(map, x, (y as isize + dir.0) as usize, dir, robot) && move_object2(map, (x as isize + d) as usize, (y as isize + dir.0) as usize, dir, robot) {
               map[(y as isize + dir.0) as usize][x] = map[y][x];
               map[(y as isize + dir.0) as usize][(x as isize + d) as usize] = map[y][(x as isize + d) as usize];
               map[y][x] = '.';
               map[y][(x as isize + d) as usize] = '.';

        }
    }
    else {
        move_object(map, x, y, dir, robot);
    }
    // }
    if map[y][x] == '.' { return true; }

    true
}

fn main() {
    let file = include_str!("../input1.txt");
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut map2: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<(isize, isize)> = Vec::new();
    let arr: Vec<&str> = file.split("\n\n").collect();
    let mut robot: Robot = Robot{x: 0, y: 0, dir: (0, 0)};
    for (i, line) in arr[0].lines().enumerate() {
        let mut tmp: Vec<char> = Vec::new();
        let mut tmp2: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == '#' { tmp2.push('#'); tmp2.push('#'); }
            else if c == 'O' { tmp2.push('['); tmp2.push(']'); }
            else if c == '@' { tmp2.push('@'); tmp2.push('.');robot = Robot{x: j, y: i, dir: robot.dir}; }
            else { tmp2.push('.'); tmp2.push('.'); }
            tmp.push(c);
        }
        map.push(tmp);
        map2.push(tmp2);
    }
    let mut robot2: Robot = Robot{x: robot.x * 2, y: robot.y, dir: (0, 0)};
    for line in arr[1].lines() {
        for c in line.chars() {
            let dir;
            if c == '^' { dir = (-1, 0); }
            else if c == 'v' { dir = (1, 0); }
            else if c == '<' { dir = (0, -1); }
            else { dir = (0, 1); }
            moves.push(dir);
        }
    }
    for m in &moves {
        robot.dir = *m;
        move_object(&mut map, robot.x, robot.y, robot.dir, &mut robot);
    }
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                sum += i * 100 + j;
            }
        }
    }
    println!("{sum}");
    print(&map2);
    println!();
    println!("{:?}", robot2);
    for m in &moves {
        robot2.dir = *m;
        println!("{:?}", robot2.dir);
        move_object2(&mut map2, robot2.x, robot2.y, robot2.dir, &mut robot2);
        print(&map2);
        println!();
    }
}
