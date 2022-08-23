use crate::ReadableRe::*;
use crate::{chars, either, exactly, group, ReadableRe};
use once_cell::sync::Lazy;

/// Date Format `YYYY-MM-dd`
const DATE: Lazy<ReadableRe> = Lazy::new(|| {
    group(group(chars("12") + exactly(3, Digit)))
        + "-".into()
        + group(either([Raw("0") + chars("1-9"), Raw("1") + chars("0-2")]))
        + "-".into()
        + group(either([
            Raw("0") + chars("1-9"),
            chars("12") + Digit,
            Raw("3") + chars("01"),
        ]))
});
