use std::ops::{Mul, Add};
use nalgebra::{DefaultAllocator, U1, RowVector2, RowVectorN, DimName, U2};
use nalgebra::allocator::Allocator;

use {SQRT_2_INVERSE, Ket, Outer, Complex};

#[derive(Clone)]
pub struct Bra<D: DimName>(pub(crate) RowVectorN<Complex, D>)
    where DefaultAllocator: Allocator<Complex, U1, D>;

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
                Complex::new(SQRT_2_INVERSE, 0.0),
                Complex::new(0.0, -SQRT_2_INVERSE),
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

// Only bra can be multiplied my square matrix
impl<D: DimName> Mul<Outer<D>> for Bra<D>
    where DefaultAllocator: Allocator<Complex, D, D> + Allocator<Complex, U1, D>
{
    type Output = Self;

    fn mul(self, other: Outer<D>) -> Self::Output {
        Bra(self.0 * other.0)
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

impl<D: DimName> ::std::fmt::Display for Bra<D>
    where DefaultAllocator: Allocator<Complex, U1, D>,
    DefaultAllocator: Allocator<usize, U1, D>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
