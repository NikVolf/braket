use {Bra2, Ket2};

const E: f64 = 0.00000000001;

#[test]
fn orthogonal() {
    assert!((Bra2::up() * Ket2::down()).norm() == 0.0);
    assert!((Bra2::left() * Ket2::right()).norm() == 0.0);
    assert!((Bra2::inw() * Ket2::out()).norm() == 0.0);
}

#[test]
fn prob() {
    assert!((Bra2::up() * Ket2::left()).norm().powi(2) - 0.5 < E);
    assert!((Bra2::down() * Ket2::right()).norm().powi(2) - 0.5 < E);
    assert!((Bra2::up() * Ket2::inw()).norm().powi(2) - 0.5 < E);
}

#[test]
fn outer() {
    let outer = (Ket2::up() * Bra2::down()).into_matrix();
    assert!(unsafe { outer.get_unchecked((0, 0)).im } < E);
    assert!(unsafe { outer.get_unchecked((0, 1)).re } - 1.0 < E);
}