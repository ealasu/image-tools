#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RaDec {
    pub ra: f64,
    pub dec: f64,
}

impl Default for RaDec {
    fn default() -> Self {
        RaDec {
            ra: 0.0,
            dec: 0.0
        }
    }
}
