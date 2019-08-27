use std::ops::{Mul, Add};
use nalgebra::{DefaultAllocator, MatrixMN, DimName, U2, U4, Matrix2, Matrix4, Matrix};
use nalgebra::allocator::Allocator;

use {Ket, Complex, SQRT_2_INVERSE};

/// Outer product in bra-ket notation, as well used as a linear operatior.
/// You can multiply it by ket to put it in the observable state defined by the operator.
#[derive(Clone, Debug, PartialEq)]
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
    /// Deconstruct the outer product returning the inner matrix.
    pub fn into_matrix(self) -> MatrixMN<Complex, D, D> {
        self.0
    }

    /// H2 (2 dim hadamard) operator
    pub fn h2() -> Outer<U2> {
        Outer::<U2>(Matrix2::<Complex>::new(1.0.into(), 1.0.into(), 1.0.into(), (-1.0).into())) * SQRT_2_INVERSE
    }

    /// Z2 (2 dim Z-gate) operator
    pub fn z2() -> Outer<U2> {
        Outer::<U2>(Matrix2::<Complex>::new(1.0.into(), 0.0.into(), 0.0.into(), (-1.0).into()))
    }

    /// N2 (2 dim N-gate) operator
    pub fn n2() -> Outer<U2> {
        Outer::<U2>(Matrix2::<Complex>::new(0.0.into(), 1.0.into(), 1.0.into(), 0.0.into()))
    }

    /// CNOT (2*2 dim) operator
    pub fn cnot() -> Outer<U4> {
        Outer::<U4>(
            Matrix4::<Complex>::new(
                1.0.into(), 0.0.into(), 0.0.into(), 0.0.into(),
                0.0.into(), 1.0.into(), 0.0.into(), 0.0.into(),
                0.0.into(), 0.0.into(), 0.0.into(), 1.0.into(),
                0.0.into(), 0.0.into(), 1.0.into(), 0.0.into(),
            )
        )
    }

    /// Quantum Fourier Transform (QFT) matrix operator
    pub fn qft() -> Outer<D> {
        let mut matrix: MatrixMN<Complex, D, D> = Matrix::zeros_generic(D::name(), D::name());
        let dim = D::name().value();

        let n = dim as f64;

        let coef = (Complex::from(1.0) / (n as f64)).sqrt();

        for i in 0..dim {
            for j in 0..dim {
                let power = (Complex::from(2.0) * ::std::f64::consts::PI * Complex::i() / n)
                    * (i as f64) * (j as f64);

                *matrix.get_mut((i, j)).expect("(i, j) in (dim, dim) range") = power.exp() * coef;
            }
        }

        Outer(matrix)
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