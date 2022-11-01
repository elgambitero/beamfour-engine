use crate::domain::entities::geometry::vector::Vector;

pub trait Propagator {
    fn propagate_mut(&self, v: &mut Vector, d: f64);
    fn propagate(&self, v: &Vector, d: f64) -> Vector;
}