use crate::point::Point;

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: i32,
    pub height: i32,
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let resolved_index = index.i + (index.j * self.width);

        &self.data[resolved_index as usize]
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let resolved_index = index.i + (index.j * self.width);

        &mut self.data[resolved_index as usize]
    }
}
