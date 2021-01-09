use std::env;
use std::fs;
use std::path::Path;
use std::process;

const ROWS: usize = 128;
const COLS: usize = 8;

fn seat_id(row: usize, col: usize) -> usize {
    row * COLS + col
}

fn seat_row(seat: &str) -> usize {
    find_seat(seat, 0, ROWS - 1)
}

fn seat_col(seat: &str) -> usize {
    find_seat(seat, 0, COLS - 1)
}

fn find_seat(seat: &str, mut lower: usize, mut upper: usize) -> usize {
    for c in seat.chars() {
        let mid = lower + (upper - lower) / 2;
        match c {
            'F' | 'L' => upper = mid,
            'B' | 'R' => lower = mid + 1,
            _ => unreachable!(),
        }
    }
    lower
}

fn solve(file_name: impl AsRef<Path>) -> (usize, usize) {
    let content = fs::read_to_string(&file_name).unwrap();

    let mut seats = [false; ROWS * COLS];

    for line in content.lines() {
        let row = seat_row(&line[..COLS - 1]);
        let col = seat_col(&line[COLS - 1..]);
        let seat_id = seat_id(row, col);
        seats[seat_id] = true;
    }

    let highest = seats.len() - 1 - seats.iter().rev().position(|&x| x).unwrap();
    let first_occupied = seats.iter().position(|&x| x).unwrap();
    let first_free = seats.iter().skip(first_occupied).position(|&x| !x).unwrap();

    (highest, first_occupied + first_free)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let (highest_seat_id, free_seat_id) = solve(env::args().nth(1).unwrap());
    println!("Result (Part 1): {}", highest_seat_id);
    println!("Result (Part 2): {}", free_seat_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(solve("example.txt"), (357, 358));
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(solve("input.txt"), (991, 534));
    }
}
