use std::{fs, str::FromStr};

pub fn solution_part1() -> i32 {
    let input = fs::read_to_string("src/year2025/inputs/day01.txt").expect("couldn't read a file");
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut current = 50;
    let mut out = 0;
    for command in lines {
        current = increment_dial(current, command);
        if current == 0 {
            out += 1;
        }
    }

    return out;
}

pub fn solution_part2() -> i32 {
    let input = fs::read_to_string("src/year2025/inputs/day01.txt").expect("couldn't read a file");
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut current = 50;
    let mut out = 0;
    for command in lines {
        let answer = increment_dial_part2(current, command);
        current = answer.0;
        if current == 0 {
            out += 1;
        }

        out += answer.1;
    }

    return out;
}

// returns a new value for the dial
fn increment_dial(current: i32, command: &str) -> i32 {
    let dir = if command.get(0..1).unwrap() == "L" {
        -1
    } else {
        1
    };
    let dist = command.get(1..).unwrap();
    let dist = i32::from_str(dist).unwrap();

    let dial = 100;
    let a = current + dir * dist;
    let factor = a.div_euclid(dial);

    a - factor * dial
}

// returns a new value for the dial
fn increment_dial_part2(current: i32, command: &str) -> (i32, i32) {
    let dir = if command.get(0..1).unwrap() == "L" {
        -1
    } else {
        1
    };
    let dist = command.get(1..).unwrap();
    let dist = i32::from_str(dist).unwrap();

    let dial = 100;

    let a = current + dir * dist;

    let factor = a.div_euclid(dial);
    let answer = a - factor * dial;

    let mut count = factor.abs();
    if (current == 0 && dir == -1) || (answer == 0 && dir == 1) {
        count -= 1;
    }

    (answer, count)
}

mod test {
    use crate::year2025::day01::{increment_dial, increment_dial_part2};

    #[test]
    fn test_increment_dial() {
        let current = 11;
        let command = "R8";
        let new = increment_dial(current, command);

        assert_eq!(new, 19);

        let current = 19;
        let command = "L19";
        let new = increment_dial(current, command);

        assert_eq!(new, 0);

        let current = 0;
        let command = "L99";
        let new = increment_dial(current, command);

        assert_eq!(new, 0);

        let current = 0;
        let command = "R99";
        let new = increment_dial(current, command);

        assert_eq!(new, 99);
    }

    #[test]
    fn example_part1() {
        let inputs = [("L68", 82), ("L30", 52), ("R48", 0), ("L5", 95)];
        let mut current = 50;

        for (command, gt) in inputs {
            current = increment_dial(current, command);
            assert_eq!(current, gt);
        }
    }

    #[test]
    fn example_part2() {
        let inputs = [
            // ("L68", 82),
            // ("L30", 52),
            // ("R48", 0),
            // ("L5", 95),
            // ("R60", 55),
            // ("L55", 0),
            // ("L1", 99),
            // ("L99", 0),
            // ("R14", 14),
            // ("L82", 32),
            // ("L32", 0),
            // ("R50", 50),
            ("R1000", 50),
            // ("L50", 0),
        ];
        let mut current = 50;
        let mut out = 0;

        for (command, gt) in inputs {
            let answer = increment_dial_part2(current, command);
            current = answer.0;
            // if answer >
            // if answer.1 > 0 {
            //     out += answer.1;
            // }
            if current == 0 {
                out += 1;
            }

            out += answer.1;

            // if current == 0 {
            //     // out += 1;
            //     if answer.1 == 0 {
            //         out += 1;
            //     } else {
            //         out += answer.1;
            //     }
            // } else {
            //     out += answer.1;
            // }

            assert_eq!(current, gt);
        }

        assert_eq!(out, 10);
    }
}
