use std::collections::HashSet;

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
    let mut visited = HashSet::new();

    visit(map, start, start, &mut visited);

    visited.len()
}

fn visit(map: &Grid<i32>, position: Point, previous_position: Point, visited: &mut HashSet<Point>) {
    if map[position] == 9 {
        visited.insert(position);
        return;
    }

    //visited.insert(position);

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

    let mut check = |p: Point| {
        let value = map[p];

        if value == 0 {
            points.insert(p);
        }
    };

    for x in 0..map.width {
        check(Point::new(x, 0));
        check(Point::new(x, map.height - 1));
    }

    for y in 0..map.height {
        check(Point::new(0, y));
        check(Point::new(map.width - 1, y));
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

        assert_eq!(result, 36);
    }

    #[test]
    fn test_input() {
        let result = result(INPUT);

        assert_eq!(result, 0);
    }
}
