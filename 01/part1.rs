use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const TARGET: i32 = 2020;

fn main() {
    let input = env::args().nth(1).unwrap();
    let file = File::open(input).unwrap();
    let lines = BufReader::new(file).lines();

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
