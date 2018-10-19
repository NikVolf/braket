extern crate brakets;

fn main() {

    let bra_up = brakets::Bra2::up();
    let ket_up = brakets::Ket2::up();

    println!("<u|u> = {}", bra_up.clone()*ket_up.clone());
    println!(">u|<u = {}", ket_up.clone()*bra_up.clone());

    let bra_inw = brakets::Bra2::inw();
    let ket_inw = brakets::Ket2::inw();

    println!("<i|i> = {}", bra_inw.clone()*ket_inw.clone());
    println!(">i|<i = {}", ket_inw.clone()*bra_inw.clone());

    let bra_out = brakets::Bra2::out();
    let ket_out = brakets::Ket2::out();
    println!(">o|<o = {}", ket_out.clone()*bra_out.clone());

    let sum_of_outers = ket_out.clone()*bra_out.clone() + ket_inw.clone()*bra_inw.clone();

    println!(">o|<o + >i|<i = {}", sum_of_outers.clone())
}