use itertools::Itertools;

fn main() {
    let file = include_str!("../input1.txt");
    let mut avail: Vec<String> = file.split("\n\n").nth(0).unwrap().split(", ").map(|a| a.to_string()).collect();
    // avail.sort_by_key(|a| -(a.len() as isize));

    dbg!(&avail.len());
    for i in 0..1 {
        for a in avail.clone().iter() {
            avail.push(a.clone());
        }
    }
    dbg!(&avail.len());
    // let mut perms: Vec<Vec<&String>> = ;
    // dbg!(&avail);


    let designs: Vec<String> = file.split("\n\n").nth(1).unwrap().lines().map(|a| a.to_string()).collect();

    let mut possibles: Vec<String> = Vec::new();
    for design in designs.iter() {
        for perm in avail.iter().permutations(avail.len()).unique() {
            // dbg!(&perm.into_iter().join(""));
            let tmp_perm = perm.into_iter().join("");
            if tmp_perm.contains(design) {
                possibles.push(design.clone());
                break;
            }
        }
    }

    dbg!(&possibles);
    println!("{}", possibles.len());

    // 339 low
    // 340 not good idk which way
    // 352 not good idk which way
}
