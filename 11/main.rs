use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Clone, Copy)]
struct Move(isize, isize);

const MOVES: [Move; 8] = [
    Move(-1, -1),
    Move(-1, 0),
    Move(-1, 1),
    Move(0, 1),
    Move(1, 1),
    Move(1, 0),
    Move(1, -1),
    Move(0, -1),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum SeatStatus {
    Empty,
    Occupied,
    Floor,
}

struct SeatingSystem {
    seats: Vec<Vec<SeatStatus>>,
    nrows: isize,
    ncols: isize,
    swap_threshold: u32,
    recurse: bool,
}

impl SeatingSystem {
    fn new(seats: Vec<Vec<SeatStatus>>, swap_threshold: u32, recurse: bool) -> Self {
        SeatingSystem {
            nrows: seats.len() as isize,
            ncols: seats[0].len() as isize,
            seats,
            swap_threshold,
            recurse,
        }
    }

    fn count_occupied(&mut self) -> usize {
        while self.iterate() {}
        self.seats
            .iter()
            .flat_map(|x| x.iter())
            .filter(|&&x| x == SeatStatus::Occupied)
            .count()
    }

    fn iterate(&mut self) -> bool {
        let mut copy = self.seats.clone();
        let mut changed = false;

        for (r, row) in copy.iter_mut().enumerate() {
            for (c, seat_status) in row.iter_mut().enumerate() {
                match seat_status {
                    SeatStatus::Empty => {
                        let num_occupied = self.count_occupied_neighbors(r, c);
                        if num_occupied == 0 {
                            *seat_status = SeatStatus::Occupied;
                            changed = true;
                        }
                    }
                    SeatStatus::Occupied => {
                        let num_occupied = self.count_occupied_neighbors(r, c);
                        if num_occupied >= self.swap_threshold {
                            *seat_status = SeatStatus::Empty;
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

    fn count_occupied_neighbors(&self, row: usize, col: usize) -> u32 {
        MOVES
            .iter()
            .map(|&x| self.check_seat(row as isize, col as isize, x))
            .sum()
    }

    fn check_seat(&self, row: isize, col: isize, mov: Move) -> u32 {
        let (row, col) = (row + mov.0, col + mov.1);
        if row < 0 || col < 0 || row == self.nrows || col == self.ncols {
            return 0;
        }

        match self.seats[row as usize][col as usize] {
            SeatStatus::Empty => 0,
            SeatStatus::Occupied => 1,
            SeatStatus::Floor => match self.recurse {
                true => self.check_seat(row, col, mov),
                false => 0,
            },
        }
    }
}

fn parse_input(file_name: impl AsRef<Path>) -> Vec<Vec<SeatStatus>> {
    fs::read_to_string(&file_name)
        .unwrap()
        .lines()
        .map(|x| {
            x.chars()
                .map(|ch| match ch {
                    'L' => SeatStatus::Empty,
                    '#' => SeatStatus::Occupied,
                    '.' => SeatStatus::Floor,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let seats = parse_input(env::args().nth(1).unwrap());

    let mut seating_system = SeatingSystem::new(seats.clone(), 4, false);
    let num_occupied_part1 = seating_system.count_occupied();
    println!("Result (Part 1): {:?}", num_occupied_part1);

    let mut seating_system = SeatingSystem::new(seats, 5, true);
    let num_occupied_part2 = seating_system.count_occupied();
    println!("Result (Part 2): {:?}", num_occupied_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let seats = parse_input("example.txt");
        let mut seating_system = SeatingSystem::new(seats.clone(), 4, false);
        assert_eq!(seating_system.count_occupied(), 37);

        let mut seating_system = SeatingSystem::new(seats, 5, true);
        assert_eq!(seating_system.count_occupied(), 26);
    }

    #[test]
    fn test_puzzle_input() {
        let seats = parse_input("input.txt");
        let mut seating_system = SeatingSystem::new(seats.clone(), 4, false);
        assert_eq!(seating_system.count_occupied(), 2468);

        let mut seating_system = SeatingSystem::new(seats, 5, true);
        assert_eq!(seating_system.count_occupied(), 2214);
    }
}
