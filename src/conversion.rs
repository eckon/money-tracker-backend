pub struct Conversion;

impl Conversion {
    #[allow(clippy::cast_precision_loss)]
    pub fn to_float(value: i64) -> f64 {
        (value as f64) / 100.0
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn to_int(value: f64) -> i64 {
        (value * 100.0_f64).round() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_float_to_int() {
        let float_amount = 17.4;

        #[allow(clippy::cast_possible_truncation)]
        let int_amount: i64 = (float_amount * 100.0) as i64;

        // this should work but the 17,4 * 100 will result in under 1740
        assert_ne!(1740, int_amount);
    }

    #[test]
    fn test_float_to_int() {
        let amount = 17.4;
        assert_eq!(1740, Conversion::to_int(amount));
    }

    #[test]
    fn test_int_to_float() {
        let amount = 1740;
        assert_eq!(17.40, Conversion::to_float(amount));
    }
}
