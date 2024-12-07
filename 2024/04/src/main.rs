fn main() {
    let file: String = std::fs::read_to_string("input.txt").unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for a in file.lines() {
        matrix.push(a.chars().collect());
    }
    let mut count1 = 0;
    let mut count2 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if j < matrix[i].len() - 3
                && matrix[i][j] == 'X'
                && matrix[i][j + 1] == 'M'
                && matrix[i][j + 2] == 'A'
                && matrix[i][j + 3] == 'S'
            {
                count1 += 1;
            }
            if j < matrix[i].len() - 3
                && matrix[i][j] == 'S'
                && matrix[i][j + 1] == 'A'
                && matrix[i][j + 2] == 'M'
                && matrix[i][j + 3] == 'X'
            {
                count1 += 1;
            }
            if i < matrix.len() - 3
                && matrix[i][j] == 'X'
                && matrix[i + 1][j] == 'M'
                && matrix[i + 2][j] == 'A'
                && matrix[i + 3][j] == 'S'
            {
                count1 += 1;
            }
            if i < matrix.len() - 3
                && matrix[i][j] == 'S'
                && matrix[i + 1][j] == 'A'
                && matrix[i + 2][j] == 'M'
                && matrix[i + 3][j] == 'X'
            {
                count1 += 1;
            }
            if i < matrix.len() - 3
                && j < matrix[i].len() - 3
                && matrix[i][j] == 'X'
                && matrix[i + 1][j + 1] == 'M'
                && matrix[i + 2][j + 2] == 'A'
                && matrix[i + 3][j + 3] == 'S'
            {
                count1 += 1;
            }
            if i < matrix.len() - 3
                && j < matrix[i].len() - 3
                && matrix[i][j] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j + 2] == 'M'
                && matrix[i + 3][j + 3] == 'X'
            {
                count1 += 1;
            }
            if i < matrix.len() - 3
                && j >= 3
                && matrix[i][j] == 'X'
                && matrix[i + 1][j - 1] == 'M'
                && matrix[i + 2][j - 2] == 'A'
                && matrix[i + 3][j - 3] == 'S'
            {
                count1 += 1;
            }
            if i < matrix.len() - 3
                && j >= 3
                && matrix[i][j] == 'S'
                && matrix[i + 1][j - 1] == 'A'
                && matrix[i + 2][j - 2] == 'M'
                && matrix[i + 3][j - 3] == 'X'
            {
                count1 += 1;
            }

            if i >= 1 && i < matrix.len() - 1 && j >= 1 && j < matrix[i].len() - 1 {
                if matrix[i][j] == 'A' {
                    let vec: Vec<char> = vec![
                        matrix[i - 1][j - 1],
                        matrix[i - 1][j + 1],
                        matrix[i + 1][j - 1],
                        matrix[i + 1][j + 1],
                    ];
                    let mut m = 0;
                    let mut s = 0;
                    for v in vec.iter() {
                        if *v == 'M' {
                            m += 1;
                        } else if *v == 'S' {
                            s += 1;
                        }
                    }
                    if vec[0] != vec[3] && m == 2 && s == 2 {
                        count2 += 1;
                    }
                }
            }
        }
    }
    println!("{count1}");
    println!("{count2}");
}
