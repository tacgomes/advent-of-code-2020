use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Eq, PartialEq)]
enum StackType {
    Parens,
    Mult,
}

fn calculate_sum_part1(line: &[char]) -> usize {
    let add: fn(usize, usize) -> usize = |x: usize, y: usize| -> usize { x + y };
    let mul: fn(usize, usize) -> usize = |x: usize, y: usize| -> usize { x * y };

    let mut num = 0;
    let mut op = add;
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

fn calculate_sum_part2(line: &[char]) -> usize {
    let mut num = 0;
    let mut stack = vec![];

    for c in line {
        match c {
            '0'..='9' => {
                let new_num = c.to_digit(10).unwrap() as usize;
                num += new_num;
            }
            '+' => (),
            '*' => {
                if let Some((n, StackType::Mult)) = stack.last() {
                    num *= n;
                    stack.pop();
                }
                stack.push((num, StackType::Mult));
                num = 0;
            }
            '(' => {
                stack.push((num, StackType::Parens));
                num = 0;
            }
            ')' => {
                if let Some((n, StackType::Mult)) = stack.last() {
                    num *= n;
                    stack.pop();
                }
                let (prev_num, _stype) = stack.pop().unwrap();
                num += prev_num;
            }
            _ => unreachable!(),
        }
    }

    if let Some((m, StackType::Mult)) = stack.pop() {
        num *= m;
    }

    num
}

fn calculate_all_sums_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    content
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part1(&x.chars().filter(|&c| c != ' ').collect::<Vec<_>>()))
        .sum()
}

fn calculate_all_sums_part2(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    content
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part2(&x.chars().filter(|&c| c != ' ').collect::<Vec<_>>()))
        .sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let res1 = calculate_all_sums_part1(env::args().nth(1).unwrap());
    let res2 = calculate_all_sums_part2(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", res1);
    println!("Result (Part 1): {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_all_sums_part1("example1.txt"), 71);
        assert_eq!(calculate_all_sums_part2("example1.txt"), 231);
    }

    #[test]
    fn test_example_input_2() {
        assert_eq!(calculate_all_sums_part1("example2.txt"), 51);
        assert_eq!(calculate_all_sums_part2("example2.txt"), 51);
    }

    #[test]
    fn test_example_input_3() {
        assert_eq!(calculate_all_sums_part1("example3.txt"), 26335);
        assert_eq!(calculate_all_sums_part2("example3.txt"), 693891);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_all_sums_part1("input.txt"), 3647606140187);
        assert_eq!(calculate_all_sums_part2("input.txt"), 323802071857594);
    }
}
