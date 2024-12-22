use std::collections::BTreeMap;

fn main() {
    let secret_numbers: Vec<usize> = include_str!("../input2.txt").lines().into_iter().map(|l| l.parse().unwrap()).collect();

    let mut sum = 0;
    let mut diffs: Vec<Vec<isize>> = vec![Vec::new(); secret_numbers.len()];
    let mut digits: Vec<Vec<usize>> = vec![Vec::new(); secret_numbers.len()];
    for (i, n) in secret_numbers.iter().enumerate() {
        let mut secret_number = *n;
        let mut secret_digit: usize  = secret_number.to_string().chars().nth_back(0).unwrap().to_string().parse().unwrap();
        let mut prev_secret_digit = secret_digit;
        let mut diff;
        for _j in 0..2000 {
            let a = secret_number;
            let b = a * 64;
            let c = a ^ b; 
            let d = c % 16777216;

            let e = (d as f32 / 32_f32).floor().to_string().parse::<usize>().unwrap();
            let f = e ^ d;
            let g = f % 16777216;

            let h = g * 2048;
            let j = h ^ g;
            let k = j % 16777216;

            secret_number = k;
            secret_digit = secret_number.to_string().chars().nth_back(0).unwrap().to_string().parse().unwrap();
            diff = (secret_digit - prev_secret_digit) as isize;
            prev_secret_digit = secret_digit;

            diffs[i].push(diff);
            digits[i].push(secret_digit);

        }
        sum += secret_number;
    }
    println!("{sum}");


    let mut map: BTreeMap<Vec<isize>, usize> = BTreeMap::new();
    for i in 0..diffs.len() {
        for j in 0..diffs[i].len() - 3 {

            let sequence = vec![diffs[i][j], diffs[i][j + 1], diffs[i][j + 2], diffs[i][j + 3]];
            if map.contains_key(&sequence) { continue; }
            for k in 0..diffs.len() {
                for l in 0..diffs[k].len() - 3 {

                    if diffs[k][l] == sequence[0] && diffs[k][l + 1] == sequence[1] && diffs[k][l + 2] == sequence[2] && diffs[k][l + 3] == sequence[3] {
                        map.entry(sequence.clone()).and_modify(|v| *v += digits[k][l + 3]).or_insert(digits[k][l + 3]);
                        break;
                    }
                }
            }
        }
    }
    println!("{}", map.into_values().max().unwrap());
}
