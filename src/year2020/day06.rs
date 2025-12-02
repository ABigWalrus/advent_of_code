use std::{fs, task::Wake};

pub fn solution_part1() -> usize {
    let input = fs::read_to_string("src/input_day06").expect("couldn't read a file");
    let groups:Vec<&str> = input.split("\n\n").collect();

    let mut sum = 0;
    
    for group in groups {
        sum += count_group_distinct(group);
    }

    return sum;
}   

fn count_group_distinct(group: &str) -> usize {
    let mut distinct:Vec<char> = vec![];
    let forms:Vec<&str> = group.split_whitespace().collect();
    
    for form in forms {
        let questions:Vec<char> = form.chars().collect();
        for q in questions {
            if !distinct.contains(&q) {
                distinct.push(q);
            }
        }
    }

    return distinct.len();
}

pub fn solution_part2() -> usize {
    let input = fs::read_to_string("src/input_day06").expect("couldn't read a file");
    let groups:Vec<&str> = input.split("\n\n").collect();

    let mut sum = 0;
    
    for group in groups {
        sum += count_group_all(group);
    }

    return sum;
}   


fn count_group_all(group: &str) -> usize {
    let qs = "abcdefghijklmnopqrstuvwxyz";
    let qs:Vec<char> = qs.chars().collect();
    let mut out = 0;

    
    for a in qs {
        let mut contains = true;
        let forms:Vec<&str> = group.split_whitespace().collect();
        for form in forms {
            let questions:Vec<char> = form.chars().collect();
            
            if !questions.contains(&a) { contains = false; }
        }
        
        if contains { out += 1; }
    }

    return out;
}
