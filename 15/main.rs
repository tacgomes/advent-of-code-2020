use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn calculate_num(file_name: impl AsRef<Path>, target_turn: usize) -> usize {
    let mut file = File::open(file_name).unwrap();
    let mut numbers = String::new();
    file.read_to_string(&mut numbers).unwrap();

    let mut spoken_numbers = HashMap::new();
    let mut last_number_spoken = 0;

    for (turn, num) in numbers.trim().split(',').enumerate() {
        last_number_spoken = num.parse::<usize>().unwrap();
        spoken_numbers.insert(last_number_spoken, turn + 1);
    }

    for turn in spoken_numbers.len()..target_turn {
        let this_turn_number = match spoken_numbers.get(&last_number_spoken) {
            None => 0,
            Some(t) => turn - t,
        };
        spoken_numbers.insert(last_number_spoken, turn);
        last_number_spoken = this_turn_number;
    }

    last_number_spoken
}

fn main() {
    if env::args().count() != 3 {
        eprintln!("USAGE: {} FILE TURN", env::args().next().unwrap());
        process::exit(1);
    }

    let file_name = env::args().nth(1).unwrap();
    let target_turn = env::args().nth(2).unwrap().parse().unwrap();
    let result = calculate_num(file_name, target_turn);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_num("example1.txt", 2020), 436);
    }

    #[test]
    fn test_example_input_2() {
        assert_eq!(calculate_num("example2.txt", 2020), 1);
    }

    #[test]
    fn test_example_input_3() {
        assert_eq!(calculate_num("example3.txt", 2020), 10);
    }

    #[test]
    fn test_example_input_4() {
        assert_eq!(calculate_num("example4.txt", 2020), 27);
    }

    #[test]
    fn test_example_input_5() {
        assert_eq!(calculate_num("example5.txt", 2020), 78);
    }

    #[test]
    fn test_example_input_6() {
        assert_eq!(calculate_num("example6.txt", 2020), 438);
    }

    #[test]
    fn test_example_input_7() {
        assert_eq!(calculate_num("example7.txt", 2020), 1836);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_num("input.txt", 2020), 929);
    }
}
