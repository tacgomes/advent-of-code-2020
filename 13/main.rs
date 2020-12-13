use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

struct BusSchedule {
    earliest_ts: usize,
    bus_ids: Vec<usize>,
    departing_offsets: Vec<(usize, isize)>,
}

impl BusSchedule {
    fn from_file(file_name: impl AsRef<Path>) -> Self {
        let file = File::open(file_name).unwrap();
        let mut lines = BufReader::new(file).lines();

        let mut bus_ids = vec![];
        let mut departing_offsets = vec![];

        let earliest_ts = lines.next().unwrap().unwrap().parse().unwrap();

        for (index, bus_id) in lines
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .enumerate()
            .filter(|(_, x)| !x.starts_with('x'))
        {
            let bus_id = bus_id.parse().unwrap();
            bus_ids.push(bus_id);
            departing_offsets.push((bus_id, index as isize));
        }

        BusSchedule {
            earliest_ts,
            bus_ids,
            departing_offsets,
        }
    }

    fn part1(&self) -> usize {
        let mut timestamps: Vec<_> = self
            .bus_ids
            .iter()
            .map(|x| (x, (x - (self.earliest_ts % x) % x) + self.earliest_ts))
            .collect();
        timestamps.sort_by_key(|&(_, ts)| ts);
        (timestamps[0].1 - self.earliest_ts) * timestamps[0].0
    }

    fn part2_brute_force(&self) -> Option<isize> {
        // TODO: add an implementation of part 2 using the much more
        // efficient Chinese Remainder Theorem.

        let highest_bus_id = self.bus_ids.iter().max().unwrap();
        let highest_bus_offset = self
            .departing_offsets
            .iter()
            .find(|(x, _)| x == highest_bus_id)
            .unwrap()
            .1;

        let mut departing_offsets = self.departing_offsets.clone();
        for (_, offset) in &mut departing_offsets {
            *offset -= highest_bus_offset as isize;
        }

        for n in (0isize..).step_by(*highest_bus_id) {
            if departing_offsets
                .iter()
                .all(|(bus_id, offset)| (n + offset) % *bus_id as isize == 0)
            {
                return Some(n + departing_offsets[0].1);
            }
        }

        None
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let bus_schedule = BusSchedule::from_file(env::args().nth(1).unwrap());
    let part1 = bus_schedule.part1();
    let part2_brute_force = bus_schedule.part2_brute_force();
    println!("Result (Part 1): {}", part1);
    println!("Result (Part 2): {:?}", part2_brute_force);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let bus_schedule = BusSchedule::from_file("example1.txt");
        assert_eq!(bus_schedule.part1(), 295);
        assert_eq!(bus_schedule.part2_brute_force(), Some(1068781));
    }

    #[test]
    fn test_example_input_2() {
        let bus_schedule = BusSchedule::from_file("example2.txt");
        assert_eq!(bus_schedule.part2_brute_force(), Some(3417));
    }

    #[test]
    fn test_example_input_3() {
        let bus_schedule = BusSchedule::from_file("example3.txt");
        assert_eq!(bus_schedule.part2_brute_force(), Some(754018));
    }

    #[test]
    fn test_example_input_4() {
        let bus_schedule = BusSchedule::from_file("example4.txt");
        assert_eq!(bus_schedule.part2_brute_force(), Some(779210));
    }

    #[test]
    fn test_example_input_5() {
        let bus_schedule = BusSchedule::from_file("example5.txt");
        assert_eq!(bus_schedule.part2_brute_force(), Some(1261476));
    }

    #[test]
    fn test_example_input_6() {
        let bus_schedule = BusSchedule::from_file("example6.txt");
        assert_eq!(bus_schedule.part2_brute_force(), Some(1202161486));
    }

    #[test]
    fn test_puzzle_input() {
        let bus_schedule = BusSchedule::from_file("input.txt");
        assert_eq!(bus_schedule.part1(), 115);
    }
}
