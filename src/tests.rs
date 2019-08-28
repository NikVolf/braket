use assert_approx_eq::assert_approx_eq;

use {Ket, Bra2, Bra4, Ket2, Ket4, Outer2, Outer4, SQRT_2_INVERSE, Complex, U8};

#[test]
fn orthogonal() {
    assert!((Bra2::up() * Ket2::down()).norm() == 0.0);
    assert!((Bra2::left() * Ket2::right()).norm() == 0.0);
    assert!((Bra2::inw() * Ket2::out()).norm() == 0.0);
}

#[test]
fn prob() {
    assert_approx_eq!((Bra2::up() * Ket2::left()).norm().powi(2), 0.5);
    assert_approx_eq!((Bra2::down() * Ket2::right()).norm().powi(2), 0.5);
    assert_approx_eq!((Bra2::up() * Ket2::inw()).norm().powi(2), 0.5);
}

#[test]
fn outer() {
    let outer = (Ket2::up() * Bra2::down()).into_matrix();
    assert_approx_eq!(outer[(0, 0)].im, 0.0);
    assert_approx_eq!(outer[(0, 1)].re, 1.0);
}

#[test]
fn hadamard() {
    let hadamard_product_up = Bra2::up() * Outer2::h2();
    let hadamard_product_down = Bra2::up() * Outer2::h2();

    // hadamard gate produces 50/50 probability
    assert_approx_eq!(
        (hadamard_product_up * Ket2::up()).norm().powi(2), 0.5
    );
    assert_approx_eq!(
        (hadamard_product_down * Ket2::up()).norm().powi(2), 0.5
    );

    // double hadamard should equal to identity transformation
    assert_approx_eq!(
        (((Bra2::up() * Outer2::h2()) * Outer2::h2()) * Ket2::up())
            .norm().powi(2),

        1.0
    );
    assert_approx_eq!(
        (((Bra2::down() * Outer2::h2()) * Outer2::h2()) * Ket2::down())
            .norm().powi(2),

        1.0
    );
    assert_approx_eq!(
        (((Bra2::up() * Outer2::h2()) * Outer2::h2()) * Ket2::down())
            .norm().powi(2),

        0.0
    );
    assert_approx_eq!(
        (((Bra2::down() * Outer2::h2()) * Outer2::h2()) * Ket2::up())
            .norm().powi(2),

        0.0
    );
}

#[test]
fn zgate_phase() {
    let hadamard_product_up = Bra2::up() * Outer2::h2();
    let hadamard_product_down = Bra2::down() * Outer2::h2();
    let z_up = hadamard_product_up.clone() * Outer2::z2();
    let z_down = hadamard_product_down.clone() * Outer2::z2();

    assert_eq!(
        z_up, hadamard_product_down
    );

    assert_eq!(
        z_down, hadamard_product_up
    );
}

#[test]
fn bell_state() {
    let state = (Ket2::up().cross(Ket2::up()) + Ket2::down().cross(Ket2::down())) * Complex::from(SQRT_2_INVERSE);

    let q11 = Ket2::up().cross(Ket2::up());

    // bell state (|00> + |11>)/2^-1/2 is 50% in computational basis
    let prob = (Bra4::from(q11) * state).norm().powi(2);

    assert_approx_eq!(prob.abs(), 0.5);
}

#[test]
fn bell_cnot() {
    let bell_state = (Ket2::up().cross(Ket2::up()) + Ket2::down().cross(Ket2::down())) * Complex::from(SQRT_2_INVERSE);

    let q1 = Outer2::h2() * Ket2::up();
    let q2  = Ket2::up();

    let bell_state2 = Outer4::cnot() * q1.cross(q2);

    assert_eq!(bell_state, bell_state2);
}

#[test]
fn qft() {
    let qft4 = Outer4::qft().into_matrix();

    // QFT(4) matrix:
    //  1   1   1   1
    //  1   i  -1  -i
    //  1  -1   1  -1
    //  1  -i  -1   i
    // (multiplied by 1/sqrt(4) = 1/2)

    assert_approx_eq!(qft4[(0, 0)].im, 0.0);
    assert_approx_eq!(qft4[(0, 0)].re, 0.5);

    assert_approx_eq!(qft4[(1, 1)].im, 0.5);
    assert_approx_eq!(qft4[(1, 1)].re, 0.0);

    assert_approx_eq!(qft4[(2, 2)].im, 0.0);
    assert_approx_eq!(qft4[(2, 2)].re, 0.5);

    assert_approx_eq!(qft4[(2, 3)].im, 0.0);
    assert_approx_eq!(qft4[(2, 3)].re, -0.5);
}

#[test]
fn bits() {
    let q11 = Ket2::up().cross(Ket2::up());
    let q11_bits: Ket4 = Ket::from_bits(0b11).expect("11 are a valid ket4 bits");

    assert_eq!(q11, q11_bits);

    let q00 = Ket2::down().cross(Ket2::down());
    let q00_bits: Ket4 = Ket::from_bits(0b00).expect("11 are a valid ket4 bits");

    assert_eq!(q00, q00_bits);

    let q111_bits: Ket<U8> = Ket::from_bits(0b111).expect("111 are valid ket8 bits");

    assert_approx_eq!(q111_bits.0.get(0).expect("at least 8").re, 1.0);

    for i in 1..8 {
        assert_approx_eq!(q111_bits.0.get(i).expect("at least 8").re, 0.0);
    }
}