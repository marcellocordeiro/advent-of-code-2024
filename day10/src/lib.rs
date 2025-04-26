use common::grid::Grid;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Grid<i32> {
    let raw = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    Grid::from(raw)
}
