//! Two modes on working with precision are available:
//! 1. `Exact` when there are no roundations and `f64` are compared as absolute difference against some epsilon
//! 2. `Round` when all operations are performed with following roundation, so `f64` are compared as they are


/// Precision modes
enum PrecMode { Exact, Round }

/// Current precision mode
static mut PRECMODE: PrecMode = PrecMode::Exact;

/// Select `Exact` precision mode
pub fn set_exact_mode() {
    unsafe {
        PRECMODE = PrecMode::Exact
    }
}

/// Select `Round` precision mode
pub fn set_round_mode() {
    unsafe {
        PRECMODE = PrecMode::Round
    }
}

/// Set precision with coefficient `p` within [0, 255] where greater the `p` means higher precision
pub fn set_precision(p: u8) {
    unsafe {
        round_mode::PRECISION = (round_mode::MANTISSA_BYTES as f32 * (p as f32 / 255.0)) as u16;
        exact_mode::EPSILON = f64::EPSILON * (256.0 - p as f64);
    }
}

/// Roundation based on the current precision mode
pub fn round(f: f64) -> f64 {
    unsafe {
        match PRECMODE {
            PrecMode::Exact => f,
            PrecMode::Round => round_mode::round(f)
        }
    }
}

/// Equality comparasion based on the current precision mode
pub fn eq(lhs: f64, rhs: f64) -> bool {
    unsafe {
        match PRECMODE {
            PrecMode::Exact => exact_mode::eq(lhs, rhs),
            PrecMode::Round => lhs == rhs
        }
    }
}


/// Roundation in `Round` mode
pub(in super) mod round_mode {
    const EXPONENT_BYTES: u16 = 11;
    const EXPONENT_SHIFT: u16 = 1023;
    pub(in super) const MANTISSA_BYTES: u16 = 52;

    /// Number of digits after point in binary notation
    pub(in super) static mut PRECISION: u16 = 40;

    /// 1024 - mantissa shifting
    pub fn float_exponent(f: f64) -> u16 {
        u16::try_from((f.to_bits() << 1) >> (MANTISSA_BYTES + 1)).unwrap()
    }

    /// Sets all the digits after `PRECISION`'th to zero in binary notation
    pub fn round(f: f64) -> f64 {
        if f == 0.0 || f.is_nan() || f.is_infinite() { return f; };

        let exp = float_exponent(f);
        let extra_signs = unsafe {
            (MANTISSA_BYTES + EXPONENT_SHIFT).saturating_sub(exp + PRECISION)
        };
        let mask = (u64::MAX >> extra_signs) << extra_signs;
        f64::from_bits(f.to_bits() & mask)
    }

    /// Sets all the digits after prec'th to zero in binary notation
    pub fn round_prec(f: f64, prec: u16) -> f64 {
        if f == 0.0 || f.is_nan() || f.is_infinite() { return f; };

        let exp = float_exponent(f);
        let extra_signs = (MANTISSA_BYTES + EXPONENT_SHIFT).saturating_sub(exp + prec);
        let mask = (u64::MAX >> extra_signs) << extra_signs;
        f64::from_bits(f.to_bits() & mask)
    }
}

/// Equation in `Exact` mode
pub(in super) mod exact_mode {
    pub(in super) static mut EPSILON: f64 = f64::EPSILON;

    pub fn eq(lhs: f64, rhs: f64) -> bool {
        unsafe {
            (lhs - rhs).abs() < EPSILON
        }
    }
}
