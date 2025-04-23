use common::point::{DOWN, LEFT, Point, RIGHT, UP};
use itertools::Itertools;

use crate::parse_input;

pub fn result(input: &str) -> usize {
    let word_search = parse_input(input);

    let mut count = 0;

    let directions = [UP + RIGHT, DOWN + RIGHT, DOWN + LEFT, UP + LEFT];

    for j in 0..word_search.height {
        for i in 0..word_search.width {
            let point = Point::new(i, j);
            let value = word_search[point];

            if value != 'A' {
                continue;
            }

            let configuration = directions
                .iter()
                .map(|p| point + *p)
                .filter(|p| word_search.contains_point(*p))
                .map(|p| (p, word_search[p]))
                .collect_vec();

            let configuration_encoding = configuration.iter().map(|c| c.1).collect::<String>();

            let m_a_count = configuration_encoding
                .chars()
                .filter(|&c| c == 'M' || c == 'S')
                .count();

            if m_a_count != 4 {
                continue;
            }

            const POSSIBLE_ENCODINGS: [&str; 4] = ["MMSS", "SSMM", "SMMS", "MSSM"];

            if !POSSIBLE_ENCODINGS.contains(&configuration_encoding.as_str()) {
                continue;
            }

            count += 1;
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

        assert_eq!(result, 9);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1948);
    }
}
