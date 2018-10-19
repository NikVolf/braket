extern crate brakets;
extern crate nalgebra;

use brakets::{Bra2, Ket2, Complex, Outer};

fn main() {

    /// Various operators on standard states

    let A = nalgebra::Matrix2::<Complex>::new(
        0.0.into(),
        1.0.into(),
        1.0.into(),
        0.0.into(),
    );


    println!(
        "{} on {} = {}",
        Outer::from(A),
        Ket2::up(),
        Outer::from(A) * Ket2::up()
    );

    println!(
        "{} on {} = {}",
        Outer::from(A),
        Ket2::down(),
        Outer::from(A) * Ket2::down()
    );
}