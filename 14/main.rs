use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem(usize, usize),
}

fn bit2char(x: usize) -> char {
    if x & 1 > 0 {
        '1'
    } else {
        '0'
    }
}

fn get_floating_address(floating_mask: &str, addr: usize) -> String {
    let len = floating_mask.chars().count();
    floating_mask
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => bit2char(addr >> (len - i - 1)),
            _ => c,
        })
        .collect()
}

fn expand_address(floating_addr: &str, map: usize) -> usize {
    let mut i = -1;
    let expanded_addr = floating_addr
        .chars()
        .map(|c| match c {
            'X' => {
                i += 1;
                bit2char(map >> i)
            }
            _ => c,
        })
        .collect::<String>();

    usize::from_str_radix(&expanded_addr, 2).unwrap()
}

fn solve_part1(instructions: &[Instruction]) -> usize {
    let mut mask_clear = 0;
    let mut mask_set = 0;
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                mask_clear = usize::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
                mask_set = usize::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
            }
            Instruction::Mem(addr, val) => {
                memory.insert(addr, val & mask_clear | mask_set);
            }
        }
    }

    memory.values().sum()
}

fn solve_part2(instructions: &[Instruction]) -> usize {
    let mut mask = String::new();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => mask = m.to_string(),
            Instruction::Mem(addr, val) => {
                let floating_address = get_floating_address(&mask, *addr);
                let x_count = floating_address.chars().filter(|&x| x == 'X').count();
                (0..2usize.pow(x_count as u32))
                    .map(|x| expand_address(&floating_address, x))
                    .for_each(|x| {
                        memory.insert(x, *val);
                    });
            }
        }
    }

    memory.values().sum()
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split(" = ");
    let (key, val) = (parts.next().unwrap(), parts.next().unwrap());
    if key == "mask" {
        Instruction::Mask(val.to_owned())
    } else {
        let addr = key[4..key.len() - 1].parse::<usize>().unwrap();
        let val = val.parse::<usize>().unwrap();
        Instruction::Mem(addr, val)
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Instruction> {
    let content = fs::read_to_string(&file_name).unwrap();
    content.lines().map(|x| parse_instruction(x)).collect()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let instructions = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(&instructions);
    let part2 = solve_part2(&instructions);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let instructions = parse_input("example1.txt");
        assert_eq!(solve_part1(&instructions), 165);
    }

    #[test]
    fn test_example_input_2() {
        let instructions = parse_input("example2.txt");
        assert_eq!(solve_part2(&instructions), 208);
    }

    #[test]
    fn test_puzzle_input() {
        let instructions = parse_input("input.txt");
        assert_eq!(solve_part1(&instructions), 10035335144067);
        assert_eq!(solve_part2(&instructions), 3817372618036);
    }
}
