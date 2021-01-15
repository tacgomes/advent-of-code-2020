use std::env;
use std::fs;
use std::process;

enum Token {
    Digit(u8),
    OpAdd,
    OpMult,
    LeftParens,
    RightParens,
}

#[derive(Eq, PartialEq)]
enum StackType {
    Parens,
    Mult,
}

fn calculate_sum_part1(tokens: &[Token]) -> usize {
    let add: fn(usize, usize) -> usize = |x, y| x + y;
    let mul: fn(usize, usize) -> usize = |x, y| x * y;

    let mut num = 0;
    let mut op = add;
    let mut stack = vec![];

    for token in tokens {
        match token {
            Token::Digit(n) => num = op(num, *n as usize),
            Token::OpAdd => op = add,
            Token::OpMult => op = mul,
            Token::LeftParens => {
                stack.push((num, op));
                num = 0;
                op = add;
            }
            Token::RightParens => {
                let (prev_num, prev_op) = stack.pop().unwrap();
                num = prev_op(prev_num, num)
            }
        }
    }

    assert!(stack.is_empty());
    num
}

fn calculate_sum_part2(tokens: &[Token]) -> usize {
    let mut num = 0;
    let mut stack = vec![];

    for token in tokens {
        match token {
            Token::Digit(n) => num += *n as usize,
            Token::OpAdd => (),
            Token::OpMult => {
                if let Some((n, StackType::Mult)) = stack.last() {
                    num *= n;
                    stack.pop();
                }
                stack.push((num, StackType::Mult));
                num = 0;
            }
            Token::LeftParens => {
                stack.push((num, StackType::Parens));
                num = 0;
            }
            Token::RightParens => {
                if let Some((n, StackType::Mult)) = stack.last() {
                    num *= n;
                    stack.pop();
                }
                let (prev_num, _stype) = stack.pop().unwrap();
                num += prev_num;
            }
        }
    }

    if let Some((m, StackType::Mult)) = stack.pop() {
        num *= m;
    }

    num
}

fn lex(s: &str) -> Vec<Token> {
    s.chars()
        .filter(|&c| c != ' ')
        .map(|c| match c {
            '0'..='9' => Token::Digit(c.to_digit(10).unwrap() as u8),
            '+' => Token::OpAdd,
            '*' => Token::OpMult,
            '(' => Token::LeftParens,
            ')' => Token::RightParens,
            _ => unreachable!(),
        })
        .collect()
}

fn calculate_all_sums_part1(input: &str) -> usize {
    input
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part1(&lex(&x)))
        .sum()
}

fn calculate_all_sums_part2(input: &str) -> usize {
    input
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part2(&lex(&x)))
        .sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let input = fs::read_to_string(&env::args().nth(1).unwrap()).unwrap();
    let part1 = calculate_all_sums_part1(&input);
    let part2 = calculate_all_sums_part2(&input);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(calculate_all_sums_part1(&input), 71);
        assert_eq!(calculate_all_sums_part2(&input), 231);
    }

    #[test]
    fn test_example_input_2() {
        let input = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(calculate_all_sums_part1(&input), 51);
        assert_eq!(calculate_all_sums_part2(&input), 51);
    }

    #[test]
    fn test_example_input_3() {
        let input = fs::read_to_string("example3.txt").unwrap();
        assert_eq!(calculate_all_sums_part1(&input), 26335);
        assert_eq!(calculate_all_sums_part2(&input), 693891);
    }

    #[test]
    fn test_puzzle_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(calculate_all_sums_part1(&input), 3647606140187);
        assert_eq!(calculate_all_sums_part2(&input), 323802071857594);
    }
}
