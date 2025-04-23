pub type PointDataType = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub i: PointDataType,
    pub j: PointDataType,
}

pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

impl Point {
    pub const fn new(i: PointDataType, j: PointDataType) -> Self {
        Point { i, j }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self::new(self.i + rhs.i, self.j + rhs.j)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.i += rhs.i;
        self.j += rhs.j;
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::new(self.i - rhs.i, self.j - rhs.j)
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.i -= rhs.i;
        self.j -= rhs.j;
    }
}

impl std::ops::Mul<PointDataType> for Point {
    type Output = Self;

    /// Useful for specifying positions like `DOWN * 2`
    fn mul(self, rhs: PointDataType) -> Self::Output {
        Point::new(self.i * rhs, self.j * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let origin = Point::new(0, 0);

        assert!((origin + UP) == UP);
        assert!((origin + DOWN) == DOWN);
        assert!((origin + LEFT) == LEFT);
        assert!((origin + RIGHT) == RIGHT);
    }
}
