use std::collections::HashMap;

use common::{grid::Grid, point::Point};

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

struct Input {
    grid: Grid<char>,
    antenna_map: HashMap<char, Vec<Point>>,
}

fn parse_input(input: &str) -> Input {
    let raw = input.lines().map(|line| line.chars().collect()).collect();

    let grid = Grid::from(raw);
    let mut antenna_map: HashMap<char, Vec<Point>> = HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let value = grid[point];

            if value != '.' {
                antenna_map.entry(value).or_default().push(point);
            }
        }
    }

    Input { grid, antenna_map }
}
