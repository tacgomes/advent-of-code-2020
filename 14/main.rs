use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    let mut memory = HashMap::new();
    let mut and_bitmask = 0;
    let mut or_bitmask = 0;

    for line in lines {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(" = ").collect();
        let (key, value) = (parts[0], parts[1]);

        if key == "mask" {
            and_bitmask = usize::from_str_radix(&value.replace('X', "1"), 2).unwrap();
            or_bitmask = usize::from_str_radix(&value.replace('X', "0"), 2).unwrap();
        } else if key.starts_with("mem") {
            let addr = key[4..key.len() - 1].parse::<usize>().unwrap();
            let value = value.parse::<usize>().unwrap();
            memory.insert(addr, value & and_bitmask | or_bitmask);
        }
    }

    memory.values().sum()
}

fn calculate_part2(file_name: impl AsRef<Path>) -> usize {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();

    let mut memory = HashMap::new();
    let mut bitmask = String::new();

    for line in lines {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(" = ").collect();
        let (key, value) = (parts[0], parts[1]);

        if key == "mask" {
            bitmask = value.to_string();
        } else if key.starts_with("mem") {
            let addr = key[4..key.len() - 1].parse::<usize>().unwrap();
            let value = value.parse::<usize>().unwrap();

            let mut chars: Vec<_> = bitmask
                .chars()
                .rev()
                .enumerate()
                .map(|(index, ch)| match ch as char {
                    'X' => 'X',
                    '0' => {
                        if (addr >> index & 1) > 0 {
                            '1'
                        } else {
                            '0'
                        }
                    }
                    '1' => '1',
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

            let mut addresses = vec![];
            generate_addresses(&mut chars, 0, &mut addresses);

            for address in addresses {
                memory.insert(address, value);
            }
        }
    }

    memory.values().sum()
}

fn generate_addresses(bitmask: &mut Vec<char>, index: usize, addresses: &mut Vec<usize>) {
    if index == bitmask.len() {
        let bitmask = bitmask.iter().collect::<String>();
        addresses.push(usize::from_str_radix(&bitmask, 2).unwrap());
        return;
    }

    match bitmask[index] {
        '0' | '1' => generate_addresses(bitmask, index + 1, addresses),
        'X' => {
            bitmask[index] = '0';
            generate_addresses(bitmask, index + 1, addresses);
            bitmask[index] = '1';
            generate_addresses(bitmask, index + 1, addresses);
            bitmask[index] = 'X';
        }
        _ => unreachable!(),
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = calculate_part1(env::args().nth(1).unwrap());
    let part2 = calculate_part2(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(calculate_part1("example1.txt"), 165);
    }

    #[test]
    fn test_example_input_2() {
        assert_eq!(calculate_part2("example2.txt"), 208);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 10035335144067);
        assert_eq!(calculate_part2("input.txt"), 3817372618036);
    }
}
