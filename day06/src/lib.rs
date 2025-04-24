use std::collections::HashSet;

use common::{grid::Grid, point::Point};
use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<char>,
    initial_position: Point,
    initial_direction: Point,
}

fn parse_input(input: &str) -> Input {
    let raw = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut initial_position = None;
    let mut initial_direction = None;

    let mut grid = Grid::from(raw);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let value = grid[point];

            let direction = Point::from_direction_char(value);

            if let Some(direction) = direction {
                initial_position = Some(point);
                initial_direction = Some(direction);

                grid[point] = '.';
            }
        }
    }

    let grid = grid;
    let initial_position = initial_position.unwrap();
    let initial_direction = initial_direction.unwrap();

    dbg!(&initial_position);
    dbg!(&initial_direction);

    Input {
        grid,
        initial_position,
        initial_direction,
    }
}

enum Action {
    MoveTo { position: Point, direction: Point },
    Finish,
}

fn next_state(grid: &Grid<char>, position: Point, direction: Point) -> Action {
    let next_position = position + direction;
    let within_grid = grid.contains_point(next_position);

    if !within_grid {
        return Action::Finish;
    }

    if grid[next_position] == '.' {
        Action::MoveTo {
            position: next_position,
            direction,
        }
    } else {
        // '#'
        Action::MoveTo {
            position,
            direction: direction.rotated_90(),
        }
    }
}

fn print_grid(grid: &Grid<char>, position: Point, direction: Point, marked: &HashSet<Point>) {
    dbg!(&position);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            let value = grid[point];

            if point == position {
                let dir_ch = direction.to_direction_char();
                print!("{dir_ch}");
            } else if marked.contains(&point) {
                print!("X");
            } else {
                print!("{value}");
            }
        }

        println!();
    }

    println!();
}
