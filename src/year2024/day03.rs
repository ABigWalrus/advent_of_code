use regex::Regex;
use std::{fs, str::FromStr};

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day03").expect("couldn't read a file");
    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut out = 0;

    for line in lines {
        out += multiply(line);
    }

    return out;
}

fn multiply(line: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("Error by regex creation");
    let mut out = 0;
    let exprs = re.captures_iter(line);
    for reg in exprs {
        let expr: &str = &reg[0];
        let numbers = &expr[4..(expr.len() - 1)];
        let numbers: Vec<&str> = numbers.split(",").collect();
        let mul: i32 = i32::from_str(numbers[0]).expect("error by number")
            * i32::from_str(numbers[1]).expect("error by number");
        out += mul;
    }
    return out;
}

pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day03").expect("couldn't read a file");
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut out = 0;
    let mut active = true;
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").expect("Error by regex creation");
    for line in lines {
        // out += multiply_active(line, active);
        // let mut out = 0;
        // let mut active = true;
        let exprs = re.captures_iter(line);
        for reg in exprs {
            // println!("{:?}", reg);
            let expr: &str = &reg[0];
            if expr == "do()" {
                // println!("{expr:?}");
                active = true;
            } else if expr == "don't()" {
                // println!("{expr:?}");
                active = false;
            } else {
                if active {
                    let numbers = &expr[4..(expr.len() - 1)];
                    let numbers: Vec<&str> = numbers.split(",").collect();
                    let mul: i32 = i32::from_str(numbers[0]).expect("error by number")
                        * i32::from_str(numbers[1]).expect("error by number");
                    out += mul;
                }
            }
        }
        // return out;
    }

    // fn multiply_active(line: &str, active: &bool) -> i32 {}
    return out;
}
