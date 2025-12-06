use std::ops::RangeInclusive;
use std::{fs, str::FromStr};

pub fn solution_part1() -> u64 {
    let input = fs::read_to_string("src/year2025/inputs/day02.txt").expect("couldn't read a file");
    let ranges: Vec<&str> = input.trim().split(",").collect();

    check_id_ranges(ranges, 1)
}

pub fn solution_part2() -> u64 {
    let input = fs::read_to_string("src/year2025/inputs/day02.txt").expect("couldn't read a file");
    let ranges: Vec<&str> = input.trim().split(",").collect();

    check_id_ranges(ranges, 2)
}

fn check_id_ranges(ranges: Vec<&str>, part: u8) -> u64 {
    let mut out = 0;
    for range in ranges {
        let splitted: Vec<&str> = range.split("-").collect();

        let start = u64::from_str(splitted[0]).unwrap();
        let end = u64::from_str(splitted[1]).unwrap();

        out += check_id_range(start..=end, part);
    }
    out
}

fn check_id_range(range: RangeInclusive<u64>, part: u8) -> u64 {
    let mut out = 0;
    for id in range {
        if part == 1 {
            if !check_id(id) {
                out += id;
            }
        } else {
            if !check_id_part2(id) {
                out += id;
            }
        }
    }
    out
}

// return 'true' if it a valid id
fn check_id(id: u64) -> bool {
    let base = 10;
    let n = id.checked_ilog(base).unwrap_or(0) + 1;

    if n % 2 != 0 {
        return true;
    }

    let middle = base.pow(n / 2);

    let left = id.div_euclid(middle);
    let right = id % middle;
    left != right
}

// return 'true' if it a valid id
fn check_id_part2(id: u64) -> bool {
    let base = 10;
    let n = id.checked_ilog(base).unwrap_or(0) + 1;

    for idx in 1..=n / 2 {
        let tens = base.pow(idx);
        let part = id % tens;
        let mut test = 0;
        for _ in 0..n / idx {
            test *= tens;
            test += part;
        }

        if test == id {
            return false;
        }
    }

    true
}

mod test {
    use crate::year2025::day02::*;

    #[test]
    fn test_check_id() {
        let x = 22;
        assert_eq!(check_id(x), false);
        let x = 2222;
        assert_eq!(check_id(x), false);
        let x = 222;
        assert_eq!(check_id(x), true);
    }

    #[test]
    fn test_check_id_range() {
        assert_eq!(check_id_range(11..=22, 1), 33);
        assert_eq!(check_id_range(95..=115, 1), 99);
        assert_eq!(check_id_range(38593856..=38593862, 1), 38593859);
    }

    #[test]
    fn test_check_id_ranges() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges: Vec<&str> = input.split(",").collect();

        assert_eq!(check_id_ranges(ranges, 1), 1227775554);
    }

    #[test]
    fn test_check_id_part2() {
        let x = 22;
        assert_eq!(check_id_part2(x), false);
        let x = 2222;
        assert_eq!(check_id_part2(x), false);
        let x = 222;
        assert_eq!(check_id_part2(x), false);
    }

    #[test]
    fn test_check_id_range_part2() {
        assert_eq!(check_id_range(11..=22, 2), 33);
        assert_eq!(check_id_range(95..=115, 2), 99 + 111);
        assert_eq!(check_id_range(38593856..=38593862, 2), 38593859);
    }

    #[test]
    fn test_check_id_ranges_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges: Vec<&str> = input.split(",").collect();

        assert_eq!(check_id_ranges(ranges, 2), 4174379265);
    }
}
