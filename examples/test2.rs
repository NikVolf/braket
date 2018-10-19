extern crate brakets;

use brakets::{Bra2, Ket2};

fn main() {

    // this is probability of observing spin up again after it passed through left-right analyzer
    // and was originally (up)!
    // should be 1/4

    println!(
        "<u|l>|<l|u> = {}",
        (Bra2::up() * (Ket2::left() * Bra2::left()) * Ket2::up()).norm().powi(2)
    );

    // this wave function of spin originally been up after it passed through left-right analyzer
    // but was not observed (was in super position of left and right)
    // should be same as Ket2::up() = [1, 0]

    println!(
        "(>l|<l + >r|<r) u> = {}",
        (Ket2::left() * Bra2::left() + Ket2::right() * Bra2::right()) * Ket2::up()
    )
}