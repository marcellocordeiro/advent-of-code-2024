pub type PointDataType = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: PointDataType,
    pub y: PointDataType,
}

// TODO: This should be a direction instead
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

impl Point {
    pub const fn new(x: PointDataType, y: PointDataType) -> Self {
        Self { x, y }
    }

    pub const fn from_direction_char(ch: char) -> Option<Self> {
        Some(match ch {
            '^' => UP,
            'v' => DOWN,
            '<' => LEFT,
            '>' => RIGHT,

            _ => return None,
        })
    }

    pub const fn to_direction_char(self) -> char {
        match self {
            UP => '^',
            DOWN => 'v',
            LEFT => '<',
            RIGHT => '>',

            _ => panic!("Invalid argument"),
        }
    }

    #[must_use]
    pub const fn rotated_90(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<PointDataType> for Point {
    type Output = Self;

    /// Useful for specifying positions like `DOWN * 2`
    fn mul(self, rhs: PointDataType) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
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

    #[test]
    fn test_rotated_90() {
        let starting = UP;

        assert!(starting.rotated_90() == RIGHT);
        assert!(starting.rotated_90().rotated_90() == DOWN);
        assert!(starting.rotated_90().rotated_90().rotated_90() == LEFT);
        assert!(starting.rotated_90().rotated_90().rotated_90().rotated_90() == UP);
    }
}
