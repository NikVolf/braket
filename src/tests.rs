use {Bra2, Ket2, Outer2, SQRT_2_INVERSE};

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

#[test]
fn hadamard() {
    let hadamard_product_up = Bra2::up() * Outer2::h2();
    let hadamard_product_down = Bra2::up() * Outer2::h2();

    // hadamard gate produces 50/50 probability
    assert!(
        (hadamard_product_up * Ket2::up()).norm().powi(2) - 0.5 < E
    );
    assert!(
        (hadamard_product_down * Ket2::up()).norm().powi(2) - 0.5 < E
    );

    // double hadamard should equal to identity transformation
    assert!(
        (((Bra2::up() * Outer2::h2()) * Outer2::h2()) * Ket2::up())
            .norm().powi(2) - 1.0 < E
    );
    assert!(
        (((Bra2::down() * Outer2::h2()) * Outer2::h2()) * Ket2::down())
            .norm().powi(2) - 1.0 < E
    );
    assert!(
        (((Bra2::up() * Outer2::h2()) * Outer2::h2()) * Ket2::down())
            .norm().powi(2) < E
    );
    assert!(
        (((Bra2::down() * Outer2::h2()) * Outer2::h2()) * Ket2::up())
            .norm().powi(2) < E
    );
}

#[test]
fn zgate_phase() {
    let hadamard_product_up = Bra2::up() * Outer2::h2();
    let z_up = hadamard_product_up * Outer2::z2();

    assert!(
        unsafe { z_up.0.get_unchecked(0).re - SQRT_2_INVERSE } < E
    );

    assert!(
        unsafe { z_up.0.get_unchecked(1).re + SQRT_2_INVERSE } < E
    );
}