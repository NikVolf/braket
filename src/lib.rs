extern crate num_complex;

use std::ops::Mul;
use num_complex::Complex64 as Complex;

pub struct Bra {
    a: Complex,
    b: Complex,
}

impl Bra {
    pub fn up() -> Self {
        Bra {
            a: Complex::new(1.0, 0.0),
            b: Complex::new(0.0, 0.0),
        }
    }

    pub fn down() -> Self {
        Bra {
            a: Complex::new(0.0, 0.0),
            b: Complex::new(1.0, 0.0),
        }
    }

    pub fn right() -> Self {
        Bra {
            a: Complex::new(0.70710678118, 0.0),
            b: Complex::new(0.70710678118, 0.0),
        }
    }

    pub fn left() -> Self {
        Bra {
            a: Complex::new(0.70710678118, 0.0),
            b: Complex::new(-0.70710678118, 0.0),
        }
    }
}

pub struct Ket {
    a: Complex,
    b: Complex,
}

impl Ket {
    pub fn up() -> Self {
        Ket {
            a: Complex::new(1.0, 0.0),
            b: Complex::new(0.0, 0.0)
        }
    }

    pub fn down() -> Self {
        Ket {
            a: Complex::new(0.0, 0.0),
            b: Complex::new(1.0, 0.0)
        }
    }

    pub fn right() -> Self {
        Ket {
            a: Complex::new(0.70710678118, 0.0),
            b: Complex::new(0.70710678118, 0.0),
        }
    }

    pub fn left() -> Self {
        Ket {
            a: Complex::new(0.70710678118, 0.0),
            b: Complex::new(-0.70710678118, 0.0),
        }
    }
}

impl Mul<Ket> for Bra {
    type Output = Complex;

    fn mul(self, other: Ket) -> Complex {
        self.a * other.a + self.b * other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonal() {
        assert!((Bra::up() * Ket::down()).norm() == 0.0);
        assert!((Bra::left() * Ket::right()).norm() == 0.0);
    }
}