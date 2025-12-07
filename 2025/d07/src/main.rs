use std::{env::args, fs::read_to_string};

fn count(matrix: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if y >= matrix.len() || x >= matrix[0].len() || matrix[y][x] == '|' {
        return 0;
    }
    if matrix[y][x] == '^' {
        let left = count(matrix, x - 1, y);
        let right = count(matrix, x + 1, y);
        return 1 + left + right;
    }
    matrix[y][x] = '|';
    return count(matrix, x, y + 1);
}

fn main() {
    let file = read_to_string(args().collect::<Vec<String>>()[1].clone()).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        matrix.push(line.chars().collect());
    }
    let start = file.lines().nth(0).unwrap().chars().position(|c| c == 'S').unwrap();
    let count = count(&mut matrix.clone(), start, 1);
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            print!("{}", matrix[y][x]);
        }
        println!();
    }
    println!("{}", count);
}
