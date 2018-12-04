use std::fs::{read_to_string};

fn main() {
    println!("Hello, world!");
    let mut buffer = String::new();
    let contents = match read_to_string("./problem1.txt") {
        Ok(string_contents) => string_contents,
        Err(e) => panic!("whajsdkdhaks")
    };

    println!("contents {:?}", contents);
    let mut freq: isize = 0;
    for line in contents.lines() {
        let num = match line.parse::<isize>() {
            Ok(i) => i,
            Err(e) => panic!("NUM NOT PARSED")
        };
        freq += num;
        println!("FREQ IS: {}", freq);
    }
}
