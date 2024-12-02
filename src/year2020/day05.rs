use std::fs;

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/year2020/input_day05").expect("Couldn't read a file");
    let codes: Vec<&str> = input.split_whitespace().collect();

    let mut highest_id: i32 = 0;

    // let code01 = "BFFFBBFRRR";
    // let code02 = "FFFBBBFRRR";
    // let code03 = "BBFFBBFRLL";

    //let codes = vec![code01, code02, code03];

    for code in codes {
        let temp = get_seat_id(code);
        if highest_id < temp {
            highest_id = temp;
        }
    }

    return highest_id;
}

fn get_seat_id(code: &str) -> i32 {
    let mut id = 0;

    let n_col = 7;
    // let n_row = 3;

    let row_code = &code[..n_col];
    let col_code = &code[n_col..];

    let mut low = 0;
    let mut high = 128;
    for r in row_code.chars() {
        match r {
            'B' => {
                low = (high + low) / 2;
            }
            'F' => {
                high = (high + low) / 2;
            }
            _ => {
                println!("unknown char");
            }
        }
    }

    id = low;

    let mut low = 0;
    let mut high = 8;
    for r in col_code.chars() {
        match r {
            'R' => {
                low = (high + low) / 2;
            }
            'L' => {
                high = (high + low) / 2;
            }
            _ => {
                println!("unknown char");
            }
        }
    }
    id = id * 8 + low;
    // println!("The seat id is: {}", id);

    return id;
}

pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/year2020/input_day05").expect("Couldn't read a file");
    let codes: Vec<&str> = input.split_whitespace().collect();

    // let id = 0;
    let mut ids: Vec<i32> = vec![];

    for code in codes {
        ids.push(get_seat_id(code));
    }

    ids.sort();

    let mut prev = ids[0];

    for id in ids {
        if (id - prev) > 1 {
            return id - 1;
        }
        prev = id;
    }

    return 0;
}
