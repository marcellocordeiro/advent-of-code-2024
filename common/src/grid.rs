use std::collections::HashSet;

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: i32,
    pub height: i32,
}

impl<T> Grid<T> {
    pub fn from(raw: Vec<Vec<T>>) -> Self {
        let height = raw.len() as i32;
        let width = raw[0].len() as i32; // Assumes all lines have the same width

        let data = raw.into_iter().flatten().collect();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        (point.x >= 0 && point.y >= 0) && ((point.x < self.width) && (point.y < self.height))
    }
}

impl<T: std::fmt::Display + Copy + Clone> Grid<T> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                let value = self[point];

                print!("{value}");
            }

            println!();
        }
    }

    pub fn print_with_marked(&self, marked: &HashSet<Point>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                let value = self[point];

                if marked.contains(&point) {
                    print!("{value}");
                } else {
                    print!(".");
                }
            }

            println!();
        }
    }
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let resolved_index = index.x + (index.y * self.width);

        &self.data[resolved_index as usize]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let resolved_index = index.x + (index.y * self.width);

        &mut self.data[resolved_index as usize]
    }
}
