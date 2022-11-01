use crate::domain::entities::calculations::propagator::Propagator;
use crate::domain::entities::geometry::{vector::Vector, point::Point, direction::Direction};

struct PropagatorCPU {}

impl Propagator for PropagatorCPU{
    fn propagate_mut(&self, v: &mut Vector, d: f64) {
        
    }

    fn propagate(&self, v: &Vector, d: f64) -> Vector {
        todo!()
    }
}

#[cfg(test)]
mod tests{
    use crate::domain::entities::geometry::{vector::Vector, point::Point, direction::Direction};

    use super::*;


    #[test]
    fn propagate_mut(){
        let prop = PropagatorCPU {};

        let mut origin_along_x = (
            10f64,
            Vector::new(
                Point {
                    x: 0.0, y: 0.0, z: 0.0
                },
                Direction{
                    u: 1.0, v: 0.0, w: 0.0
                }
            )
        );

        let mut inputs = vec![
            origin_along_x
        ];

        let origin_along_x_ans = Vector::new(
            Point {
                x: 10.0, y: 0.0, z: 0.0
            },
            Direction{
                u: 1.0, v: 0.0, w: 0.0
            }
        );

        let answers = vec![
            origin_along_x_ans
        ];

        let mut inputs_and_answers = inputs.iter_mut().zip(answers.iter());

        for item in inputs_and_answers {
            prop.propagate_mut(&mut item.0.1, item.0.0);

            
        }
    }

}