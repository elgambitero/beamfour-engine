use crate::domain::entities::calculations::normalizer::Normalizer;
use crate::domain::entities::calculations::propagator::Propagator;
use crate::domain::entities::geometry::{vector::Vector};

use super::normalizer::NormalizerCPU;

struct PropagatorCPU<T: Normalizer> {
    norm: T
}

impl Propagator for PropagatorCPU<NormalizerCPU>{
    fn propagate_mut(&self, v: &mut Vector, d: f64) {
        self.norm.normalize(&mut v.v);
        v.p.x += v.v.u * d;
        v.p.y += v.v.v * d;
        v.p.z += v.v.w * d;
    }

    fn propagate(&self, v: &Vector, d: f64) -> Vector {
        let mut result = Vector::new(
            v.p,
            v.v
        );
        self.propagate_mut(&mut result, d);
        return result;
    }
}

#[cfg(test)]
mod tests{
    use crate::domain::entities::geometry::{vector::Vector, point::Point, direction::Direction};
    use crate::implementation::entities::calculations::cpu::normalizer::NormalizerCPU;

    use super::*;


    #[test]
    fn propagate_mut(){
        let prop = PropagatorCPU {
            norm: NormalizerCPU {}
        };

        let origin_along_x = (
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
        let origin_along_y = (
            10f64,
            Vector::new(
                Point {
                    x: 0.0, y: 0.0, z: 0.0
                },
                Direction{
                    u: 0.0, v: 1.0, w: 0.0
                }
            )
        );
        let not_normalized_y = (
            10f64,
            Vector::new(
                Point {
                    x: 0.0, y: 0.0, z: 0.0
                },
                Direction{
                    u: 0.0, v: 2.0, w: 0.0
                }
            )
        );

        let mut inputs = vec![
            origin_along_x,
            origin_along_y,
            not_normalized_y
        ];

        let origin_along_x_ans = Vector::new(
            Point {
                x: 10.0, y: 0.0, z: 0.0
            },
            Direction{
                u: 1.0, v: 0.0, w: 0.0
            }
        );
        let origin_along_y_ans = Vector::new(
            Point {
                x: 0.0, y: 10.0, z: 0.0
            },
            Direction{
                u: 0.0, v: 1.0, w: 0.0
            }
        );
        let not_normalized_y_ans = Vector::new(
            Point {
                x: 0.0, y: 10.0, z: 0.0
            },
            Direction{
                u: 0.0, v: 1.0, w: 0.0
            }
        );

        let answers = vec![
            origin_along_x_ans,
            origin_along_y_ans,
            not_normalized_y_ans
        ];

        let inputs_and_answers = inputs.iter_mut().zip(answers.iter());

        for item in inputs_and_answers {
            prop.propagate_mut(&mut item.0.1, item.0.0);

            assert_eq!(
                &item.0.1,
                item.1,
                "Supposed to end up like {:?}, but was {:?}",
                item.1,
                item.0.1
            )
        }
    }

    #[test]
    fn propagate() {
        let prop = PropagatorCPU {
            norm: NormalizerCPU {}
        };

        let origin_along_x = (
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
        let origin_along_y = (
            10f64,
            Vector::new(
                Point {
                    x: 0.0, y: 0.0, z: 0.0
                },
                Direction{
                    u: 0.0, v: 1.0, w: 0.0
                }
            )
        );
        let not_normalized_y = (
            10f64,
            Vector::new(
                Point {
                    x: 0.0, y: 0.0, z: 0.0
                },
                Direction{
                    u: 0.0, v: 2.0, w: 0.0
                }
            )
        );

        let mut inputs = vec![
            origin_along_x,
            origin_along_y,
            not_normalized_y
        ];

        let origin_along_x_ans = Vector::new(
            Point {
                x: 10.0, y: 0.0, z: 0.0
            },
            Direction{
                u: 1.0, v: 0.0, w: 0.0
            }
        );
        let origin_along_y_ans = Vector::new(
            Point {
                x: 0.0, y: 10.0, z: 0.0
            },
            Direction{
                u: 0.0, v: 1.0, w: 0.0
            }
        );
        let not_normalized_y_ans = Vector::new(
            Point {
                x: 0.0, y: 10.0, z: 0.0
            },
            Direction{
                u: 0.0, v: 1.0, w: 0.0
            }
        );

        let answers = vec![
            origin_along_x_ans,
            origin_along_y_ans,
            not_normalized_y_ans
        ];

        let inputs_and_answers = inputs.iter_mut().zip(answers.iter());

        for item in inputs_and_answers {
            let answer = prop.propagate(&mut item.0.1, item.0.0);

            assert_eq!(
                &answer,
                item.1,
                "Supposed to end up like {:?}, but was {:?}",
                item.1,
                item.0.1
            )
        }
    }

}