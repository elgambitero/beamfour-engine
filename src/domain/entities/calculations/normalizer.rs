use crate::domain::entities::geometry::direction::Direction;

pub trait Normalizer {
    fn is_normalized(&self, v: &Direction) -> bool;
    fn normalize(&self, v: &mut Direction);
    fn module(&self, v: &Direction) -> f64;
}