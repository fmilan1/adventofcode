fn main() {
    let f = std::fs::read_to_string("input.txt").unwrap();
    // println!("{f}");
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    for a in f.split("\r\n") {
        let b = a.split("   ").collect::<Vec<_>>();
        first.push(b[0].parse().unwrap());
        second.push(b[1].parse().unwrap());
    }
    first.sort();
    second.sort();
    let mut c: i32 = 0;
    for i in 0..first.len() {
        c += (second[i] - first[i]).abs();
    }
    println!("{c}");

    let mut b = 0;
    for i in 0..first.len() {
        let mut a = 0;
        for j in 0..second.len() {
            if first[i] == second[j] {
                a += 1;
            }
        }
        b += first[i] * a;
    }
    println!("{b}");
}
