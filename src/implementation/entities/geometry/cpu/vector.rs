use crate::domain::entities::geometry::vector::Vector;
use crate::domain::entities::geometry::point::Point;
use crate::domain::entities::geometry::direction::Direction;

static EPSILON_MULT:f64 = 2f64;

#[derive(Debug, Clone, Copy)]
pub struct VectorCPU {
    p: Point,
    v: Direction,
}

impl VectorCPU{
    fn module(&self) -> f64 {
        (self.v.u.powi(2) + self.v.v.powi(2) + self.v.w.powi(2)).sqrt()
    }
}

impl Vector for VectorCPU {
    fn new(p: Point, v: Direction) -> Self {
        VectorCPU {
            p, v
        }
    }

    fn is_normalized(&self) -> bool {
        let sum = self.module();
        (sum - 1f64).abs() < EPSILON_MULT * f64::EPSILON
    }

    fn normalize(&mut self) {
        let module = self.module();
        self.v.u = self.v.u / module;
        self.v.v = self.v.v / module;
        self.v.w = self.v.w / module;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_normalized() {

        let any_point = Point::new( 0.0, 0.0, 0.0);

        let single_x =
            VectorCPU::new(
                any_point, 
                Direction { u: 1f64, v: 0f64, w: 0f64 }
            );
        let single_y =
            VectorCPU::new(
                any_point, 
                Direction { u: 0f64, v: 1f64, w: 0f64 }
                );
        let norm_xyz =
            VectorCPU::new(
                any_point, 
                Direction::new(
                    1f64/(3f64.sqrt()),
                    1f64/(3f64.sqrt()),
                    1f64/(3f64.sqrt())
                )
            );
        let norm_xyz_minus =
            VectorCPU::new(
                any_point, 
                Direction::new(
                    -1f64/(3f64.sqrt()),
                    -1f64/(3f64.sqrt()),
                    1f64/(3f64.sqrt())
                )
            );
        let norm_xy = VectorCPU::new(
            any_point,
            Direction::new(
                1f64/(2f64.sqrt()),
                1f64/(2f64.sqrt()),
                0f64
            )
        );

        let valid =
            vec!(
                single_x,
                single_y,
                norm_xyz,
                norm_xyz_minus,
                norm_xy
            );

        for item in valid {
            assert!(
                item.is_normalized(),
                "{:?} was marked as not normalized, when it should",
                item
            )
        }

        let zero =
            VectorCPU::new(
                any_point,
                Direction { u: 0f64, v: 0f64, w: 0f64}
            );
        let diag_xy =
            VectorCPU::new(
                any_point,
                Direction::new(1f64, 1f64, 0f64)
            );
        let some_big_vector =
            VectorCPU::new(
                any_point,
                Direction::new(2.53f64, 5.423f64, 1337.42f64)
            );

        let not_valid =
            vec!(
                zero,
                diag_xy,
                some_big_vector
            );

        for item in not_valid {
            assert!(
                !item.is_normalized(),
                "{:?} was marked as normalized when it shouldn't",
                item
            )
        }
    }

    #[test]
    fn normalize(){
        let origin = Point::new(0.0, 0.0,0.0);
        let single_x =
            VectorCPU::new(
                origin,
                Direction::new(1.0, 0.0, 0.0)
            );
        let double_x =
            VectorCPU::new(
                origin,
                Direction::new(2.0, 0.0, 0.0)
            );
        
        let mut inputs =
            vec!(
                single_x,
                double_x
            );

        for item in &mut inputs {
            item.normalize()
        }

        let single_x_response =
            VectorCPU::new(
                origin,
                Direction::new(1.0, 0.0, 0.0)
            );

        let responses =
            vec!(
                single_x_response,
                single_x_response
            );

        let inputs_and_responses = inputs.iter().zip(responses);

        for (i, r) in inputs_and_responses {
            assert_eq!(
                i.v, r.v
            )
        }
    }

}