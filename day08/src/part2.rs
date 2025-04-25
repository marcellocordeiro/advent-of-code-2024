use std::collections::HashSet;

use itertools::Itertools;

use crate::parse_input;

pub fn result(input: &str) -> usize {
    let input = parse_input(input);

    let mut antinodes = HashSet::new();

    for (_, points) in input.antenna_map {
        for ab in points.iter().combinations(2) {
            let a = *ab[0];
            let b = *ab[1];

            let diff = b - a;

            for k in 2.. {
                let antinode_1 = a + (diff * k);
                let antinode_2 = b - (diff * k);

                antinodes.insert(a);
                antinodes.insert(b);

                let contains_1 = input.grid.contains_point(antinode_1);
                let contains_2 = input.grid.contains_point(antinode_2);

                if contains_1 {
                    antinodes.insert(antinode_1);
                }

                if contains_2 {
                    antinodes.insert(antinode_2);
                }

                if !contains_1 && !contains_2 {
                    break;
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 34);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1229);
    }
}
