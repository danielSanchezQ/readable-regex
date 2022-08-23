use crate::ReadableRe::*;
use crate::*;
use once_cell::sync::Lazy;

const POSITIVE_INTEGER: Lazy<ReadableRe> = Lazy::new(|| starts_and_ends_with(one_or_more(Digit)));

const NEGATIVE_INTEGER: Lazy<ReadableRe> =
    Lazy::new(|| starts_and_ends_with(MinusSign + one_or_more(Digit)));

const INTEGER: Lazy<ReadableRe> =
    Lazy::new(|| starts_and_ends_with(optional(MinusSign) + one_or_more(Digit)));

const POSITIVE_DECIMAL: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(
        group(zero_or_more(Digit)) + chars(".,".chars()) + group(one_or_more(Digit)),
    )
});

const NEGATIVE_DECIMAL: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(
        MinusSign + group(zero_or_more(Digit)) + chars(".,".chars()) + group(one_or_more(Digit)),
    )
});

const DECIMAL: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(
        optional(MinusSign)
            + group(zero_or_more(Digit))
            + chars(".,".chars())
            + group(one_or_more(Digit)),
    )
});

const FRACTION: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(group(
        one_or_more(Digit) + chars("/\\\\".chars()) + group(one_or_more(Digit)),
    ))
});

#[cfg(test)]
mod tests {
    use crate::common::numeric::{
        DECIMAL, FRACTION, INTEGER, NEGATIVE_DECIMAL, NEGATIVE_INTEGER, POSITIVE_DECIMAL,
        POSITIVE_INTEGER,
    };

    #[test]
    fn positive_integer() {
        let query = POSITIVE_INTEGER.compile().unwrap();
        assert!(query.is_match("1234567890"));
        assert!(!query.is_match("1a234567890"));
    }

    #[test]
    fn negative_integer() {
        let query = NEGATIVE_INTEGER.compile().unwrap();
        assert!(query.is_match("-1234567890"));
        assert!(!query.is_match("1234567890"));
    }

    #[test]
    fn integer() {
        let query = INTEGER.compile().unwrap();
        assert!(query.is_match("-1234567890"));
        assert!(query.is_match("1234567890"));
        assert!(!query.is_match("-1234567890a"));
    }

    #[test]
    fn positive_decimal() {
        let query = POSITIVE_DECIMAL.compile().unwrap();
        assert!(query.is_match("12345.67890"));
        assert!(!query.is_match("1a2345.67890"));
    }

    #[test]
    fn negative_decimal() {
        let query = NEGATIVE_DECIMAL.compile().unwrap();
        assert!(query.is_match("-12345.67890"));
        assert!(!query.is_match("12345.67890"));
    }

    #[test]
    fn decimal() {
        let query = DECIMAL.compile().unwrap();
        assert!(query.is_match("-12345.67890"));
        assert!(query.is_match("12345.67890"));
        assert!(!query.is_match("-12345.67890a"));
    }

    #[test]
    fn fraction() {
        let query = FRACTION.compile().unwrap();
        assert!(query.is_match("9/6"));
        assert!(query.is_match("9\\6"));
        assert!(!query.is_match("9/a"));
    }
}
