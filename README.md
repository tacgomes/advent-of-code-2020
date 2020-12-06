# Advent of Code 2020

![Rust workflow](https://github.com/tacgomes/AoC-2020/workflows/Rust/badge.svg)

Solutions to the [Advent of Code], 2020 edition.

## Building and Running

You can either build and run the programs with `cargo`, or build directly with
`rustc` as no external packages are used.

Following are the instructions to build and run the program to solve the _Part
1_ of the _Puzzle 01_, with and without `cargo`.

1. With Cargo:

```sh
cargo build
cargo run --bin 01-part1 01/input.txt
```

2. Without Cargo:

```sh
cd 01
rustc part1.rs
./part1 input.txt
```

[Advent of Code]: https://adventofcode.com/2020/about
