use super::tolerance::TOL;

#[derive(Debug, Clone, Copy)]
pub struct Direction{
    pub u: f64,
    pub v: f64,
    pub w: f64
}

impl Direction{
    pub fn new(u: f64, v: f64, w: f64) -> Direction{
        Direction { u, v, w }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        (self.u - other.u).abs() < TOL &&
        (self.v - other.v).abs() < TOL &&
        (self.w - other.w).abs() < TOL
    }
}



#[cfg(test)]
mod tests {

    use crate::domain::entities::geometry::tolerance::TOL;

    use super::*;

    #[test]
    fn eq() {
        let exactly_equal = (
            Direction::new(0.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 0.0)
        );
        let within_tolerance_x = (
            Direction::new(0.0 + TOL/2.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 0.0)
        );
        let within_tolerance_y = (
            Direction::new(0.0, 0.0 + TOL/2.0, 0.0),
            Direction::new(0.0, 0.0, 0.0)
        );
        let within_tolerance_z = (
            Direction::new(0.0, 0.0, 0.0 + TOL/2.0),
            Direction::new(0.0, 0.0, 0.0)
        );
        let beyond_tolerance_diagonally_but_not_per_coordinate = (
            Direction::new(0.0 + TOL*0.998, 0.0 + TOL*0.998, 0.0 + TOL*0.998),
            Direction::new(0.0, 0.0, 0.0)
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
            Direction::new(1.2, 3.4, 5.6),
            Direction::new(7.8, 9.0, 10.0)
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