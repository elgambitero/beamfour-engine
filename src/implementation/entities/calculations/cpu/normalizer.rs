use crate::domain::entities::calculations::normalizer::Normalizer;
use crate::domain::entities::geometry::direction::Direction;

static EPSILON_MULT:f64 = 2f64;
struct NormalizerCPU{

}

impl Normalizer for NormalizerCPU{
    fn is_normalized(&self, v: &Direction) -> bool {
        let sum = self.module(&v);
        (sum - 1f64).abs() < EPSILON_MULT * f64::EPSILON
    }

    fn module(&self, dir: &Direction) -> f64 {
        (dir.u.powi(2) + dir.v.powi(2) + dir.w.powi(2)).sqrt()
    }


    fn normalize(&self, dir: &mut Direction) {
        let module = self.module(dir);
        dir.u = dir.u / module;
        dir.v = dir.v / module;
        dir.w = dir.w / module;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::geometry::vector::Vector;
    use crate::domain::entities::geometry::point::Point;

    #[test]
    fn is_normalized() {

        let norm = NormalizerCPU {};

        let single_x =
                Direction { u: 1f64, v: 0f64, w: 0f64 };
        let single_y =
                Direction { u: 0f64, v: 1f64, w: 0f64 };
        let norm_xyz =
                Direction::new(
                    1f64/(3f64.sqrt()),
                    1f64/(3f64.sqrt()),
                    1f64/(3f64.sqrt())
                );
        let norm_xyz_minus =
                Direction::new(
                    -1f64/(3f64.sqrt()),
                    -1f64/(3f64.sqrt()),
                    1f64/(3f64.sqrt())
                );
        let norm_xy = 
            Direction::new(
                1f64/(2f64.sqrt()),
                1f64/(2f64.sqrt()),
                0f64
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
                norm.is_normalized(&item),
                "{:?} was marked as not normalized, when it should",
                item
            )
        }

        let zero =
                Direction { u: 0f64, v: 0f64, w: 0f64};
        let diag_xy =
                Direction::new(1f64, 1f64, 0f64);
        let some_big_vector =
                Direction::new(2.53f64, 5.423f64, 1337.42f64);

        let not_valid =
            vec!(
                zero,
                diag_xy,
                some_big_vector
            );

        for item in not_valid {
            assert!(
                !norm.is_normalized(&item),
                "{:?} was marked as normalized when it shouldn't",
                item
            )
        }
    }

    #[test]
    fn normalize(){
        let normCPU = NormalizerCPU {};

        let single_x =
                Direction::new(1.0, 0.0, 0.0);
        let double_x =
                Direction::new(2.0, 0.0, 0.0);
        
        let mut inputs =
            vec!(
                single_x,
                double_x
            );

        for item in &mut inputs {
            normCPU.normalize(item)
        }

        let single_x_response =
                Direction::new(1.0, 0.0, 0.0);

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