use std::{env::args, fs::read_to_string, isize, usize};

const DIRS: [(isize, isize); 8] = [(0, 1), (1, 1), (1, 0), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

fn accessible(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if matrix[y][x] != '@' {
        return false;
    }
    let mut neighbours = 0;
    for dir in DIRS {
        let new_x = ((x as isize) + dir.0) as usize;
        let new_y = ((y as isize) + dir.1) as usize;
        if new_x >= matrix[0].len() || new_y >= matrix.len() {
            continue;
        }
        if matrix[new_y][new_x] == '@' {
            neighbours += 1;
        }
    }
    neighbours < 4
}

fn count_accessible(matrix: &Vec<Vec<char>>) -> usize {
    let mut counter = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if accessible(&matrix, j, i) {
                counter += 1;
            }
        }
    }
    counter
}

fn clean_up(matrix: &mut Vec<Vec<char>>) -> usize {
    let mut removed = 0;
    while count_accessible(matrix) != 0 {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if accessible(&matrix, j, i) {
                    matrix[i][j] = '.';
                    removed += 1;
                }
            }
        }
    }
    removed
}

fn main() {
    let args: Vec<String> = args().collect();
    let file = read_to_string(args[1].clone()).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    println!("{}", count_accessible(&matrix));
    println!("{}", clean_up(&mut matrix));
}
