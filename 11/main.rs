use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

#[derive(Clone, Copy)]
struct Movement(isize, isize);

const MOVEMENTS: [Movement; 8] = [
    Movement(-1, -1),
    Movement(-1, 0),
    Movement(-1, 1),
    Movement(0, 1),
    Movement(1, 1),
    Movement(1, 0),
    Movement(1, -1),
    Movement(0, -1),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum SeatStatus {
    Empty,
    Occupied,
    Floor,
}

struct SeatingSystem {
    seats: Vec<Vec<SeatStatus>>,
    swap_num: u32,
    recurse: bool,
}

impl SeatingSystem {
    fn from_file(file_name: impl AsRef<Path>, swap_num: u32, recurse: bool) -> Self {
        let file = File::open(file_name).unwrap();
        let lines = BufReader::new(file).lines();
        let lines: Vec<_> = lines.map(|x| x.unwrap()).collect();

        let mut seats = vec![];

        for line in lines.iter() {
            let row = line
                .chars()
                .map(|ch| match ch {
                    'L' => SeatStatus::Empty,
                    '#' => SeatStatus::Occupied,
                    '.' => SeatStatus::Floor,
                    _ => unreachable!(),
                })
                .collect();
            seats.push(row);
        }

        SeatingSystem {
            seats,
            swap_num,
            recurse,
        }
    }

    fn count_occupied(&mut self) -> usize {
        while self.run_one_iteration() {}
        self.seats
            .iter()
            .flat_map(|x| x.iter())
            .filter(|&&x| x == SeatStatus::Occupied)
            .count()
    }

    fn run_one_iteration(&mut self) -> bool {
        let mut copy = self.seats.clone();
        let mut changed = false;
        for (r, row) in self.seats.iter().enumerate() {
            for (c, _cols) in row.iter().enumerate() {
                let occupied_seats = self.scan(r, c);
                match copy[r][c] {
                    SeatStatus::Empty => {
                        if occupied_seats == 0 {
                            copy[r][c] = SeatStatus::Occupied;
                            changed = true;
                        }
                    }
                    SeatStatus::Occupied => {
                        if occupied_seats >= self.swap_num {
                            copy[r][c] = SeatStatus::Empty;
                            changed = true;
                        }
                    }
                    SeatStatus::Floor => (),
                }
            }
        }
        self.seats = copy;
        changed
    }

    fn scan(&self, row: usize, col: usize) -> u32 {
        MOVEMENTS
            .iter()
            .map(|&m| self.scan_with_move(row as isize, col as isize, m))
            .sum()
    }

    fn scan_with_move(&self, mut row: isize, mut col: isize, movement: Movement) -> u32 {
        row += movement.0;
        col += movement.1;
        if row < 0 || col < 0 || row == self.num_rows() || col == self.num_cols() {
            return 0;
        }

        match self.seats[row as usize][col as usize] {
            SeatStatus::Empty => 0,
            SeatStatus::Occupied => 1,
            SeatStatus::Floor => match self.recurse {
                false => 0,
                true => self.scan_with_move(row, col, movement),
            },
        }
    }

    fn num_rows(&self) -> isize {
        self.seats.len() as isize
    }

    fn num_cols(&self) -> isize {
        self.seats[0].len() as isize
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let mut seating_system = SeatingSystem::from_file(env::args().nth(1).unwrap(), 4, false);
    let num_occupied_part1 = seating_system.count_occupied();
    println!("Result (Part 1): {:?}", num_occupied_part1);

    let mut seating_system = SeatingSystem::from_file(env::args().nth(1).unwrap(), 5, true);
    let num_occupied_part2 = seating_system.count_occupied();
    println!("Result (Part 2): {:?}", num_occupied_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let mut seating_system = SeatingSystem::from_file("example.txt", 4, false);
        assert_eq!(seating_system.count_occupied(), 37);

        let mut seating_system = SeatingSystem::from_file("example.txt", 5, true);
        assert_eq!(seating_system.count_occupied(), 26);
    }

    #[test]
    fn test_puzzle_input() {
        let mut seating_system = SeatingSystem::from_file("input.txt", 4, false);
        assert_eq!(seating_system.count_occupied(), 2468);

        let mut seating_system = SeatingSystem::from_file("input.txt", 5, true);
        assert_eq!(seating_system.count_occupied(), 2214);
    }
}
