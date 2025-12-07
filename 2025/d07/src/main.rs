use std::{env::args, fs::read_to_string, usize};

fn count(matrix: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if y >= matrix.len() || matrix[y][x] == '|' {
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

fn do_timelines(matrix: &mut Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    if y >= matrix.len() {
        return 1;
    }
    if matrix[y][x] <= 2 && matrix[y][x] != 0 {
        let left = do_timelines(matrix, x - 1, y);
        let right = do_timelines(matrix, x + 1, y);
        matrix[y][x] = left + right;
        return left + right;
    } else if matrix[y][x] > 2 {
        return matrix[y][x];
    }
    return do_timelines(matrix, x, y + 1);
}

fn main() {
    let file = read_to_string(args().collect::<Vec<String>>()[1].clone()).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut matrix2: Vec<Vec<usize>> = Vec::new();
    for line in file.lines() {
        matrix.push(line.chars().collect());
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            if c == '^' {
                row.push(2);
            } else {
                row.push(0);
            }
        }
        matrix2.push(row);
    }
    let start = file.lines().nth(0).unwrap().chars().position(|c| c == 'S').unwrap();
    let count = count(&mut matrix.clone(), start, 1);
    println!("{}", count);

    do_timelines(&mut matrix2, start, 1);
    println!("{}", matrix2[2][start]);
}
