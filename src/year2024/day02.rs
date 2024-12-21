use std::{fs, str::FromStr};

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day02").expect("couldn't read a file");
    //     let input = "7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9";

    let reports: Vec<&str> = input.trim().split("\n").collect();

    let mut safe: i32 = 0;

    for report in reports {
        let report: Vec<i32> = report
            .split_whitespace()
            .map(|rep| i32::from_str(rep).expect("error by parsing int from string"))
            .collect();
        if is_report_safe(&report) {
            safe += 1;
        }
    }

    return safe;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut t: i32;
    let mut prev: i32 = report[0] - report[1];

    for i in 0..(report.len() - 1) {
        t = report[i] - report[i + 1];
        if t.abs() < 1 || t.abs() > 3 {
            // println!("{:?}", report);
            // println!("{}, {}, {}, {}", report[i], report[i + 1], t, prev);

            return false;
        }
        if t * prev < 1 {
            // println!("{:?}", report);
            // println!("{}, {}, {}, {}", report[i], report[i + 1], t, prev);
            return false;
        }
        prev = t;
    }

    // println!("{:?}", report);
    return true;
}

pub fn solution_part2() -> i32 {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let input = fs::read_to_string("src/year2024/data/input_day02").expect("couldn't read a file");
    let reports: Vec<&str> = input.trim().split("\n").collect();

    let mut safe: i32 = 0;

    for report in reports {
        let report: Vec<i32> = report
            .split_whitespace()
            .map(|rep| i32::from_str(rep).expect("error by parsing int from string"))
            .collect();
        if is_report_super_safe(report) {
            safe += 1;
        }
    }

    return safe;
}

fn is_report_super_safe(report: Vec<i32>) -> bool {
    let mut t: i32;
    let mut prev: i32 = report[0] - report[1];

    let mut out = false;

    if is_report_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let temp_report = [&report[..i], &report[i + 1..]].concat();
        if is_report_safe(&temp_report) {
            return true;
        }
    }

    // println!("{:?}", report);
    return false;
}
