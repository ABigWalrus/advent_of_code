use std::fs::{self};
use std::str::FromStr;

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/input_day03").expect("Could't open a file");
    let lines:Vec<&str> = input.lines().collect();
    let n = lines[0].len();

    let down = 1;
    let right = 3;

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut trees = 0;
    for line in lines {

        // println!("{}", n);

        if &line[x..(x + 1)] == "#" {
            trees += 1;
        }

        x += right;
        y += down;

        if x >= n {
            x = x - n;
        } 
    }

    return trees;
} 


fn count_slope(_right: usize, _down:usize) -> i64 {
    let input = fs::read_to_string("src/input_day03").expect("Could't open a file");
    let lines:Vec<&str> = input.lines().collect();
    let n = lines[0].len();

    let down = _down;
    let right = _right;

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut trees = 0;
    for line in lines {

        // println!("{}", n);
        if y == 0 {
            if &line[x..(x + 1)] == "#" {
                trees += 1;
            }

            x += right;
            y += down;

            if x >= n {
                x = x - n;
            }
        }
        y -= 1;
    }

    return trees;
} 

pub fn solution_part2() -> i64{
    let slope01 = count_slope(1, 1);
    let slope02 = count_slope(3, 1);
    let slope03 = count_slope(5, 1);
    let slope04 = count_slope(7, 1);
    let slope05 = count_slope(1, 2);

    return slope01 * slope02 * slope03 * slope04 * slope05; 
}

