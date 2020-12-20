use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn calculate_sum(line: &[char]) -> usize {
    let add = |x: usize, y: usize| -> usize { x + y };
    let mul = |x: usize, y: usize| -> usize { x * y };

    let mut num = 0;
    let mut op: fn(usize, usize) -> usize = add;
    let mut stack = vec![];

    for c in line {
        match c {
            '0'..='9' => {
                let new_num = c.to_digit(10).unwrap() as usize;
                num = op(num, new_num);
            }
            '+' => op = add,
            '*' => op = mul,
            '(' => {
                stack.push((num, op));
                num = 0;
                op = add;
            }
            ')' => {
                let (prev_num, prev_op) = stack.pop().unwrap();
                num = prev_op(prev_num, num)
            }
            _ => unreachable!(),
        }
    }

    assert!(stack.is_empty());
    num
}

fn calculate_all_sums(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    content
        .trim()
        .split('\n')
        .map(|x| calculate_sum(&x.chars().filter(|&c| c != ' ').collect::<Vec<_>>()))
        .sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = calculate_all_sums(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_all_sums("example1.txt"), 71);
    }

    #[test]
    fn test_example_input_2() {
        assert_eq!(calculate_all_sums("example2.txt"), 51);
    }

    #[test]
    fn test_example_input_3() {
        assert_eq!(calculate_all_sums("example3.txt"), 26335);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_all_sums("input.txt"), 3647606140187);
    }
}
