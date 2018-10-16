extern crate num_complex;
extern crate nalgebra;

use std::ops::Mul;
use num_complex::Complex64 as Complex;
use nalgebra::Matrix2;

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

    fn mul(self, other: Ket) -> Self::Output {
        self.a * other.a + self.b * other.b
    }
}

impl Mul<Bra> for Ket {
    type Output = Matrix2<Complex>;

    fn mul(self, other: Bra) -> Self::Output {
        let mut m = Matrix2::zeros();
        unsafe {
            *m.get_unchecked_mut(0, 0) = self.a * other.a;
            *m.get_unchecked_mut(0, 1) = self.b * other.a;
            *m.get_unchecked_mut(1, 0) = self.a * other.b;
            *m.get_unchecked_mut(1, 1) = self.b * other.b;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const E: f64 = 0.00000000001;

    #[test]
    fn orthogonal() {
        assert!((Bra::up() * Ket::down()).norm() == 0.0);
        assert!((Bra::left() * Ket::right()).norm() == 0.0);
    }

    #[test]
    fn prob() {
        assert!((Bra::up() * Ket::left()).norm().powi(2) - 0.5 < E);
        assert!((Bra::down() * Ket::right()).norm().powi(2) - 0.5 < E);
    }

    #[test]
    fn outer() {
        let outer = Ket::up() * Bra::down();
        assert!(unsafe { outer.get_unchecked(0, 0).im } < E);
        assert!(unsafe { outer.get_unchecked(0, 1).re } - 1.0 < E);
    }
}