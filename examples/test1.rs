extern crate brakets;

fn main() {

    let bra = brakets::Bra2::up();
    let ket = brakets::Ket2::up();

    println!("<u|u> = {}", bra*ket);
}