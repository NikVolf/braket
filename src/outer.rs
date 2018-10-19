use std::ops::{Mul, Add};
use nalgebra::{DefaultAllocator, MatrixMN, DimName};
use nalgebra::allocator::Allocator;

use {Ket, Complex};

#[derive(Clone)]
pub struct Outer<D: DimName>(pub(crate) MatrixMN<Complex, D, D>)
    where DefaultAllocator: Allocator<Complex, D, D>;

impl<D: DimName> Mul<Ket<D>> for Outer<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D>
{
    type Output = Ket<D>;

    fn mul(self, other: Ket<D>) -> Self::Output {
        Ket(self.0 * other.0)
    }
}

impl<D: DimName> ::std::fmt::Display for Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>,
    DefaultAllocator: Allocator<usize, D, D>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
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

impl<D: DimName> Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>
{
    pub fn into_matrix(self) -> MatrixMN<Complex, D, D> {
        self.0
    }
}

impl<D: DimName> From<MatrixMN<Complex, D, D>> for Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>
{
    fn from(v: MatrixMN<Complex, D, D>) -> Self {
        Outer(v)
    }
}

impl<D: DimName> Mul<f64> for Outer<D>
    where DefaultAllocator: Allocator<Complex, D, D>
{
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Outer(self.0 * Complex::new(other, 0.0))
    }
}