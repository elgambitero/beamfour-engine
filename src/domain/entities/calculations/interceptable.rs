use crate::domain::entities::geometry::vector::Vector;

pub trait Interceptable {
    fn intercept(vector: Vector) -> f64;
}