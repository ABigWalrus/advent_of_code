use std::{fs, str::FromStr};


pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/input_day08").expect("couldnt read a file");
    let cmds: Vec<&str> = input.split("\n").collect();
    let mut acc:i32 = 0;
    let n = cmds.len();
    let mut executed = vec![0; n];
    let mut i = 0;

    loop {
        if executed[i] == 1 {
            break;
        } else {
            executed[i] += 1;
        }

        let cmd:Vec<&str> = cmds[i].split_whitespace().collect();
        
        println!("{}: {}", cmd[0], cmd[1]);
        match cmd[0] {
            "jmp" => {
                let val = i32::from_str(cmd[1]).expect("error by conversion from str to usize");
                if val < 0 {
                    i -= val.wrapping_abs() as u32 as usize
                } else {
                    i += val as usize;
                }
            }, 
            "nop" => {
                i += 1;
            },
            "acc" => {
                let val = i32::from_str(cmd[1]).expect("error by conversion from str to usize");
acc += val;
                i += 1;
            },
            _ => {
                println!("unexpected command");
            }
        }
    }


    return acc;
}

pub fn solution_part2() -> i32 {
    return 0;
}

