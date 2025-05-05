use common::digits::{count_digits, split_digits_in_half};

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn iterate(stones: &mut Vec<usize>) {
    //let even_digits_count = stones.iter().filter(|n| n.len() % 2 == 0).count();

    let mut i = 0;

    while i < stones.len() {
        let value = &mut stones[i];

        if *value == 0 {
            *value = 1;
        } else if count_digits(*value) % 2 == 0 {
            let (a, b) = split_digits_in_half(*value);

            *value = a;
            stones.insert(i + 1, b);

            i += 1;
        } else {
            *value *= 2024;
        }

        i += 1;
    }
}
