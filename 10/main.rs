use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn parse_input(file_name: impl AsRef<Path>) -> Vec<usize> {
    let mut jolts: Vec<_> = fs::read_to_string(&file_name)
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    jolts.sort_unstable();
    jolts.push(jolts.last().unwrap() + 3);
    jolts
}

fn count_differences(jolts: &[usize]) -> Option<usize> {
    let (mut prev_jolt, mut diff1, mut diff3) = (0, 0, 0);

    for &jolt in jolts {
        match jolt - prev_jolt {
            1 => diff1 += 1,
            3 => diff3 += 1,
            0 | 2 => continue,
            _ => return None,
        }
        prev_jolt = jolt;
    }

    Some(diff1 * diff3)
}

fn count_arrangements(jolts: &[usize]) -> usize {
    let mut counts = vec![0; *jolts.last().unwrap() + 1];
    counts[0] = 1;

    // Algorithm:
    //   Q: How many arrangements are possible for jolt `j`?
    //   A: As many as the sum of the arrangements for jolts `j - 1`,
    //   `j - 2` and `j - 3`.
    for &jolt in [0].iter().chain(jolts.iter().take(jolts.len() - 1)) {
        counts[jolt + 1] += counts[jolt];
        counts[jolt + 2] += counts[jolt];
        counts[jolt + 3] += counts[jolt];
    }

    *counts.last().unwrap()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let jolts = parse_input(env::args().nth(1).unwrap());
    let part1 = count_differences(&jolts);
    let part2 = count_arrangements(&jolts);
    println!("Result (Part 1): {:?}", part1);
    println!("Result (Part 2): {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let jolts = parse_input("example1.txt");
        assert_eq!(count_differences(&jolts), Some(35));
        assert_eq!(count_arrangements(&jolts), 8);
    }

    #[test]
    fn test_example_input_2() {
        let jolts = parse_input("example2.txt");
        assert_eq!(count_differences(&jolts), Some(220));
        assert_eq!(count_arrangements(&jolts), 19208);
    }

    #[test]
    fn test_puzzle_input() {
        let jolts = parse_input("input.txt");
        assert_eq!(count_differences(&jolts), Some(1876));
        assert_eq!(count_arrangements(&jolts), 14173478093824);
    }
}
