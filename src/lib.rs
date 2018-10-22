//! Strongly-typed bra-ket notation for discrete quantum objects.

#![warn(missing_docs)]

extern crate num_complex;
extern crate nalgebra;
extern crate typenum;

pub use num_complex::Complex64 as Complex;
pub(crate) const SQRT_2_INVERSE: f64 = 0.707106781186547524400844362104849039284835937688474036588;

mod bra;
mod ket;
mod outer;
#[cfg(test)] mod tests;

pub use bra::Bra;
pub use ket::Ket;
pub use outer::Outer;

/// 2-dimension bra
pub type Bra2 = Bra<nalgebra::U2>;
/// 2-dimension ket
pub type Ket2 = Ket<nalgebra::U2>;
