const EXPONENT_BYTES: u16 = 11;
const EXPONENT_SHIFT: u16 = 1023;
const MANTISSA_BYTES: u16 = 52;

const PRECISION: u16 = 40;

pub(in crate::math) fn float_exponent(f: f64) -> u16 {
    u16::try_from((f.to_bits() << 1) >> (MANTISSA_BYTES + 1)).unwrap()
}

pub fn round(f: f64) -> f64 {
    if f == 0.0 || f.is_nan() || f.is_infinite() { return f; };

    let exp = float_exponent(f);
    let extra_signs = (MANTISSA_BYTES + EXPONENT_SHIFT).saturating_sub(exp + PRECISION);
    let mask = (u64::MAX >> extra_signs) << extra_signs;
    f64::from_bits(f.to_bits() & mask)
}

pub(in crate::math) fn round_prec(f: f64, prec: u16) -> f64 {
    if f == 0.0 || f.is_nan() || f.is_infinite() { return f; };

    let exp = float_exponent(f);
    let extra_signs = (MANTISSA_BYTES + EXPONENT_SHIFT).saturating_sub(exp + prec);
    let mask = (u64::MAX >> extra_signs) << extra_signs;
    f64::from_bits(f.to_bits() & mask)
}
