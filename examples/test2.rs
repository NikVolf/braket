extern crate brakets;

use brakets::{Bra2, Ket2};

fn main() {

    // this is probability of observing spin up again after it passed through left-right analizer
    // and was originally (up)!
    // should be 1/4

    println!(
        "<u|l>|<l|u> = {}",
        (Bra2::up() * (Ket2::left() * Bra2::left()) * Ket2::up()).norm().powi(2)
    );
}