use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn solve_part1(timestamp: usize, schedule: &[(usize, usize)]) -> usize {
    let mut times = schedule
        .iter()
        .map(|(x, _)| (x, x - timestamp % x))
        .collect::<Vec<_>>();

    times.sort_by_key(|(_, ts)| *ts);
    times[0].1 * times[0].0
}

fn solve_part2(schedule: &[(usize, usize)]) -> usize {
    /* The algorithm assumes that all the bus ids are co-prime, and as
     * that implies that gcd(bus1, bus2) is 1, we can derive lcm(bus1,
     * bus2) by simply multiplying the bus ids together: bus1 * bus2.
     */
    let mut num = 0;
    let mut step = schedule[0].0;
    let mut rem = &schedule[1..];

    while !rem.is_empty() {
        let (bus_id, distance) = rem[0];
        num = (num..).step_by(step).find(|x| (x + distance) % bus_id == 0).unwrap();
        step *= bus_id;
        rem = &rem[1..];
    }

    num
}

fn parse_input(file_name: impl AsRef<Path>) -> (usize, Vec<(usize, usize)>) {
    let content = fs::read_to_string(&file_name).unwrap();
    let mut lines = content.lines();

    let timestamp = lines.next().unwrap().parse().unwrap();
    let schedule = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, b)| !b.starts_with('x'))
        .map(|(i, b)| (b.parse().unwrap(), i as usize))
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
    let part2 = solve_part2(&schedule);
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
        assert_eq!(solve_part2(&schedule), 1068781);
    }

    #[test]
    fn test_example_input_2() {
        let (_, schedule) = parse_input("example2.txt");
        assert_eq!(solve_part2(&schedule), 3417);
    }

    #[test]
    fn test_example_input_3() {
        let (_, schedule) = parse_input("example3.txt");
        assert_eq!(solve_part2(&schedule), 754018);
    }

    #[test]
    fn test_example_input_4() {
        let (_, schedule) = parse_input("example4.txt");
        assert_eq!(solve_part2(&schedule), 779210);
    }

    #[test]
    fn test_example_input_5() {
        let (_, schedule) = parse_input("example5.txt");
        assert_eq!(solve_part2(&schedule), 1261476);
    }

    #[test]
    fn test_example_input_6() {
        let (_, schedule) = parse_input("example6.txt");
        assert_eq!(solve_part2(&schedule), 1202161486);
    }

    #[test]
    fn test_puzzle_input() {
        let (timestamp, schedule) = parse_input("input.txt");
        assert_eq!(solve_part1(timestamp, &schedule), 115);
        assert_eq!(solve_part2(&schedule), 756261495958122);
    }
}
