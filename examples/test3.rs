extern crate brakets;

use brakets::{Bra2, Ket2};

fn main() {

    /// This is how up-down operator is calculated

    println!(
        ">u|<u - >d|<d = {}",
        Ket2::up() * Bra2::up() + (Ket2::down() * Bra2::down() * -1.0)
    );
}