fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut orders: Vec<Vec<i32>> = Vec::new();
    let mut productions: Vec<Vec<i32>> = Vec::new();
    let mut is_orders = true;
    for line in file.lines() {
        if is_orders {
            if line == "" { is_orders = false; continue; }
            let numbers: Vec<&str> = line.split("|").collect();
            orders.push(vec![numbers[0].parse().unwrap(), numbers[1].parse().unwrap()]);
        }
        else {
            productions.push(line.split(",").map(|p| p.parse::<i32>().unwrap()).collect());
        }
    }
    let mut goods: Vec<Vec<i32>> = Vec::new();
    let mut bads: Vec<Vec<i32>> = Vec::new();
    for p in productions.iter() {
        let mut is_good = true;
        for i in 0..orders.len() {
            if p.contains(&orders[i][0]) && p.contains(&orders[i][1]) {
                let first = p.iter().position(|f: &i32| f == &orders[i][0]).unwrap();
                let second= p.iter().position(|f: &i32| f == &orders[i][1]).unwrap();
                if first > second { is_good = false; }
            }
            if is_good && i == orders.len() - 1 { goods.push(p.to_vec()); }
            else if !is_good && i == orders.len() - 1 { bads.push(p.to_vec()); }
        }
    }
    let mut sum1 = 0;
    for g in goods {
        sum1 += g[g.len() / 2];
    }
    println!("{sum1}");
    for b in bads.iter_mut() {
        'restart: loop {
            for i in 0..orders.len() {
                if b.contains(&orders[i][0]) && b.contains(&orders[i][1]) {
                    let first = b.iter().position(|f: &i32| f == &orders[i][0]).unwrap();
                    let second= b.iter().position(|f: &i32| f == &orders[i][1]).unwrap();
                    if first > second { 
                        let tmp = b[first];
                        b[first] = b[second];
                        b[second] = tmp;
                        continue 'restart;
                    }
                }
            }
            break;
        }
    }
    let mut sum2 = 0;
    for b in bads {
        sum2 += b[b.len() / 2];
    }
    println!("{sum2}");
}
