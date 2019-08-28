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

    /// Cross product of basis (|up> & |down>) qubits picked from a bit string.
    pub fn from_bits(number: u16) -> Result<Ket<D>, &'static str>
    {
        fn get_bit_at(input: usize, n: usize) -> bool {
            if n < 32 {
                input & (1 << n) != 0
            } else {
                false
            }
        }

        let size = D::name().value();

        let value_bits: usize = 16 - number.leading_zeros() as usize;

        if size < (1 << value_bits) {
            return Err("Cannot fit number to the qubit state: not enough space");
        }

        if size & (size - 1) != 0 {
            return Err("Should be a power of 2 to represent a tensor product");
        }

        let mut bit_space = Vec::new();
        for i in 0..value_bits {
            if get_bit_at(number as usize,i ) {
                bit_space.push(Ket::<U2>::up());
            } else {
                bit_space.push(Ket::<U2>::down());
            }
        }

        let mut result = Vector::zeros_generic(D::name(), U1);

        let def0 = &Ket::down();

        let size_log2 = 15 - (size as u16).leading_zeros() as usize;

        for i in 0..size {
            let mut ith = Complex::from(1.0);
            for k in 0..size_log2 {
                let kth = if k < bit_space.len() { &bit_space[k] } else { def0 };
                ith = ith *
                    {
                        if get_bit_at(i, k) { kth.0.get(1).expect("always 2 elements") }
                        else { kth.0.get(0).expect("always 2 elements") }
                    }
            }
            *result.get_mut(i).expect("less than value_bits elements") = ith;
        }

        Ok(Ket(result))
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
