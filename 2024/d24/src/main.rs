use std::collections::BTreeMap;

fn main() {
    let file = include_str!("../input2.txt");
    let mut wires: BTreeMap<&str, bool> = BTreeMap::new();
    for line in file.split("\n\n").nth(0).unwrap().lines() {

        let arr = line.split(": ").collect::<Vec<&str>>();
        wires.insert(arr[0], if arr[1] == "1" {true} else {false} );
    }

    let mut i = 0;
    let lines = file.split("\n\n").nth(1).unwrap().lines().collect::<Vec<&str>>();
    let mut checked: Vec<usize> = Vec::new();
    while i < lines.len() {
        if checked.contains(&i) {
            i += 1;
            continue;
        }
        let arr = lines[i].split(" ").collect::<Vec<&str>>();

        if wires.contains_key(arr[0]) && wires.contains_key(arr[2]) {
            let a = wires[arr[0]];
            let b = wires[arr[2]];
            if arr.contains(&"AND") {
                wires.entry(arr[4]).and_modify(|v| *v = a && b).or_insert(a && b);
            }
            else if arr.contains(&"OR") {
                wires.entry(arr[4]).and_modify(|v| *v = a || b).or_insert(a || b);
            }
            else if arr.contains(&"XOR") {
                wires.entry(arr[4]).and_modify(|v| *v = a != b).or_insert(a != b);
            }
            checked.push(i);
            i = 0;
        }
        else {
            i += 1;
        }
    }

    let keys = wires.keys().filter(|k| k.starts_with("z")).map(|k| *k).collect::<Vec<&str>>();
    let mut dec = 0;
    for (i, key) in keys.iter().enumerate() {
        dec += 2_usize.pow(i as u32) * if wires[key] {1} else {0};
    }
    println!("{dec}");
}
