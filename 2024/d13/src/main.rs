use nalgebra::{Matrix2, Vector2};

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

    let mut n: usize = 0;
    for _i in 0..2 {
        let mut tokens = 0;
        for machine in machines.iter_mut() {
            machine.prize_x += n;
            machine.prize_y += n;
            let m = Matrix2::new(machine.x1 as f64, machine.x2 as f64, machine.y1 as f64, machine.y2 as f64);
            let r = Vector2::new(machine.prize_x as f64, machine.prize_y as f64);
            if let Some(inverse) = m.try_inverse() {
                let mult = inverse * r;
                let tolerance = 1e-3;
                if (mult[0] - mult[0].round()).abs() > tolerance || (mult[1] - mult[1].round()).abs() > tolerance { continue; }
                tokens += mult[0].round() as usize * 3 + mult[1].round() as usize;
            }
        }
        println!("{tokens}");
        n = 10000000000000;
    }
}
