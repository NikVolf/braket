use std::ops::{Mul, Add};
use nalgebra::{DefaultAllocator, U1, VectorN, Vector2, DimName, U2};
use nalgebra::allocator::Allocator;

use {SQRT_2_INVERSE, Bra, Outer, Complex};

#[derive(Clone)]
pub struct Ket<D: DimName>(pub(crate) VectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, D>;

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
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(SQRT_2_INVERSE, 0.0),
            )
        )
    }

    pub fn left() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(-SQRT_2_INVERSE, 0.0),
            )
        )
    }

    pub fn inw() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(0.0, SQRT_2_INVERSE),
            )
        )
    }

    pub fn out() -> Ket<U2> {
        Ket::<U2>(
            Vector2::new(
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(0.0, -SQRT_2_INVERSE),
            )
        )
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

impl<D: DimName> From<Bra<D>> for Ket<D>
    where DefaultAllocator: Allocator<Complex, D> + Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    fn from(v: Bra<D>) -> Self {
        Ket(v.0.transpose())
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