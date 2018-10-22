extern crate brakets;
extern crate nalgebra;
extern crate num_complex as complex;

use brakets::{Bra2, Ket2};
use nalgebra::Vector2;
use complex::{Complex64 as Complex};

fn main() {

    // new spin function

    let f: Ket2 = Vector2::new(Complex::new(0.6, 0.2), Complex::new(0.4, 0.2)).into();

    // extract up probability

    let v = (Bra2::up() * f).norm().powi(2);

    println!("v = {}", v);
}