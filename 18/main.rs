use std::env;
use std::fs;
use std::process;

#[derive(PartialEq)]
enum Token {
    Digit(u8),
    OpAdd,
    OpMult,
    LeftParens,
    RightParens,
}

#[derive(PartialEq)]
enum PrecedenceLevel {
    Parens,
    Mult,
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

fn evaluate_reverse_polish(tokens: &[Token]) -> usize {
    let mut stack = vec![];
    for token in tokens {
        match token {
            Token::Digit(n) => stack.push(*n as usize),
            Token::OpAdd => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::OpMult => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            _ => unreachable!(),
        }
    }
    *stack.first().unwrap()
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

fn calculate_sum_part2_inplace(tokens: &[Token]) -> usize {
    let mut num = 0;
    let mut stack = vec![];

    for token in tokens {
        match token {
            Token::Digit(n) => num += *n as usize,
            Token::OpAdd => (),
            Token::OpMult => {
                if let Some((n, PrecedenceLevel::Mult)) = stack.last() {
                    num *= n;
                    stack.pop();
                }
                stack.push((num, PrecedenceLevel::Mult));
                num = 0;
            }
            Token::LeftParens => {
                stack.push((num, PrecedenceLevel::Parens));
                num = 0;
            }
            Token::RightParens => {
                if let Some((n, PrecedenceLevel::Mult)) = stack.last() {
                    num *= n;
                    stack.pop();
                }
                let (prev_num, _stype) = stack.pop().unwrap();
                num += prev_num;
            }
        }
    }

    if let Some((n, PrecedenceLevel::Mult)) = stack.pop() {
        num *= n;
    }

    num
}

fn calculate_sum_part2_shunting_yard(tokens: &[Token]) -> usize {
    let mut outqueue = vec![];
    let mut opstack = vec![];

    for token in tokens {
        match token {
            Token::Digit(n) => outqueue.push(Token::Digit(*n)),
            Token::OpAdd => opstack.push(Token::OpAdd),
            Token::OpMult => {
                while let Some(Token::OpAdd) = opstack.last() {
                    outqueue.push(opstack.pop().unwrap());
                }
                opstack.push(Token::OpMult);
            }
            Token::LeftParens => {
                opstack.push(Token::LeftParens);
            }
            Token::RightParens => {
                while let Some(t) = opstack.pop() {
                    if t != Token::LeftParens {
                        outqueue.push(t);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    while let Some(t) = opstack.pop() {
        outqueue.push(t);
    }

    evaluate_reverse_polish(&outqueue)
}

fn calculate_sums_part1(input: &str) -> usize {
    input
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part1(&lex(&x)))
        .sum()
}

fn calculate_sums_part2_inplace(input: &str) -> usize {
    input
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part2_inplace(&lex(&x)))
        .sum()
}

fn calculate_sums_part2_shunting_yard(input: &str) -> usize {
    input
        .trim()
        .split('\n')
        .map(|x| calculate_sum_part2_shunting_yard(&lex(&x)))
        .sum()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let input = fs::read_to_string(&env::args().nth(1).unwrap()).unwrap();
    let part1 = calculate_sums_part1(&input);
    let part2a = calculate_sums_part2_inplace(&input);
    let part2b = calculate_sums_part2_shunting_yard(&input);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2 inplace): {}", part2a);
    println!("Result (Part 2 shunting-yard): {}", part2b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(calculate_sums_part1(&input), 71);
        assert_eq!(calculate_sums_part2_inplace(&input), 231);
        assert_eq!(calculate_sums_part2_shunting_yard(&input), 231);
    }

    #[test]
    fn test_example_input_2() {
        let input = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(calculate_sums_part1(&input), 51);
        assert_eq!(calculate_sums_part2_inplace(&input), 51);
        assert_eq!(calculate_sums_part2_shunting_yard(&input), 51);
    }

    #[test]
    fn test_example_input_3() {
        let input = fs::read_to_string("example3.txt").unwrap();
        assert_eq!(calculate_sums_part1(&input), 26335);
        assert_eq!(calculate_sums_part2_inplace(&input), 693891);
        assert_eq!(calculate_sums_part2_shunting_yard(&input), 693891);
    }

    #[test]
    fn test_puzzle_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(calculate_sums_part1(&input), 3647606140187);
        assert_eq!(calculate_sums_part2_inplace(&input), 323802071857594);
        assert_eq!(calculate_sums_part2_shunting_yard(&input), 323802071857594);
    }
}
