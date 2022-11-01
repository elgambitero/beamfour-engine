use crate::domain::entities::geometry::direction::Direction;
use crate::domain::entities::geometry::point::Point;

#[derive(Debug)]
pub struct Vector {
    pub p: Point,
    pub v: Direction,
}

impl Vector{
    pub fn new(p: Point, v: Direction) -> Self {
        Vector {
            p, v
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.v == other.v
    }
}

#[cfg(test)]
mod tests{

    use crate::domain::entities::geometry::tolerance::TOL;

    use super::*;

    #[test]
    fn eq() {
        let identical_point_and_direction = (
            Vector::new(
                Point::new(0.0, 0.0, 0.0),
                Direction::new(1.0, 0.0, 0.0)
            ),
            Vector::new(
                Point::new(0.0, 0.0, 0.0),
                Direction::new(1.0, 0.0, 0.0)
            )
        );
        let identical_direction_point_within_tolerance = (
            Vector::new(
                Point::new(0.0 - TOL/2.0, 0.0, 0.0),
                Direction::new(1.0, 0.0, 0.0)
            ),
            Vector::new(
                Point::new(0.0, 0.0, 0.0),
                Direction::new(1.0, 0.0, 0.0)
            )
        );

        let equals = vec![
            identical_point_and_direction,
            identical_direction_point_within_tolerance
        ];

        for item in equals {
            assert_eq!(
                item.0, item.1,
                "{:?} and {:?} were supposed to be equal",
                item.0,
                item.1
            )
        }

        let point_within_tolerance_direction_outside_tolerance = (
            Vector::new(
                Point::new(0.0 + TOL/2.0, 0.0, 0.0),
                Direction::new(1.0 + TOL*2.0, 0.0, 0.0)
            ),
            Vector::new(
                Point::new(0.0, 0.0, 0.0),
                Direction::new(1.0, 0.0, 0.0)
            )
        );

        let differents = vec![
            point_within_tolerance_direction_outside_tolerance
        ];

        for item in differents {
            assert!(
                item.1 != item.0,
                "{:?} and {:?} were supposed to be different",
                item.0,
                item.1
            )
        }
    }
}
