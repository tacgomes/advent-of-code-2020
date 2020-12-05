use std::cmp;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
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
    for c in seat.bytes() {
        let mid = lower + (upper - lower) / 2;
        match c as char {
            'F' | 'L' => upper = mid,
            'B' | 'R' => lower = mid + 1,
            _ => unreachable!(),
        }
    }
    lower
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let lines = BufReader::new(file).lines();

    let mut highest_seat_id = 0;
    let mut seats_occupied = [false; ROWS * COLS];

    for line in lines {
        let line = line.unwrap();
        let row = seat_row(&line[..COLS - 1]);
        let col = seat_col(&line[COLS - 1..]);
        let seat_id = seat_id(row, col);
        highest_seat_id = cmp::max(highest_seat_id, seat_id);
        seats_occupied[seat_id] = true;
    }

    let first_occupied = seats_occupied.iter().position(|&x| x == true).unwrap();
    let first_free = seats_occupied
        .iter()
        .skip(first_occupied)
        .position(|&x| x == false)
        .unwrap();

    println!("Result (part 1): {}", highest_seat_id);
    println!("Result (part 2): {}", first_occupied + first_free);
}
