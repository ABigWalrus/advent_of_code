use std::{fs, str::FromStr};

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day01").expect("couldn't read a file");
    //     let input = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";

    let locs: Vec<&str> = input.trim().split("\n").collect();

    let mut out: i32 = 0;

    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];

    for loc in locs {
        let pos: Vec<&str> = loc.split_whitespace().collect();
        let val = i32::from_str(pos[0]).expect("error by converting string to u32");
        l1.push(val);
        let val = i32::from_str(pos[1]).expect("error by converting string to u32");
        l2.push(val);
    }

    l1.sort();
    l2.sort();
    // println!("{:?}", l1);
    // println!("{:?}", l2);

    // println!("{:#?}", locs);
    for i in 0..l1.len() {
        let x = l1[i] - l2[i];
        out += x.abs();
    }
    return out;
}

pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/year2024/data/input_day01").expect("couldn't read a file");
    //     let input = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";

    let locs: Vec<&str> = input.trim().split("\n").collect();

    let mut similarity_score: i32 = 0;

    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];

    for loc in locs {
        let pos: Vec<&str> = loc.split_whitespace().collect();
        let val = i32::from_str(pos[0]).expect("error by converting string to u32");
        l1.push(val);
        let val = i32::from_str(pos[1]).expect("error by converting string to u32");
        l2.push(val);
    }

    for id1 in l1 {
        let mut n = 0;
        for id2 in &l2 {
            if id1 == *id2 {
                n += 1;
            }
        }

        similarity_score += id1 * n;
    }
    return similarity_score;
}
