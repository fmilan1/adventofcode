#[derive(Clone, Debug)]
struct Machine {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    prize_x: usize,
    prize_y: usize,
}

fn main() {
    let file = include_str!("../input2.txt");
    let mut machines: Vec<Machine> = Vec::new();
    for f in file.split("\n\n") {
        let mut x1 = 0;
        let mut y1 = 0;
        let mut x2 = 0;
        let mut y2 = 0;
        let mut prize_x = 0;
        let mut prize_y = 0;
        for i in 0..f.lines().count() {
            let line = f.lines().nth(i).unwrap();
            if i == 0 {
                x1 = line.split(", ").nth(0).unwrap().split("X+").nth(1).unwrap().parse().unwrap();
                y1 = line.split(", ").nth(1).unwrap().split("Y+").nth(1).unwrap().parse().unwrap();
            }
            else if i == 1 {
                x2 = line.split(", ").nth(0).unwrap().split("X+").nth(1).unwrap().parse().unwrap();
                y2 = line.split(", ").nth(1).unwrap().split("Y+").nth(1).unwrap().parse().unwrap();
            }
            else {
                prize_x = line.split(", ").nth(0).unwrap().split("X=").nth(1).unwrap().parse().unwrap();
                prize_y = line.split(", ").nth(1).unwrap().split("Y=").nth(1).unwrap().parse().unwrap();

            }
        }
        machines.push(Machine{x1, y1, x2, y2, prize_x, prize_y});
    }
    let mut tokens = 0;
    for m in &machines {
        let mut x: usize;
        let mut y: usize;
        let mut good = false;
        for i in 1..101 {
            if good { break; }
            x = i * m.x2;
            y = i * m.y2;
            for j in 1..101 {
                if x + j * m.x1 == m.prize_x && y + j * m.y1 == m.prize_y {
                    tokens += j * 3 + i;
                    good = true;
                    break;
                }
            }
        }
    }
    println!("{tokens}");
}
