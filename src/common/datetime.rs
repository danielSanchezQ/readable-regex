use crate::ReadableRe::*;
use crate::{chars, either, exactly, group, ReadableRe};
use once_cell::sync::Lazy;

/// Month day, `01`-`31`
const DAY: Lazy<ReadableRe> = Lazy::new(|| {
    either([
        Raw("0") + chars("1-9"),
        chars("12") + Digit,
        Raw("3") + chars("01"),
    ])
});

/// Month numeral, `01`-`12`
const MONTH: Lazy<ReadableRe> =
    Lazy::new(|| either([Raw("0") + chars("1-9"), Raw("1") + chars("0-2")]));

/// Calendar month, `Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec`
const MONTH_CALENDAR: Lazy<ReadableRe> = Lazy::new(|| {
    either(
        [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .map(Into::into),
    )
});

/// Years from `1000` to `2999`
const YEAR: Lazy<ReadableRe> = Lazy::new(|| chars("12") + exactly(3, Digit));

/// Date Format `YYYY-MM-dd`
const DATE_Y_M_D: Lazy<ReadableRe> = Lazy::new(|| {
    group(
        group(YEAR.clone())
            + chars("-/.\\\\")
            + group(MONTH.clone())
            + chars("-/.\\\\")
            + group(DAY.clone()),
    )
});

/// Date Format `YYYY-MM-dd`
const DATE_D_M_Y: Lazy<ReadableRe> = Lazy::new(|| {
    group(
        group(DAY.clone())
            + chars("-/.\\\\")
            + group(MONTH.clone())
            + chars("-/.\\\\")
            + group(YEAR.clone()),
    )
});

/// Minutes or seconds representation `00`-`59`
const MIN_SEC: Lazy<ReadableRe> = Lazy::new(|| chars("0-5") + chars("0-9"));

/// Hours 12h format
const HOURS_12: Lazy<ReadableRe> =
    Lazy::new(|| either([Raw("0") + chars("0-9"), Raw("1") + chars("12")]));

/// Hours 24h format
const HOURS_24: Lazy<ReadableRe> = Lazy::new(|| {
    either([
        Raw("0") + chars("0-9"),
        Raw("1") + chars("0-9"),
        Raw("2") + chars("0-4"),
    ])
});

#[cfg(test)]
mod tests {
    use crate::common::datetime::{DATE_D_M_Y, DATE_Y_M_D, DAY, MONTH, YEAR};
    use std::fmt::format;

    #[test]
    fn day() {
        let query = DAY.compile().unwrap();
        for i in 1..32 {
            assert!(query.is_match(&format!("{i:02}")));
        }
        for i in 32..99 {
            assert!(!query.is_match(&format!("{i:02}")));
        }
    }

    #[test]
    fn month() {
        let query = MONTH.compile().unwrap();
        for i in 1..13 {
            assert!(query.is_match(&format!("{i:02}")));
        }
        for i in 13..99 {
            assert!(!query.is_match(&format!("{i:02}")));
        }
    }

    #[test]
    fn year() {
        let query = YEAR.compile().unwrap();
        for i in 1000..3000 {
            assert!(query.is_match(&format!("{i:02}")));
        }
        assert!(!query.is_match("999"));
        assert!(!query.is_match("3000"));
    }

    #[test]
    fn date_y_m_d() {
        let query = DATE_Y_M_D.compile().unwrap();
        assert!(query.is_match("2022/04/18"));
        assert!(query.is_match("2022-04-18"));
        assert!(query.is_match("2022\\04\\18"));
    }

    #[test]
    fn date_d_m_y() {
        let query = DATE_D_M_Y.compile().unwrap();
        assert!(query.is_match("18/04/2022"));
        assert!(query.is_match("18-04-2022"));
        assert!(query.is_match("18\\04\\2022"));
    }
}
