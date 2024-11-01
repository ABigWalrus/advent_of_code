use std::fs::{self};
use std::str::FromStr;

fn solution_day01() -> i32 {
    let expense_report:Vec<i32> = fs::read_to_string("src/input_day01")
        .expect("Could't open a file")
        .split_whitespace()
        .map(|x| i32::from_str(x).unwrap())
        .collect();
    let n = expense_report.len();
    let x = 1;
    for i in 0..n {
        let comp = 2020 - expense_report[i];
        for j in x..n {
            if expense_report[j] == comp {
                println!("Found the numbers: {} and {} !!!", expense_report[i], expense_report[j]);
                return expense_report[i] * expense_report[j]; 
            }
        }
    } 
    return 0;
}

fn solution_day01_part2() -> i32 {
    let expense_report:Vec<i32> = fs::read_to_string("src/input_day01")
        .expect("Could't open a file")
        .split_whitespace()
        .map(|x| i32::from_str(x).unwrap())
        .collect();
    let n = expense_report.len();
    for i in 0..(n - 2) {
        for j in i..(n - 1) {
            let comp = 2020 - expense_report[i] - expense_report[j];
            for g in j..n {
                if expense_report[g] == comp {
                    println!("Found the numbers: {}, {} and {} !!!", expense_report[i], expense_report[j], expense_report[g]);
                    return expense_report[i] * expense_report[j] * expense_report[g]; 
                }
            }
        }
    } 
    return 0;
}

fn is_password_valid(password: &str) -> bool {
    let mut iter = password.split_whitespace();
    let cond:Vec<&str> = iter.next().unwrap().split('-').collect();
    let (min_cond, max_cond) = (i32::from_str(cond[0]).unwrap(), i32::from_str(cond[1]).unwrap());
    let ch = char::from_str(iter.next().unwrap().trim_end_matches(':')).unwrap();
    let password = iter.next().unwrap();
    
    let mut n = 0;
    
    for c in password.chars() {
        if c == ch {
            n += 1;
            if n > max_cond {
                return false;
            }
        }
    }

    if n < min_cond {
        return false;
    }
    true
}

fn solution_day02() -> i32 {
    let input = fs::read_to_string("src/input_day02").expect("Could't open a file");

    let passwords:Vec<&str> = input.lines()
        .collect();
    let mut n = 0;
    for password in passwords {
        if is_password_valid(password) {
            n += 1;
        }
    }
    n
}

fn is_password_valid_part02(password: &str) -> bool {
    let mut iter = password.split_whitespace();
    let cond:Vec<&str> = iter.next().unwrap().split('-').collect();
    let (pos1, pos2) = (usize::from_str(cond[0]).unwrap(), usize::from_str(cond[1]).unwrap());
    let ch = char::from_str(iter.next().unwrap().trim_end_matches(':')).unwrap();
    let mut password: Vec<char> = iter.next().unwrap().chars().collect();
    
    let mut n = 0;
    if password[pos1 - 1] == ch {
        n +=1;
    } 
    if password[pos2 - 1] == ch {
        n +=1;
    } 
    if n == 1 { return true;}
    return false;
}

fn solution_day02_part02() -> i32 {
    let input = fs::read_to_string("src/input_day02").expect("Could't open a file");

    let passwords:Vec<&str> = input.lines()
        .collect();
    let mut n = 0;
    for password in passwords {
        if is_password_valid_part02(password) {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("The answer part 2 is: {}", solution_day02_part02());
}
