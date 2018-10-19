extern crate num_complex;
extern crate nalgebra;
extern crate typenum;

pub use num_complex::Complex64 as Complex;
pub(crate) const SQRT_2_INVERSE: f64 = 0.70710678118;

mod bra;
mod ket;
mod outer;
#[cfg(test)] mod tests;

pub use bra::Bra;
pub use ket::Ket;
pub use outer::Outer;

pub type Bra2 = Bra<nalgebra::U2>;
pub type Ket2 = Ket<nalgebra::U2>;
