use crate::domain::entities::geometry::vector::Vector;

pub enum LightBend{
    Refraction(Vector),
    Reflection(Vector)
}