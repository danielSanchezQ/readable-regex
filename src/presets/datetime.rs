use crate::ReadableRe::*;
use crate::{chars, either, exactly, group, optional, starts_and_ends_with, ReadableRe};
use once_cell::sync::Lazy;

/// Month day, `01`-`31`
pub const DAY: Lazy<ReadableRe> = Lazy::new(|| {
    either([
        Raw("0") + chars("1-9"),
        chars("12") + Digit,
        Raw("3") + chars("01"),
    ])
});

/// Month numeral, `01`-`12`
pub const MONTH: Lazy<ReadableRe> =
    Lazy::new(|| either([Raw("0") + chars("1-9"), Raw("1") + chars("0-2")]));

/// Calendar month, `Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec`
pub const MONTH_CALENDAR: Lazy<ReadableRe> = Lazy::new(|| {
    either(
        [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .map(Into::into),
    )
});

/// Years from `1000` to `2999`
pub const YEAR: Lazy<ReadableRe> = Lazy::new(|| chars("12") + exactly(3, Digit));

/// Date Format `YYYY-MM-dd`
pub const DATE_Y_M_D: Lazy<ReadableRe> = Lazy::new(|| {
    group(
        group(YEAR.clone())
            + chars(r"-/.\\")
            + group(MONTH.clone())
            + chars(r"-/.\\")
            + group(DAY.clone()),
    )
});

/// Date Format `YYYY-MM-dd`
pub const DATE_D_M_Y: Lazy<ReadableRe> = Lazy::new(|| {
    group(
        group(DAY.clone())
            + chars(r"-/.\\")
            + group(MONTH.clone())
            + chars(r"-/.\\")
            + group(YEAR.clone()),
    )
});

/// Minutes or seconds representation `00`-`59`
pub const MIN_SEC: Lazy<ReadableRe> = Lazy::new(|| chars("0-5") + chars("0-9"));

/// Hours 12h format
pub const HOURS_12: Lazy<ReadableRe> =
    Lazy::new(|| either([Raw("0") + chars("0-9"), Raw("1") + chars("0-2")]));

/// Hours 24h format
pub const HOURS_24: Lazy<ReadableRe> = Lazy::new(|| {
    either([
        Raw("0") + chars("0-9"),
        Raw("1") + chars("0-9"),
        Raw("2") + chars("0-4"),
    ])
});

const MERIDIEMS: Lazy<ReadableRe> =
    Lazy::new(|| either([chars("ap") + "m".into(), chars("AP") + chars("Mm")]));

pub const HH_MM_12: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(
        group(HOURS_12.clone())
            + ":".into()
            + group(MIN_SEC.clone())
            + optional(" ".into())
            + group(MERIDIEMS.clone()),
    )
});

pub const HH_MM_24: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(group(HOURS_24.clone()) + ":".into() + group(MIN_SEC.clone()))
});

pub const HH_MM_SS_24: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(
        group(HOURS_24.clone())
            + ":".into()
            + group(MIN_SEC.clone())
            + ":".into()
            + group(MIN_SEC.clone()),
    )
});

#[cfg(test)]
mod tests {
    use crate::presets::datetime::{
        DATE_D_M_Y, DATE_Y_M_D, DAY, HH_MM_12, HH_MM_24, HH_MM_SS_24, HOURS_12, HOURS_24,
        MERIDIEMS, MIN_SEC, MONTH, YEAR,
    };

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

    #[test]
    fn min_sec() {
        let query = MIN_SEC.compile().unwrap();
        for i in 0..60 {
            assert!(query.is_match(&format!("{i:02}")));
        }
        assert!(!query.is_match("60"));
    }

    #[test]
    fn hour_12() {
        let query = HOURS_12.compile().unwrap();
        for i in 0..13 {
            let v = format!("{i:02}");
            assert!(query.is_match(&v), "Failed matching: {}", v);
        }
        for i in 13..25 {
            let v = format!("{i:02}");
            assert!(!query.is_match(&v), "Failed matching: {}", v);
        }
    }

    #[test]
    fn hour_24() {
        let query = HOURS_24.compile().unwrap();
        for i in 0..25 {
            let v = format!("{i:02}");
            assert!(query.is_match(&v), "Failed matching: {}", v);
        }
        for i in 25..99 {
            let v = format!("{i:02}");
            assert!(!query.is_match(&v), "Failed matching: {}", v);
        }
    }

    #[test]
    fn meridiems() {
        let query = MERIDIEMS.compile().unwrap();
        assert!(query.is_match("am"));
        assert!(query.is_match("Am"));
        assert!(query.is_match("AM"));
        assert!(query.is_match("pm"));
        assert!(query.is_match("Pm"));
        assert!(query.is_match("PM"));
        assert!(!query.is_match("aM"));
        assert!(!query.is_match("pM"));
    }

    #[test]
    fn hh_mm_12() {
        let query = HH_MM_12.compile().unwrap();
        assert!(query.is_match("11:30 pm"));
        assert!(query.is_match("11:30 am"));
        assert!(!query.is_match("18:30 pm"));
    }

    #[test]
    fn hh_mm_24() {
        let query = HH_MM_24.compile().unwrap();
        assert!(query.is_match("11:30"));
        assert!(query.is_match("23:30"));
        assert!(!query.is_match("11:30 pm"));
    }

    #[test]
    fn hh_mm_ss_24() {
        let query = HH_MM_SS_24.compile().unwrap();
        assert!(query.is_match("11:30:59"));
        assert!(query.is_match("23:30:59"));
        assert!(!query.is_match("18:30:89"));
    }
}
