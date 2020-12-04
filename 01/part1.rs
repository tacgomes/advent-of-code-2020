use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

const TARGET: i32 = 2020;

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let file = File::open(env::args().nth(1).unwrap()).unwrap();
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
