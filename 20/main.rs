use std::env;
use std::fs;
use std::path::Path;
use std::process;


struct _Tile {
    tile_id: usize,
    borders: [Vec<char>; 4],
}


fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();
    let tiles: Vec<_> = content.split("\n\n").collect();

    for tile in tiles {
        let mut lines = tile.split('\n');
        let tile_id = &lines.next().unwrap();
        let tile_id = &tile_id[5..tile_id.len() - 1].parse::<usize>().unwrap();
        println!("\ntileid: {}", tile_id);

        let mut borders = vec![vec![]; 4];

        borders[0] = lines.clone().next().unwrap().chars().collect::<Vec<_>>();
        borders[2] = lines.clone().last().unwrap().chars().collect::<Vec<_>>();
        for line in lines {
            borders[3].push(line.chars().next().unwrap());
            borders[1].push(line.chars().last().unwrap());
        }

        println!("top: {:?}", borders[0]);
        println!("right: {:?}", borders[1]);
        println!("bottom: {:?}", borders[2]);
        println!("left: {:?}", borders[3]);
    }

    0
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = calculate_part1(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_example_input() {
        assert_eq!(calculate_part1("example.txt"), 0);
    }

    #[test]
    #[ignore]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 0);
    }
}
