extern crate brakets;
extern crate nalgebra;

use brakets::{Ket2, Complex, Outer};

fn main() {

    // Various operators on standard states

    let a = nalgebra::Matrix2::<Complex>::new(
        0.0.into(),
        1.0.into(),
        1.0.into(),
        0.0.into(),
    );


    println!(
        "{} on {} = {}",
        Outer::from(a),
        Ket2::up(),
        Outer::from(a) * Ket2::up()
    );

    println!(
        "{} on {} = {}",
        Outer::from(a),
        Ket2::down(),
        Outer::from(a) * Ket2::down()
    );
}