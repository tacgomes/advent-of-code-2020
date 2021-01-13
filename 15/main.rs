use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn solve(numbers: &[usize], target_turn: usize) -> usize {
    let mut spoken_numbers = numbers
        .iter()
        .enumerate()
        .map(|(turn, num)| (*num, turn + 1))
        .collect::<HashMap<_, _>>();

    let mut last_turn_number = *numbers.last().unwrap();
    for turn in spoken_numbers.len()..target_turn {
        let this_turn_number = match spoken_numbers.get(&last_turn_number) {
            None => 0,
            Some(t) => turn - t,
        };
        spoken_numbers.insert(last_turn_number, turn);
        last_turn_number = this_turn_number;
    }

    last_turn_number
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<usize> {
    fs::read_to_string(&file_name)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    if env::args().count() != 3 {
        eprintln!("USAGE: {} FILE TURN", env::args().next().unwrap());
        process::exit(1);
    }

    let numbers = parse_input(env::args().nth(1).unwrap());
    let target_turn = env::args().nth(2).unwrap().parse().unwrap();
    let result = solve(&numbers, target_turn);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let numbers = parse_input("example1.txt");
        assert_eq!(solve(&numbers, 2020), 436);
    }

    #[test]
    fn test_example_input_2() {
        let numbers = parse_input("example2.txt");
        assert_eq!(solve(&numbers, 2020), 1);
    }

    #[test]
    fn test_example_input_3() {
        let numbers = parse_input("example3.txt");
        assert_eq!(solve(&numbers, 2020), 10);
    }

    #[test]
    fn test_example_input_4() {
        let numbers = parse_input("example4.txt");
        assert_eq!(solve(&numbers, 2020), 27);
    }

    #[test]
    fn test_example_input_5() {
        let numbers = parse_input("example5.txt");
        assert_eq!(solve(&numbers, 2020), 78);
    }

    #[test]
    fn test_example_input_6() {
        let numbers = parse_input("example6.txt");
        assert_eq!(solve(&numbers, 2020), 438);
    }

    #[test]
    fn test_example_input_7() {
        let numbers = parse_input("example7.txt");
        assert_eq!(solve(&numbers, 2020), 1836);
    }

    #[test]
    fn test_puzzle_input() {
        let numbers = parse_input("input.txt");
        assert_eq!(solve(&numbers, 2020), 929);
    }
}
