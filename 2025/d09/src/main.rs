use std::{env::args, fs::read_to_string, isize};

fn area(a: (isize, isize), b: (isize, isize)) -> isize {
    let dist_x = if a.0 > b.0 { a.0 - b.0 + 1 } else { b.0 - a.0 + 1 };
    let dist_y = if a.1 > b.1 { a.1 - b.1 + 1 } else { b.1 - a.1 + 1 };
    return dist_x * dist_y;
}

fn is_in_shape(reds: &Vec<(isize, isize)>, p: (isize, isize)) -> bool {
    let mut tmp = reds.clone();
    tmp.push(reds[0]);
    let mut counter = 0;
    for i in 0..tmp.len() - 1 {
        let r1 = tmp[i];
        let r2 = tmp[i + 1];
        let mut asd = vec![r1, r2];
        asd.sort_by_key(|r| r.0);
        let min_x = asd[0].0;
        asd.sort_by_key(|r| -r.0);
        let max_x = asd[0].0;
        asd.sort_by_key(|r| r.1);
        let min_y = asd[0].1;
        asd.sort_by_key(|r| -r.1);
        let max_y = asd[0].1;
        if r1.0 == r2.0 {
            if p.0 == r1.0 && p.1 <= max_y && p.1 >= min_y {
                return true;
            }
            if p.1 >= min_y && p.1 <= max_y && p.0 <= r1.0 {
                counter += 1;
            }
        } else if r1.1 == r2.1 {
            if p.1 == r1.1 && p.0 <= max_x && p.0 >= min_x {
                return true;
            }
        }
    }
    counter % 2 != 0
}

fn is_on_edge_of_shape(shape: &Vec<(isize, isize)>, p: (isize, isize)) -> bool {
    let mut tmp = shape.clone();
    tmp.push(shape[0]);
    for i in 0..tmp.len() - 1 {
        if is_in_shape(&vec![tmp[i], tmp[i + 1]], p) {
            return true;
        }
    }
    false
}

fn main() {
    let file = read_to_string(args().collect::<Vec<String>>()[1].clone()).unwrap();
    let mut reds: Vec<(isize, isize)> = Vec::new();
    for line in file.lines() {
        let (x, y) = line.split_once(",").map(|s| (s.0.parse::<isize>().unwrap(), s.1.parse::<isize>().unwrap())).unwrap();
        reds.push((x, y));
    }

    let mut max_distance = 0;
    let mut a = (0, 0);
    let mut b = (0, 0);
    for i in 0..reds.len() {
        for j in i + 1..reds.len() {
            let new_max = (reds[i].0 - reds[j].0).pow(2) + (reds[i].1 - reds[j].1).pow(2);
            if new_max > max_distance {
                max_distance = new_max;
                a = reds[i];
                b = reds[j];
            }
        }
    }
    println!("{}", area(a, b));
    let mut max_distance2 = 0;
    let mut a2 = (0, 0);
    let mut b2 = (0, 0);

    for i in 0..reds.len() {
        for j in i + 1..reds.len() {
            let new_max = (reds[i].0 - reds[j].0).pow(2) + (reds[i].1 - reds[j].1).pow(2);
            if new_max > max_distance2 {
                let c1 = (reds[i].0, reds[i].1);
                let c2 = (reds[j].0, reds[j].1);

                let mut diag_vector = ((c2.0 - c1.0) as f32, (c2.1 - c1.1) as f32);
                diag_vector = (diag_vector.0 / 2.0, diag_vector.1 / 2.0);
                let m = ((c1.0 + c2.0) as f32 / 2.0, (c1.1 + c2.1) as f32 / 2.0);
                let c3 = ((m.0 + diag_vector.0) as isize, (m.1 - diag_vector.1) as isize);
                let c4 = ((m.0 - diag_vector.0) as isize, (m.1 + diag_vector.1) as isize);
                // let c3 = (c2.1, c1.1);
                // let c4 = (c1.0, c2.1);
                // dbg!(c1, c3, c4, c2);
                let mut good = true;
                let rectangle = vec![
                    (c1.0 as isize, c1.1 as isize),
                    (c2.0 as isize, c2.1 as isize),
                    (c3.0 as isize, c3.1 as isize),
                    (c4.0 as isize, c4.1 as isize),
                ];
                if is_in_shape(&reds, c1) && is_in_shape(&reds, c3) && is_in_shape(&reds, c4) && is_in_shape(&reds, c2) {
                    let mut counter = 0;
                    for r in reds.iter() {
                        if rectangle.contains(r) {
                            counter += 1;
                        }
                        if !rectangle.contains(r) && !is_on_edge_of_shape(&rectangle, *r) && is_in_shape(&rectangle, *r) {
                            break;
                        }
                    }
                    if counter == 3 {
                        max_distance2 = new_max;
                        a2 = reds[i];
                        b2 = reds[j];
                        println!("{:?} -> {}", rectangle, area(a2, b2));
                    }
                }
            }
        }
    }
    // dbg!(a2, b2);
    println!("{}", area(a2, b2));
    // let asd = do_segments_intersect(&reds, 2, 1, 11, 1, 6, 1, 6, 3);
    // println!("{:?}", asd);
    // 64774930 low
    // 159738075 idk
    // 4364487550 high
}
