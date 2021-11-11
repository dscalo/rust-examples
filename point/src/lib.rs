use std::ops;

pub struct Point {
   pub x: i32,
   pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn distance_to(&self, other: &Point) -> i32 {
       (self.x - other.x).abs() + (self.y - other.y).abs()       
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

    #[test]
    fn distance_between_two_points_1() {
        let point_a = Point::new(0, 6);
        let point_b = Point::new(6, 0);

        assert_eq!(point_a.distance_to(&point_b), 12);
    }

    #[test]
    fn distance_between_two_points_2() {
        let point_a = Point::new(3, 4);
        let point_b = Point::new(2, 6);

        assert_eq!(point_a.distance_to(&point_b), 3);
    }
}
