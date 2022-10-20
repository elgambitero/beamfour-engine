#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction{
    pub u: f64,
    pub v: f64,
    pub w: f64
}

impl Direction{
    pub fn new(u: f64, v: f64, w: f64) -> Direction{
        Direction { u, v, w }
    }
}
