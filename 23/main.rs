use std::env;
use std::process;

fn do_move(cups: &mut Vec<usize>, mut cup_index: usize, lowest: usize, highest: usize) -> usize {
    let remove_idx = |idx, len| if (idx + 1) < len { idx + 1 } else { 0 };

    let mut destination = cups[cup_index] - 1;

    let adjust = cups.len() as isize - (cup_index as isize + 1) - 3;

    let (a, b, c) = (
        cups.remove(remove_idx(cup_index, cups.len())),
        cups.remove(remove_idx(cup_index, cups.len())),
        cups.remove(remove_idx(cup_index, cups.len())),
    );

    if adjust < 0 {
        cup_index = (cup_index as isize + adjust) as usize;
    }

    loop {
        if destination < lowest {
            destination = highest;
        }

        if destination != a && destination != b && destination != c {
            break;
        }

        destination -= 1;
    }

    let position = cups.iter().position(|&x| x == destination).unwrap();

    cups.insert(position + 1, c);
    cups.insert(position + 1, b);
    cups.insert(position + 1, a);

    if position < cup_index {
        cup_index += 3;
    }

    (cup_index + 1) % cups.len()
}

fn calculate_part1(cups: &str) -> String {
    let mut cups = cups
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let lowest = *cups.iter().min().unwrap();
    let highest = *cups.iter().max().unwrap();

    let mut cup_index = 0;
    for _r in 0..100 {
        cup_index = do_move(&mut cups, cup_index, lowest, highest);
    }

    let mut result = String::new();
    let cup1_position = cups.iter().position(|&x| x == 1).unwrap();
    for i in cup1_position + 1..cup1_position + cups.len() {
        result.push(cups[i % cups.len()].to_string().chars().next().unwrap());
    }

    result
}

fn calculate_part2(cups: &str) -> usize {
    let mut cups = cups
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let lowest = *cups.iter().min().unwrap();
    let highest = *cups.iter().max().unwrap();

    for i in highest..1_000_000 {
        cups.push(i);
    }

    let mut cup_index = 0;
    for _r in 0..10_000_000 {
        cup_index = do_move(&mut cups, cup_index, lowest, highest);
    }

    let cup1_position = cups.iter().position(|&x| x == 1).unwrap();
    cups[cup1_position - 1] * cups[cup1_position - 2]
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} NUMBER", env::args().next().unwrap());
        process::exit(1);
    }

    let part1 = calculate_part1(&env::args().nth(1).unwrap());
    // let part2 = calculate_part2(&env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", part1);
    // println!("Result (Part 2): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(calculate_part1("389125467"), "67384529");
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("418976235"), "96342875");
    }
}
