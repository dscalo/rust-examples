use std::ops;

pub struct Point {
   pub x: i32,
   pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

// owerloading + operator
impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn adding_two_points() {
        let point_a = Point::new(1, 2);
        let point_b = Point::new(3, 4);

        let result =point_a + point_b;

        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }
}
