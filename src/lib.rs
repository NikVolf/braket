//! Strongly-typed bra-ket notation for discrete quantum objects.

#![warn(missing_docs)]

extern crate num_complex;
extern crate nalgebra;
extern crate typenum;

#[cfg(test)] extern crate assert_approx_eq;


pub use num_complex::Complex64 as Complex;
pub(crate) const SQRT_2_INVERSE: f64 = std::f64::consts::FRAC_1_SQRT_2;

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