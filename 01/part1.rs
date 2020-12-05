use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

const TARGET_SUM: i32 = 2020;

fn find_solution(file_name: impl AsRef<Path>) -> Option<i32> {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    let mut set = HashSet::new();

    for line in lines {
        let n = line.unwrap().parse::<i32>().unwrap();
        let diff = TARGET_SUM - n;
        if set.contains(&diff) {
            return Some(n * diff);
        }
        set.insert(n);
    }
    None
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let result = find_solution(env::args().nth(1).unwrap());
    println!("Result: {:?}", result);
}
