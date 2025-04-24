use std::collections::HashSet;

use crate::{Action, Input, next_state, parse_input};

pub fn result(input: &str) -> usize {
    let Input {
        grid,
        initial_position,
        initial_direction,
    } = parse_input(input);

    let mut position = initial_position;
    let mut direction = initial_direction;

    let mut marked = HashSet::new();

    marked.insert(position);

    loop {
        let action = next_state(&grid, position, direction);

        match action {
            Action::Finish => break,
            Action::MoveTo {
                position: next_position,
                direction: next_direction,
            } => {
                marked.insert(next_position);

                position = next_position;
                direction = next_direction;
            }
        }
    }

    marked.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 41);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 4580);
    }
}
