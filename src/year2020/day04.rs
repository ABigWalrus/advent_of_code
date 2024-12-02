use std::fs;
use std::str::FromStr;

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/input_day04").expect("Could't open a file");
    let passports:Vec<&str> = input.split("\n\n").collect();
    
    let mut valid = 0;

    for passport in passports {
        if is_valid(passport) { valid += 1; }
    }

    return valid;
} 

fn is_valid(passport: &str) -> bool{
    let fields:Vec<&str> = passport.split_whitespace().collect();
    let mut n = 0;

    for field in fields {
        let field_paras:Vec<&str> = field.split(":").collect();

        let field_name = field_paras[0];
         
        if field_name != "cid" { n += 1; }
    }

    if n == 7 { return true; }

    return false;
}


pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/input_day04").expect("Could't open a file");
    let passports:Vec<&str> = input.split("\n\n").collect();
    
    let mut valid = 0;

    for passport in passports {
        if is_super_valid(passport) { valid += 1; }
    }

    return valid;
} 

fn is_super_valid(passport: &str) -> bool{
    let fields:Vec<&str> = passport.split_whitespace().collect();
    let mut n = 0;

    for field in fields {
        let field_paras:Vec<&str> = field.split(":").collect();
        let field_name = field_paras[0];
        let field_value = field_paras[1];         
        match field_name {
            "byr" => {
                n += 1;
                if !byr_is_valid(field_value) { return false; }
            },
            "iyr" => {
                n += 1;
                if !iyr_is_valid(field_value) { return false; }
            },
            "eyr" => {
                n += 1;
                if !eyr_is_valid(field_value) { return false; }
            },
            "hgt" => {
                n += 1;
                if !hgt_is_valid(field_value) { return false; }
            },
            "hcl" => {
                n += 1;
                if !hcl_is_valid(field_value) { return false; }
            },
            "ecl" => {
                n += 1;
                if !ecl_is_valid(field_value) { return false; }
            },
            "pid" => {
                n += 1;
                if !pid_is_valid(field_value) { return false; }
            },
            "cid" => {},
            _ => { println!("unknown field name"); }
        }
    }

    if n == 7 { return true; }

    return false;
}

fn byr_is_valid(value: &str) -> bool {
    if value.len() == 4 {
        match i32::from_str(value) {
            Ok(x) => {
                if (x >= 1920) && (x <= 2002) { return true; }
            },
            Err(_) => {}
        }
    }
    return false;
}
fn iyr_is_valid(value: &str) -> bool {
    if value.len() == 4 {
        match i32::from_str(value) {
            Ok(x) => {
                if (x >= 2010) && (x <= 2020) { return true; }
            },
            Err(_) => {}
        }
    }
    return false;
}
fn eyr_is_valid(value: &str) -> bool {
    if value.len() == 4 {
        match i32::from_str(value) {
            Ok(x) => {
                if (x >= 2020) && (x <= 2030) { return true; }
            },
            Err(_) => {}
        }
    }
    return false;
}
fn hgt_is_valid(value: &str) -> bool {
    let n = value.len();   
    if n >= 4 {
        let unit = &value[(n-2)..n];

        match unit {
            "cm" => {
                match i32::from_str(&value[..(n-2)]) {
                    Ok(x) => {
                        if (x >= 150) && (x <= 193) {
                            return true;
                        }
                    },
                    Err(_) => {}
                }
            },
            "in" => {
                match i32::from_str(&value[..(n-2)]) {
                    Ok(x) => {
                        if (x >= 59) && (x <= 76) {
                            return true;
                        }
                    },
                    Err(_) => {}
                }
            },
            _ => {}
        }
    } 
    return false;
}
fn hcl_is_valid(value: &str) -> bool {
    if value.len() == 7 {
        if &value[0..1] == "#" {
            for x in value[1..].as_bytes() {
                if (*x < 48) || ((*x > 57) && (*x < 97)) || (*x > 102) {
                    return false;
                }
            }
            return true;
        }
    }
    return false;
}
fn ecl_is_valid(value: &str) -> bool {
    match value {
        "amb" => { return true; },
        "blu" => { return true; },
        "brn" => { return true; },
        "gry" => { return true; },
        "grn" => { return true; },
        "hzl" => { return true; },
        "oth" => { return true; },
        _ => {}
    }
    return false;
}
fn pid_is_valid(value: &str) -> bool {
    if value.len() == 9 {
        match i32::from_str(value) {
            Ok(x) => {
               return true;
            },
            Err(_) => {}
        }
    }
    return false;
}
