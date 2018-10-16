extern crate num_complex;
extern crate nalgebra;
extern crate typenum;

use std::ops::Mul;
use num_complex::Complex64 as Complex;
use nalgebra::{DefaultAllocator, MatrixMN, U1, VectorN, RowVector2, RowVectorN, Vector2, DimName, U2};
use nalgebra::allocator::Allocator;

#[derive(Clone)]
pub struct Bra<D: DimName>(RowVectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, U1, D>;

#[derive(Clone)]
pub struct Ket<D: DimName>(VectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, D>;

impl<D: DimName> Bra<D>
    where DefaultAllocator: Allocator<Complex, U1, D>
{
    pub fn up() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
            )
        )
    }

    pub fn down() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            )
        )
    }

    pub fn right() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(0.70710678118, 0.0),
            )
        )
    }

    pub fn left() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(-0.70710678118, 0.0),
            )
        )
    }
}

impl<D: DimName> Ket<D>
    where DefaultAllocator: Allocator<Complex, D>
{
    pub fn up() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
            )
        )
    }

    pub fn down() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            )
        )
    }

    pub fn right() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(0.70710678118, 0.0),
            )
        )
    }

    pub fn left() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(-0.70710678118, 0.0),
            )
        )
    }
}

impl<D: DimName> Mul<Ket<D>> for Bra<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, U1, D>
{
    type Output = Complex;

    fn mul(self, other: Ket<D>) -> Self::Output {
        let mut m = self.0;
        for f in m.iter_mut() { *f = f.conj(); }
        unsafe { *(m * other.0).get_unchecked(0, 0) }
    }
}

impl<D: DimName> Mul<Bra<D>> for Ket<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    type Output = MatrixMN<Complex, D, D>;

    fn mul(self, other: Bra<D>) -> Self::Output {
        let mut m = other.0;
        for f in m.iter_mut() { *f = f.conj(); }
        self.0 * m
    }
}

pub type Bra2 = Bra<U2>;
pub type Ket2 = Ket<U2>;

#[cfg(test)]
mod tests {
    use super::*;

    const E: f64 = 0.00000000001;

    #[test]
    fn orthogonal() {
        assert!((Bra2::up() * Ket2::down()).norm() == 0.0);
        assert!((Bra2::left() * Ket2::right()).norm() == 0.0);
    }

    #[test]
    fn prob() {
        assert!((Bra2::up() * Ket2::left()).norm().powi(2) - 0.5 < E);
        assert!((Bra2::down() * Ket2::right()).norm().powi(2) - 0.5 < E);
    }

    #[test]
    fn outer() {
        let outer = Ket2::up() * Bra2::down();
        assert!(unsafe { outer.get_unchecked(0, 0).im } < E);
        assert!(unsafe { outer.get_unchecked(0, 1).re } - 1.0 < E);
    }
}