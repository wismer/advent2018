use std::fs::{read_to_string};
use std::collections::HashMap;

fn main() {
    problem_one_part_two();
}


fn problem_one_part_one() -> Vec<isize> {
    let mut numbers: Vec<isize> = vec![];
    let contents = match read_to_string("./problem1.txt") {
        Ok(string_contents) => string_contents,
        Err(e) => panic!("whajsdkdhaks")
    };

    let mut freq: isize = 0;
    for line in contents.lines() {
        let num = match line.parse::<isize>() {
            Ok(i) => i,
            Err(e) => panic!("NUM NOT PARSED")
        };
        numbers.push(num);
        freq += num;
        println!("FREQ IS: {}", freq);
    }

    numbers
}

fn problem_one_part_two() {
    let numbers = problem_one_part_one();
    let mut map: HashMap<isize, usize> = HashMap::new();
    // let numbers = vec![3, 3, 4, -2, -4];
    let mut current_freq = 0isize;
    let mut found_freq: bool = false;

    while !found_freq {
        for n in &numbers {
            current_freq += n;

            if map.contains_key(&current_freq) {
                map.insert(current_freq, 2);
                found_freq = true;
                break;
            } else {
                map.insert(current_freq, 1);
            }
        }
    }
    println!("map: {:?}", map);
    println!("ANSWER: {:?}", current_freq);
}