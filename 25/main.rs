use std::env;
use std::process;

fn find_encryption_key(door_pub_key: &str, card_pub_key: &str) -> usize {
    let door_pub_key = door_pub_key.parse::<usize>().unwrap();
    let card_pub_key = card_pub_key.parse::<usize>().unwrap();

    let pub_key1;
    let pub_key2;
    if door_pub_key % 7 == 0 {
        pub_key1 = door_pub_key;
        pub_key2 = card_pub_key;
    } else {
        pub_key1 = card_pub_key;
        pub_key2 = door_pub_key;
    }

    let mut value = 1;
    let mut loop_size = 0;

    while value != pub_key1 {
        value *= 7;
        value %= 20201227;
        loop_size += 1;
    }

    value = 1;
    for _ in 0..loop_size {
        value *= pub_key2;
        value %= 20201227;
    }

    value
}

fn main() {
    if env::args().count() != 3 {
        eprintln!(
            "USAGE: {} DOOR_PUB_KEY CARD_PUB_KEY",
            env::args().next().unwrap()
        );
        process::exit(1);
    }

    let encryption_key =
        find_encryption_key(&env::args().nth(1).unwrap(), &env::args().nth(2).unwrap());
    println!("Result: {}", encryption_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(find_encryption_key("5764801", "17807724"), 14897079);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(find_encryption_key("11349501", "5107328"), 7936032);
    }
}
