use super::tolerance::TOL;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {x, y, z}
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < TOL &&
        (self.y - other.y).abs() < TOL &&
        (self.z - other.z).abs() < TOL
    }
}


    

#[cfg(test)]
mod tests{
    use crate::domain::entities::geometry::tolerance::TOL;

    use super::*;


    #[test]
    fn eq() {
        let exactly_equal = (
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0)
        );
        let within_tolerance_x = (
            Point::new(0.0 + TOL/2.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0)
        );
        let within_tolerance_y = (
            Point::new(0.0, 0.0 + TOL/2.0, 0.0),
            Point::new(0.0, 0.0, 0.0)
        );
        let within_tolerance_z = (
            Point::new(0.0, 0.0, 0.0 + TOL/2.0),
            Point::new(0.0, 0.0, 0.0)
        );
        let beyond_tolerance_diagonally_but_not_per_coordinate = (
            Point::new(0.0 + TOL*0.998, 0.0 + TOL*0.998, 0.0 + TOL*0.998),
            Point::new(0.0, 0.0, 0.0)
        );

        let equals = vec![
            exactly_equal,
            within_tolerance_x,
            within_tolerance_y,
            within_tolerance_z,
            beyond_tolerance_diagonally_but_not_per_coordinate
        ];

        for item in equals {
            assert!(
                item.1 == item.0,
                "{:?} and {:?} were supposed to be equal",
                item.0,
                item.1
            )
        }

        let completely_different = (
            Point::new(1.2, 3.4, 5.6),
            Point::new(7.8, 9.0, 10.0)
        );

        let differents = vec![
            completely_different
        ];

        for item in differents {
            assert!(
                item.0 != item.1,
                "{:?} and {:?} were supposed to be different",
                item.0,
                item.1
            )
        }
    }
}