use std::env;
use std::process;

const PART_2_NUM_CUPS: usize = 1_000_000;
const PART_2_NUM_ITERS: usize = 10_000_000;

fn iterate(cups: &mut Vec<usize>, cup: usize) -> usize {
    let lowest = 1;
    let highest = cups.len() - 1;

    let a = cups[cup];
    let b = cups[a];
    let c = cups[b];

    cups[cup] = cups[c];

    let mut dest = cup - 1;
    while dest == a || dest == b || dest == c || dest < lowest {
        if dest < lowest {
            dest = highest;
        } else {
            dest -= 1;
        }
    }

    let dest_next = cups[dest];
    cups[dest] = a;
    cups[c] = dest_next;

    cups[cup]
}

fn calculate_part1(cups: &str) -> String {
    let input = cups
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut cups = vec![0; input.len() + 1];
    for i in 0..input.len() - 1 {
        cups[input[i]] = input[i + 1];
    }

    cups[input[input.len() - 1]] = input[0];

    let mut cup = input[0];
    for _ in 0..100 {
        cup = iterate(&mut cups, cup);
    }

    let mut cup = cups[1];
    let mut result = String::new();
    while cup != 1 {
        result.push(cup.to_string().chars().next().unwrap());
        cup = cups[cup];
    }

    result
}

fn calculate_part2(cups: &str) -> usize {
    let input = cups
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut cups = vec![0; PART_2_NUM_CUPS + 1];
    for i in 0..input.len() - 1 {
        cups[input[i]] = input[i + 1];
    }

    cups[*input.iter().last().unwrap()] = input.len() + 1;

    for (i, cup) in cups
        .iter_mut()
        .enumerate()
        .take(PART_2_NUM_CUPS)
        .skip(input.len() + 1)
    {
        *cup = i + 1;
    }

    *cups.last_mut().unwrap() = input[0];

    let mut cup = input[0];
    for _ in 0..PART_2_NUM_ITERS {
        cup = iterate(&mut cups, cup);
    }

    cups[1] * cups[cups[1]]
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} NUMBER", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = calculate_part1(&env::args().nth(1).unwrap());
    let part2 = calculate_part2(&env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(calculate_part1("389125467"), "67384529");
        assert_eq!(calculate_part2("389125467"), 149245887792);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("418976235"), "96342875");
        assert_eq!(calculate_part2("418976235"), 563362809504);
    }
}
