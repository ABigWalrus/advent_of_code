use std::{fs, str::FromStr};

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day04").expect("couldn't read a file");
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut matrix: Vec<Vec<char>> = vec![];
    let mut out = 0;
    for line in lines {
        let mut chars: Vec<char> = vec![];
        for char in line.chars() {
            chars.push(char);
        }
        matrix.push(chars);
    }
    let row = matrix.len();
    let col = matrix[4].len();

    for i in 0..row {
        for j in 0..col {
            if matrix[i][j] == 'X' {
                if go_in_direction(&matrix, -1, -1, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, -1, 0, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, -1, 1, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, 0, -1, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, 0, 1, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, 1, -1, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, 1, 0, i, j) {
                    out += 1;
                }
                if go_in_direction(&matrix, 1, 1, i, j) {
                    out += 1;
                }
            }
        }
    }
    return out;
}

fn in_boundries(x: i32, y: i32) -> bool {
    let col = 140;
    let row = 140;
    if y < 0 || y >= row as i32 {
        return false;
    }
    if x < 0 || x >= col as i32 {
        return false;
    }
    return true;
}

fn go_in_direction(matrix: &Vec<Vec<char>>, x: i32, y: i32, i: usize, j: usize) -> bool {
    let christ = ['M', 'A', 'S'];

    for k in 1..4 {
        if !in_boundries(i as i32 + k * x, j as i32 + k * y) {
            return false;
        }
        if matrix[(i as i32 + k * x) as usize][(j as i32 + k * y) as usize]
            != christ[(k - 1) as usize]
        {
            return false;
        }
    }

    return true;
}

pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day04").expect("couldn't read a file");
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut matrix: Vec<Vec<char>> = vec![];
    let mut out = 0;
    for line in lines {
        let mut chars: Vec<char> = vec![];
        for char in line.chars() {
            chars.push(char);
        }
        matrix.push(chars);
    }
    let row = matrix.len();
    let col = matrix[4].len();

    for i in 0..row {
        for j in 0..col {
            if matrix[i][j] == 'A' {
                if in_boundaries02(i, j) && check_x(&matrix, i, j) {
                    out += 1;
                }
            }
        }
    }
    return out;
}

fn check_x(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut out = 0;
    // fisrt diagonale
    if matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S' {
        out += 1;
    } else if matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M' {
        out += 1;
    }

    // second diagonale
    if matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S' {
        out += 1;
    } else if matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M' {
        out += 1;
    }

    if out == 2 {
        return true;
    } else {
        return false;
    }
}

fn in_boundaries02(i: usize, j: usize) -> bool {
    if i <= 0 || i >= 139 {
        return false;
    }
    if j <= 0 || j >= 139 {
        return false;
    }
    return true;
}
