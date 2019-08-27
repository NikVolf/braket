use std::ops::{Mul, Add};
use nalgebra::{DefaultAllocator, U1, VectorN, Vector2, DimName, U2, DimNameProd, DimNameMul, Vector};
use nalgebra::allocator::Allocator;

use {SQRT_2_INVERSE, Bra, Outer, Complex};

/// Generic ket. You can multiply it by bra to an outer product (or operator).
#[derive(Clone, Debug, PartialEq)]
pub struct Ket<D: DimName>(pub(crate) VectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, D>;

impl<D: DimName> Ket<D>
    where DefaultAllocator: Allocator<Complex, D>
{
    /// Up 2-dimension ket, [1, 0]
    pub fn up() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 0.0),
            )
        )
    }

    /// Down 2-dimension ket, [0, 1]
    pub fn down() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0),
            )
        )
    }

    /// Right 2-dimension ket, 1/√2[1, 1]
    pub fn right() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(SQRT_2_INVERSE, 0.0),
            )
        )
    }

    /// Left 2-dimension ket, 1/√2[1, -1]
    pub fn left() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(-SQRT_2_INVERSE, 0.0),
            )
        )
    }

    /// Inward 2-dimension ket, 1/√2[1, i]
    pub fn inw() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(0.0, SQRT_2_INVERSE),
            )
        )
    }

    /// Outward 2-dimension ket, 1/√2[1, -i]
    pub fn out() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(0.0, -SQRT_2_INVERSE),
            )
        )
    }

    /// Kronecker (tensor) product of two kets
    ///
    /// For qubits, it's possible to produce only 2 qubits state with this product.
    pub fn cross(self, other: Ket<D>) -> Ket<DimNameProd<D, D>>
        where
            D: DimNameMul<D>,
            DefaultAllocator: Allocator<Complex, DimNameProd<D, D>>
    {
        let mut result = Vector::zeros_generic(DimNameProd::<D, D>::name(), U1);
        let dim = D::name().value();
        for i in 0..dim {
            for j in 0..dim {
                *result.get_mut(
                    i*dim + j
                ).expect("i, j in (dim, dim) range") = self.0[i] * other.0[j];
            }
        }
        Ket(result)
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

impl<D: DimName> Add for Ket<D>
    where DefaultAllocator: Allocator<Complex, D>
{
    type Output = Self;

    fn add(self, other: Ket<D>) -> Self::Output {
        Ket(self.0 + other.0)
    }
}

impl<D: DimName> Mul<Complex> for Ket<D>
    where DefaultAllocator: Allocator<Complex, D, U1>
{
    type Output = Self;

    fn mul(self, other: Complex) -> Self::Output {
        Ket(self.0 * other)
    }
}


impl<D: DimName> From<Bra<D>> for Ket<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    fn from(v: Bra<D>) -> Self {
        Ket(v.0.transpose())
    }
}

impl<D: DimName> From<VectorN<Complex, D>> for Ket<D>
    where DefaultAllocator: Allocator<Complex, D>
{
    fn from(v: VectorN<Complex, D>) -> Self {
        Ket(v)
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
