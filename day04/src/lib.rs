use common::grid::Grid;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Grid<char> {
    Grid::from(input.lines().map(|line| line.chars().collect()).collect())
}
