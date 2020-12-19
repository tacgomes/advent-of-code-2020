use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

fn read_jolts(file_name: impl AsRef<Path>) -> Vec<u32> {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();
    let mut jolts: Vec<_> = lines.map(|x| x.unwrap().parse().unwrap()).collect();
    jolts.sort_unstable();
    jolts.push(jolts.last().unwrap() + 3);
    jolts
}

fn find_jolt_diff(file_name: impl AsRef<Path>) -> Option<u32> {
    let jolts = read_jolts(file_name);
    let (mut prev_jolt, mut diff1, mut diff3) = (0, 0, 0);

    for j in jolts {
        match j - prev_jolt {
            1 => diff1 += 1,
            3 => diff3 += 1,
            0 | 2 => continue,
            _ => return None,
        }
        prev_jolt = j;
    }

    Some(diff1 * diff3)
}


fn count_arrangements(file_name: impl AsRef<Path>) -> u64 {
    let jolts = read_jolts(file_name);
    let mut cache = HashMap::new();
    // NB: while this solution works and it is not inefficient due the
    // use of dynamic programming, there are simpler ways of calculating
    // the number of arrangements. In the page referred by the following
    // link, the solution posted by user `Zealousideal_Bit_601` is
    // simple and well explained:
    // https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions/
    //
    // TODO refactor and simplify this function
    (0..3)
        .map(|i| count_arrangements_util(0, i, &jolts, &mut cache))
        .sum()
}

fn count_arrangements_util(
    jolt: u32,
    idx: usize,
    jolts: &[u32],
    mut cache: &mut HashMap<u32, u64>,
) -> u64 {
    if idx >= jolts.len() || jolts[idx] - jolt > 3 {
        return 0;
    }

    if idx == jolts.len() - 1 {
        return 1;
    }

    if let Some(count) = cache.get(&jolts[idx]) {
        return *count;
    }

    let count = (1..=3)
        .map(|i| count_arrangements_util(jolts[idx], idx + i, &jolts, &mut cache))
        .sum();

    cache.insert(jolts[idx], count);

    count
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let jolt_diff = find_jolt_diff(env::args().nth(1).unwrap());
    let arrangements_count = count_arrangements(env::args().nth(1).unwrap());
    println!("Result (Part 1): {:?}", jolt_diff);
    println!("Result (Part 2): {:?}", arrangements_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        assert_eq!(find_jolt_diff("example1.txt"), Some(35));
        assert_eq!(count_arrangements("example1.txt"), 8);
    }

    #[test]
    fn test_example_input_2() {
        assert_eq!(find_jolt_diff("example2.txt"), Some(220));
        assert_eq!(count_arrangements("example2.txt"), 19208);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(find_jolt_diff("input.txt"), Some(1876));
        assert_eq!(count_arrangements("input.txt"), 14173478093824);
    }
}
