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
