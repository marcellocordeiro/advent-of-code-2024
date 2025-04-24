use common::point::{DOWN, LEFT, Point, RIGHT, UP};
use itertools::Itertools;

use crate::parse_input;

pub fn result(input: &str) -> usize {
    let word_search = parse_input(input);

    let mut count = 0;

    let directions = [
        UP,
        UP + RIGHT,
        RIGHT,
        DOWN + RIGHT,
        DOWN,
        DOWN + LEFT,
        LEFT,
        UP + LEFT,
    ];

    for y in 0..word_search.height {
        for x in 0..word_search.width {
            let point = Point::new(x, y);
            let value = word_search[point];

            if value != 'X' && value != 'S' {
                //print!("{value} ");
                continue;
            }

            let string_for =
                |points: [Point; 4]| points.iter().map(|p| word_search[*p]).collect::<String>();

            let words = directions
                .iter()
                .map(|&dir| {
                    [
                        point,
                        point + (dir * 1),
                        point + (dir * 2),
                        point + (dir * 3),
                    ]
                })
                .filter(|points| points.iter().all(|&p| word_search.contains_point(p)))
                .map(|points| (points, string_for(points)))
                .collect::<Vec<_>>();

            if words.is_empty() {
                continue;
            }

            let result = words
                .into_iter()
                .filter(|(_, word)| word == "XMAS")
                .collect_vec();

            count += result.len();
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 2599);
    }
}
