use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    FilePart(usize),
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .trim()
        .chars()
        .chain(['0']) // No free space at the end (will be ignored otherwise)
        .tuples()
        .enumerate()
        .flat_map(|(id, (allocated, free_space))| {
            let allocated = allocated.to_digit(10).unwrap() as usize;
            let free_space = free_space.to_digit(10).unwrap() as usize;

            assert!(allocated > 0);

            let allocated_blocks = [Block::FilePart(id)].repeat(allocated);
            let free_blocks = [Block::Free].repeat(free_space);

            [allocated_blocks, free_blocks].concat()
        })
        .collect()
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| match *block {
            Block::FilePart(id) => Some(index * id),
            Block::Free => None,
        })
        .sum()
}

fn blocks_to_string(blocks: &[Block]) -> String {
    blocks.iter().fold(String::new(), |acc, block| {
        acc + (&match block {
            Block::FilePart(id) => format!("{id}"),
            Block::Free => ".".to_string(),
        })
    })
}

#[allow(dead_code)]
fn print_blocks(blocks: &[Block]) {
    println!("{}", blocks_to_string(blocks));
}
