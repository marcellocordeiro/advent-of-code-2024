use std::collections::HashSet;

use common::point::Point;

use crate::{Action, Input, next_state, parse_input};

pub fn result(input: &str) -> usize {
    let input = parse_input(input);

    let starting_position = input.initial_position;

    let height = input.grid.height;
    let width = input.grid.width;

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            let obstruction = Point::new(x, y);

            if obstruction == starting_position {
                continue;
            }

            if input.grid[obstruction] == '#' {
                continue;
            }

            let result = with_obstruction(input.clone(), obstruction);

            if result {
                count += 1;
            }
        }
    }

    count
}

fn with_obstruction(input: Input, obstruction: Point) -> bool {
    let Input {
        mut grid,
        initial_position,
        initial_direction,
    } = input;

    grid[obstruction] = '#';

    let mut position = initial_position;
    let mut direction = initial_direction;

    let mut marked = HashSet::new();

    marked.insert((position, direction));

    loop {
        let action = next_state(&grid, position, direction);

        match action {
            Action::Finish => return false,
            Action::MoveTo {
                position: next_position,
                direction: next_direction,
            } => {
                let entry = (next_position, next_direction);

                if marked.contains(&entry) {
                    return true;
                }

                marked.insert(entry);

                position = next_position;
                direction = next_direction;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1480);
    }
}
