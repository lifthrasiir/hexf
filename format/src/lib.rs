//! Format hexadecimal floats.
//!
//! For conversion, `to_hex_string` is provided.
//! For formatting, `Format` struct is provided.

mod internal {
    use num_traits::{float::Float, Signed, Zero};
    use std::fmt;

    pub trait FormatHexf: Signed + Float + Zero {
        #[inline]
        fn sign_string(&self) -> &'static str {
            if self.is_negative() {
                "-"
            } else {
                ""
            }
        }
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                v if v.is_zero() => write!(f, "{}0x0.0p+0", v.sign_string()),
                v if v.is_infinite() => write!(f, "{}inf", v.sign_string()),
                v if v.is_nan() => write!(f, "NaN"),
                _ => self.fmt_finite(f),
            }
        }
        fn fmt_finite(&self, f: &mut fmt::Formatter) -> fmt::Result;
    }

    impl FormatHexf for f32 {
        #[inline]
        fn fmt_finite(&self, f: &mut fmt::Formatter) -> fmt::Result {
            const BITS: i16 = 23;
            const FRACT_MASK: u64 = 0x7f_ffff;
            let (mantissa, exponent, _) = self.integer_decode();
            write!(
                f,
                "{}0x{:x}.{:06x}p{:+}",
                self.sign_string(),
                mantissa >> BITS,
                (mantissa & FRACT_MASK) << 1,
                exponent + BITS
            )
        }
    }

    impl FormatHexf for f64 {
        #[inline]
        fn fmt_finite(&self, f: &mut fmt::Formatter) -> fmt::Result {
            const BITS: i16 = 52;
            const FRACT_MASK: u64 = 0xf_ffff_ffff_ffff;
            let (mantissa, exponent, _) = self.integer_decode();
            write!(
                f,
                "{}0x{:x}.{:013x}p{:+}",
                self.sign_string(),
                mantissa >> BITS,
                mantissa & FRACT_MASK,
                exponent + BITS
            )
        }
    }
}

use std::fmt;

/// Wrap a float value to be a hexadecimal formattable value.
///
/// ```rust
/// use hexf_format::*;
/// assert_eq!(format!("{:x}", Format(0.1f32)), "0x1.99999ap-4");
/// assert_eq!(format!("{:x}", Format(0.1f64)), "0x1.999999999999ap-4");
/// ```
pub struct Format<T: internal::FormatHexf>(pub T);

impl<F: internal::FormatHexf> fmt::LowerHex for Format<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Format a f32 or f64 value to hexadecimal.
pub fn to_hex_string<F: internal::FormatHexf>(value: F) -> String {
    format!("{:x}", Format(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_finite_hexf32() {
        use hexf_parse::parse_hexf32;
        use rand::Rng;

        for _ in 0..20000 {
            let bytes = rand::thread_rng().gen::<[u32; 1]>();
            let f = f32::from_bits(bytes[0]);
            if !f.is_finite() {
                continue;
            }

            let hex = format!("{:x}", Format(f));
            let roundtrip = parse_hexf32(&hex, false).unwrap();
            assert_eq!(f, roundtrip, "{} {} {}", f, hex, roundtrip);
        }
    }

    #[test]
    fn test_format_finite_hexf64() {
        use hexf_parse::parse_hexf64;
        use rand::Rng;

        for _ in 0..20000 {
            let bytes = rand::thread_rng().gen::<[u64; 1]>();
            let f = f64::from_bits(bytes[0]);
            if !f.is_finite() {
                continue;
            }
            let hex = format!("{:x}", Format(f));
            let roundtrip = parse_hexf64(&hex, false).unwrap();
            assert_eq!(f, roundtrip, "{} {} {}", f, hex, roundtrip);
        }
    }

    #[test]
    fn test_to_hex_string() {
        assert_eq!(to_hex_string(0.0f64), "0x0.0p+0");
        assert_eq!(to_hex_string(0.0f32), "0x0.0p+0");
        assert_eq!(to_hex_string(-0.0f64), "-0x0.0p+0");
        assert_eq!(to_hex_string(-0.0f32), "-0x0.0p+0");

        assert_eq!(
            to_hex_string(f64::INFINITY),
            "inf".parse::<f64>().unwrap().to_string()
        );
        assert_eq!(
            to_hex_string(-f64::INFINITY),
            "-inf".parse::<f64>().unwrap().to_string()
        );
        assert_eq!(
            to_hex_string(f64::NAN),
            "NaN".parse::<f64>().unwrap().to_string()
        );
    }
}
