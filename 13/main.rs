use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn solve_part1(timestamp: usize, schedule: &[(usize, isize)]) -> usize {
    let mut timestamps = schedule
        .iter()
        .map(|(x, _)| (x, (x - (timestamp % x) % x) + timestamp))
        .collect::<Vec<_>>();
    timestamps.sort_by_key(|&(_, ts)| ts);
    (timestamps[0].1 - timestamp) * timestamps[0].0
}

fn solve_part2_brute_force(schedule: &[(usize, isize)]) -> Option<isize> {
    // TODO: add an implementation of part 2 using the much more
    // efficient Chinese Remainder Theorem.

    let highest_bus_id = schedule.iter().map(|(x, _)| x).max().unwrap();
    let highest_bus_offset = schedule
        .iter()
        .find(|(x, _)| x == highest_bus_id)
        .unwrap()
        .1;

    let mut schedule = schedule.to_owned();
    for (_, offset) in schedule.iter_mut() {
        *offset -= highest_bus_offset as isize;
    }

    for n in (0isize..).step_by(*highest_bus_id) {
        if schedule
            .iter()
            .all(|(bus_id, offset)| (n + offset) % *bus_id as isize == 0)
        {
            return Some(n + schedule[0].1);
        }
    }

    None
}

fn parse_input(file_name: impl AsRef<Path>) -> (usize, Vec<(usize, isize)>) {
    let content = fs::read_to_string(&file_name).unwrap();
    let mut lines = content.lines();

    let timestamp = lines.next().unwrap().parse().unwrap();
    let schedule = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, b)| !b.starts_with('x'))
        .map(|(i, b)| (b.parse().unwrap(), i as isize))
        .collect();

    (timestamp, schedule)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (timestamp, schedule) = parse_input(env::args().nth(1).unwrap());
    let part1 = solve_part1(timestamp, &schedule);
    let part2 = solve_part2_brute_force(&schedule);
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let (timestamp, schedule) = parse_input("example1.txt");
        assert_eq!(solve_part1(timestamp, &schedule), 295);
        assert_eq!(solve_part2_brute_force(&schedule), Some(1068781));
    }

    #[test]
    fn test_example_input_2() {
        let (_, schedule) = parse_input("example2.txt");
        assert_eq!(solve_part2_brute_force(&schedule), Some(3417));
    }

    #[test]
    fn test_example_input_3() {
        let (_, schedule) = parse_input("example3.txt");
        assert_eq!(solve_part2_brute_force(&schedule), Some(754018));
    }

    #[test]
    fn test_example_input_4() {
        let (_, schedule) = parse_input("example4.txt");
        assert_eq!(solve_part2_brute_force(&schedule), Some(779210));
    }

    #[test]
    fn test_example_input_5() {
        let (_, schedule) = parse_input("example5.txt");
        assert_eq!(solve_part2_brute_force(&schedule), Some(1261476));
    }

    #[test]
    fn test_example_input_6() {
        let (_, schedule) = parse_input("example6.txt");
        assert_eq!(solve_part2_brute_force(&schedule), Some(1202161486));
    }

    #[test]
    fn test_puzzle_input() {
        let (timestamp, schedule) = parse_input("input.txt");
        assert_eq!(solve_part1(timestamp, &schedule), 115);
    }
}
