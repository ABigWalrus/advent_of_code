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

fn main() {
    println!("The answer for the puzzle day01 is: {}", solution_day01_part2());
}
