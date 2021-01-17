use std::env;
use std::process;

fn discrete_logarithm(power: usize, base: usize, modulus: usize) -> usize {
    /* Solve the following congruence:
     *
     * base^exponent ≡ power (mod modulus)
     *
     * A possible improvement would be finding the discrete logarithm
     * using the baby-step giant-step algorithm, but this brute-force
     * approach already runs fast enough for the input.
     */
    let mut num = 1;
    let mut exponent = 0;
    while num != power {
        num = num * base % modulus;
        exponent += 1;
    }
    exponent
}

fn find_encryption_key(door_pub_key: &str, card_pub_key: &str) -> usize {
    let door_pub_key = door_pub_key.parse::<usize>().unwrap();
    let card_pub_key = card_pub_key.parse::<usize>().unwrap();
    let loop_size = discrete_logarithm(door_pub_key, 7, 20201227);
    (0..loop_size).fold(1, |acc, _| acc * card_pub_key % 20201227)
}

fn main() {
    if env::args().count() != 3 {
        eprintln!(
            "USAGE: {} DOOR_PUB_KEY CARD_PUB_KEY",
            env::args().next().unwrap()
        );
        process::exit(1);
    }

    let door_pub_key = env::args().nth(1).unwrap();
    let card_pub_key = env::args().nth(2).unwrap();
    let encryption_key = find_encryption_key(&door_pub_key, &card_pub_key);
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
