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
/// 2-dimension outer
pub type Outer2 = Outer<nalgebra::U2>;
/// 4-dimension bra (for entangled states)
pub type Bra4 = Bra<nalgebra::U4>;
/// 4-dimension ket (for entangled states)
pub type Ket4 = Ket<nalgebra::U4>;
/// 4-dimension ket (for entangled states)
pub type Outer4 = Outer<nalgebra::U4>;