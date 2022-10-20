use crate::domain::entities::geometry::direction::Direction;
use crate::domain::entities::geometry::point::Point;

pub trait Vector {
    fn new(p: Point, v: Direction) -> Self;

    fn is_normalized(&self) -> bool;

    fn normalize(&mut self);
}