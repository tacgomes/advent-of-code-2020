use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, PartialEq)]
enum BootCodeResult {
    Terminated(i32),
    Cyclic(i32),
}

fn jmp_ip(ip: usize, jmp: i32) -> usize {
    (ip as isize + jmp as isize) as usize
}

struct BootCode {
    instructions: Vec<Instruction>,
}

impl BootCode {
    fn new(instructions: Vec<Instruction>) -> Self {
        BootCode { instructions }
    }

    fn run(&self) -> BootCodeResult {
        let mut acc = 0;
        let mut ip = 0;
        let mut executed = vec![false; self.instructions.len()];

        while ip != self.instructions.len() {
            if executed[ip] {
                return BootCodeResult::Cyclic(acc);
            } else {
                executed[ip] = true;
            }

            match self.instructions[ip] {
                Instruction::Nop(_) => {
                    ip += 1;
                }
                Instruction::Acc(val) => {
                    acc += val;
                    ip += 1;
                }
                Instruction::Jmp(val) => {
                    ip = jmp_ip(ip, val);
                }
            }
        }

        BootCodeResult::Terminated(acc)
    }

    fn run_with_fix(&self) -> BootCodeResult {
        for (ip, ins) in self.instructions.iter().enumerate() {
            match ins {
                Instruction::Nop(val) => {
                    let mut new_instructions = self.instructions.clone();
                    new_instructions[ip] = Instruction::Jmp(*val);
                    let r = BootCode::new(new_instructions).run();
                    if let BootCodeResult::Terminated(_) = r {
                        return r;
                    }
                }
                Instruction::Jmp(val) => {
                    let mut new_instructions = self.instructions.clone();
                    new_instructions[ip] = Instruction::Nop(*val);
                    let r = BootCode::new(new_instructions).run();
                    if let BootCodeResult::Terminated(_) = r {
                        return r;
                    }
                }
                Instruction::Acc(_) => (),
            }
        }

        self.run()
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Instruction> {
    fs::read_to_string(&file_name)
        .unwrap()
        .lines()
        .map(|x| {
            let mut parts = x.split_whitespace();
            let (op, val) = (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            );
            match op {
                "nop" => Instruction::Nop(val),
                "acc" => Instruction::Acc(val),
                "jmp" => Instruction::Jmp(val),
                _ => panic!("Invalid operation: {}", op),
            }
        })
        .collect()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let instructions = parse_input(env::args().nth(1).unwrap());
    let boot_code = BootCode::new(instructions);
    let part1 = boot_code.run();
    let part2 = boot_code.run_with_fix();
    println!("Result (Part 1): {:?}", part1);
    println!("Result (Part 2): {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let instructions = parse_input("example.txt");
        let boot_code = BootCode::new(instructions);
        assert_eq!(boot_code.run(), BootCodeResult::Cyclic(5));
        assert_eq!(boot_code.run_with_fix(), BootCodeResult::Terminated(8));
    }

    #[test]
    fn test_puzzle_input() {
        let instructions = parse_input("input.txt");
        let boot_code = BootCode::new(instructions);
        assert_eq!(boot_code.run(), BootCodeResult::Cyclic(1810));
        assert_eq!(boot_code.run_with_fix(), BootCodeResult::Terminated(969));
    }
}
