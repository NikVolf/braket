extern crate num_complex;
extern crate nalgebra;
extern crate typenum;

use std::ops::{Mul, Add};
use num_complex::Complex64 as Complex;
use nalgebra::{DefaultAllocator, MatrixMN, U1, VectorN, RowVector2, RowVectorN, Vector2, DimName, U2};
use nalgebra::allocator::Allocator;

#[derive(Clone)]
pub struct Bra<D: DimName>(RowVectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, U1, D>;

#[derive(Clone)]
pub struct Ket<D: DimName>(VectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, D>;

#[derive(Clone)]
pub struct Outer<D: DimName>(MatrixMN<Complex, D, D>)
    where DefaultAllocator: Allocator<Complex, D, D>;

pub const SQRT_2_INVERSE: f64 = 0.70710678118;

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
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(SQRT_2_INVERSE, 0.0),
            )
        )
    }

    pub fn left() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(-SQRT_2_INVERSE, 0.0),
            )
        )
    }

    pub fn inw() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(0.0, SQRT_2_INVERSE),
            )
        )
    }

    pub fn out() -> Bra<U2> {
        Bra::<U2>(
            RowVector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(0.0, -0.70710678118),
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

    pub fn inw() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(0.0, 0.70710678118),
            )
        )
    }

    pub fn out() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(0.70710678118, 0.0),
                Complex::new(0.0, -0.70710678118),
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
    type Output = Outer<D>;

    fn mul(self, other: Bra<D>) -> Self::Output {
        let mut m = other.0;
        for f in m.iter_mut() { *f = f.conj(); }
        Outer(self.0 * m)
    }
}

/// Only bra can be multiplied my square matrix
impl<D: DimName> Mul<Outer<D>> for Bra<D>
    where DefaultAllocator: Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    type Output = Self;

    fn mul(self, other: Outer<D>) -> Self::Output {
        Bra(self.0 * other.0)
    }
}

impl<D: DimName> Add for Ket<D>
    where DefaultAllocator: Allocator<Complex, D>
{
    type Output = Self;

    fn add(self, other: Ket<D>) -> Self::Output {
        Ket(self.0 + other.0)
    }
}

impl<D: DimName> Add for Bra<D>
    where DefaultAllocator: Allocator<Complex, U1, D>
{
    type Output = Self;

    fn add(self, other: Bra<D>) -> Self::Output {
        Bra(self.0 + other.0)
    }
}

impl<D: DimName> From<Ket<D>> for Bra<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    fn from(v: Ket<D>) -> Self {
        Bra(v.0.transpose())
    }
}

impl<D: DimName> From<Bra<D>> for Ket<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    fn from(v: Bra<D>) -> Self {
        Ket(v.0.transpose())
    }
}

impl<D: DimName> Mul<Ket<D>> for Outer<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D>
{
    type Output = Ket<D>;

    fn mul(self, other: Ket<D>) -> Self::Output {
        Ket(self.0 * other.0)
    }
}

impl<D: DimName> Add<Outer<D>> for Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>
{
    type Output = Self;

    fn add(self, other: Outer<D>) -> Self::Output {
        Outer(self.0 + other.0)
    }
}

pub type Bra2 = Bra<U2>;
pub type Ket2 = Ket<U2>;

impl<D: DimName> ::std::fmt::Display for Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>,
    DefaultAllocator: Allocator<usize, D, D>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<D: DimName> ::std::fmt::Display for Bra<D>
    where DefaultAllocator: Allocator<Complex, U1, D>,
    DefaultAllocator: Allocator<usize, U1, D>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<D: DimName> ::std::fmt::Display for Ket<D>
    where DefaultAllocator: Allocator<Complex, D>,
    DefaultAllocator: Allocator<usize, D>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<D: DimName> Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>
{
    pub fn into_matrix(self) -> MatrixMN<Complex, D, D> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const E: f64 = 0.00000000001;

    #[test]
    fn orthogonal() {
        assert!((Bra2::up() * Ket2::down()).norm() == 0.0);
        assert!((Bra2::left() * Ket2::right()).norm() == 0.0);
        assert!((Bra2::inw() * Ket2::out()).norm() == 0.0);
    }

    #[test]
    fn prob() {
        assert!((Bra2::up() * Ket2::left()).norm().powi(2) - 0.5 < E);
        assert!((Bra2::down() * Ket2::right()).norm().powi(2) - 0.5 < E);
        assert!((Bra2::up() * Ket2::inw()).norm().powi(2) - 0.5 < E);
    }

    #[test]
    fn outer() {
        let outer = (Ket2::up() * Bra2::down()).into_matrix();
        assert!(unsafe { outer.get_unchecked(0, 0).im } < E);
        assert!(unsafe { outer.get_unchecked(0, 1).re } - 1.0 < E);
    }
}