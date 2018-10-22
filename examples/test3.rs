extern crate brakets;

use brakets::{Bra2, Ket2};

fn main() {

    /// This is how up-down operator is calculated

    println!(
        "+s (>u|<u) - s(>d|<d) = {}, where s = 1.0",
        Ket2::up() * Bra2::up() + (Ket2::down() * Bra2::down() * -1.0)
    );
}