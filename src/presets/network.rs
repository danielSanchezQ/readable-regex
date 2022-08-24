use crate::ReadableRe::{Digit, Raw};
use crate::{chars, either, exactly, group, starts_and_ends_with, ReadableRe};
use once_cell::sync::Lazy;

const U8: Lazy<ReadableRe> = Lazy::new(|| {
    either([
        chars("0-9"),
        chars("1-9") + chars("0-9"),
        Raw("1") + exactly(2, Digit),
        Raw("2") + chars("0-4") + chars("0-9"),
        Raw("25") + chars("0-5"),
    ])
});

const IPV4: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(group(
        group(U8.clone())
            + ".".into()
            + group(U8.clone())
            + ".".into()
            + group(U8.clone())
            + ".".into()
            + group(U8.clone()),
    ))
});
