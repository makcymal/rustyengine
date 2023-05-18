use {
    super::super::{
        precision::round_mode::{
            float_exponent,
            round_prec,
        },
    },
};


#[test]
fn float_exponent_of_1() {
    assert_eq!(float_exponent(1.0), 1023);
}

#[test]
fn float_exponent_of_2() {
    assert_eq!(float_exponent(2.0), 1024);
}

#[test]
fn float_exponent_of_10() {
    assert_eq!(float_exponent(10.0), 1026);
}

#[test]
fn float_exponent_of_0_5() {
    assert_eq!(float_exponent(0.5), 1022);
}

#[test]
fn float_exponent_of_0_3() {
    assert_eq!(float_exponent(0.3), 1021);
}

#[test]
fn round_0_5_with_prec_0() {
    assert_eq!(round_prec(0.5, 0), 0.5);
}

#[test]
fn round_0_75_with_prec_1() {
    assert_eq!(round_prec(0.75, 1), 0.5);
}

#[test]
fn round_0_875_with_prec_1() {
    assert_eq!(round_prec(0.875, 1), 0.5);
}

#[test]
fn round_0_875_with_prec_2() {
    assert_eq!(round_prec(-0.875, 2), -0.75);
}

#[test]
fn round_1_5625_with_prec_3() {
    assert_eq!(round_prec(1.5625, 3), 1.5);
}

#[test]
fn round_0_5625_with_prec_3() {
    assert_eq!(round_prec(0.5625, 3), 0.5);
}
