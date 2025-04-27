use std::collections::{HashMap, HashSet};

use common::{
    grid::Grid,
    point::{DOWN, LEFT, Point, RIGHT, UP},
};

use crate::parse_input;

pub fn result(input: &str) -> usize {
    let map = parse_input(input);

    possible_start_points(&map)
        .into_iter()
        .map(|p| score(&map, p))
        .sum()
}

fn score(map: &Grid<i32>, start: Point) -> usize {
    let mut visited = HashMap::new();

    visit(map, start, start, &mut visited);

    visited.values().copied().sum()
}

fn visit(
    map: &Grid<i32>,
    position: Point,
    previous_position: Point,
    visited: &mut HashMap<Point, usize>,
) {
    if map[position] == 9 {
        let count = visited.entry(position).or_default();
        *count += 1;
        return;
    }

    let current_height = map[position];

    for dir in [UP, DOWN, LEFT, RIGHT] {
        let next = position + dir;

        if next == previous_position {
            continue;
        }

        if !map.contains_point(next) {
            continue;
        }

        if map[next] != current_height + 1 {
            continue;
        }

        visit(map, next, position, visited);
    }
}

fn possible_start_points(map: &Grid<i32>) -> HashSet<Point> {
    let mut points = HashSet::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);

            if map[point] == 0 {
                points.insert(point);
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let result = result(SAMPLE);

        assert_eq!(result, 81);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 1960);
    }
}
