use std::fs::{self};
use std::str::FromStr;



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

pub fn solution() -> i32 {
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

fn is_password_valid_part2(password: &str) -> bool {
    let mut iter = password.split_whitespace();
    let cond:Vec<&str> = iter.next().unwrap().split('-').collect();
    let (pos1, pos2) = (usize::from_str(cond[0]).unwrap(), usize::from_str(cond[1]).unwrap());
    let ch = char::from_str(iter.next().unwrap().trim_end_matches(':')).unwrap();
    let password: Vec<char> = iter.next().unwrap().chars().collect();
    
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

pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/input_day02").expect("Could't open a file");

    let passwords:Vec<&str> = input.lines()
        .collect();
    let mut n = 0;
    for password in passwords {
        if is_password_valid_part2(password) {
            n += 1;
        }
    }
    n
}
