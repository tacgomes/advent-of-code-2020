use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const TARGET: i32 = 2020;

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut set = HashSet::new();

    for line in lines {
        let n = line.unwrap().parse::<i32>().unwrap();
        let diff = TARGET - n;
        if set.contains(&diff) {
            println!("Result: {}", diff * n);
            break;
        }
        set.insert(n);
    }
}
