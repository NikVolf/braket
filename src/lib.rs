extern crate num_complex;

use num_complex::Complex64 as Complex;

pub struct Bra {
    a: Complex,
    b: Complex,
}

pub struct Ket {
    a: Complex,
    b: Complex,
}

#[cfg(test)]
mod tests {

}