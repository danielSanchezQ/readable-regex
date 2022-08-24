# Readable regex

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/readable-regex.svg)](https://crates.io/crates/readable-regex)
[![GHA Build Status](https://github.com/danielsanchezq/readable-regex/workflows/CI/badge.svg)](https://github.com/danielsanchezq/readable-regex/actions?query=workflow%3ACI)
[![Docs Badge](https://docs.rs/readable-regex/badge.svg)](https://docs.rs/readable-regex)

### Regex for human beings.

## Why?

Regex is useful. But, sincerely, since code it is more read than written regex could 
be more understandable in a verbose mode.

`readable-regex` crate is a set of tools to build those regexes in a verbose way. Which aims
to improve readability of code.

It builds on top on the already excellent [`regex`](https://crates.io/crates/regex) and [`fancy-regex`](https://crates.io/crates/fancy-regex) crates.


## Add dependency
Add the dependency to your `Cargo.toml` file: 

```toml
readable-regex = "0.1.0"
```

## Usage

### Available APIs

The main wrapper is the `ReadableRe` enum.

There are tow main options to build regexes with it, either using the **enum** itself:

```rust
use readable_regex::ReadableRe;
let query = ReadableRe::Raw("<some regex expression>");
```

or by using the functions wrappers around it:

```rust
use readable_regex::raw_regex;
let query = raw_regex("<some regex expression>");
```

Also, any combination of them:

```rust
use readable_regex::{digit, group};
use readable_regex::ReadableRe::*;
let query = group(digit() + Digit + digit());
println!("{}", query.to_string());
```

### Example
How to build a simple date match (as implemented in the `datetime` module under `presets` feature):
```
use once_cell::sync::Lazy;
use readable_regex::*;
use readable_regex::ReadableRe::*;

/// Month day, `01`-`31`
pub const DAY: Lazy<ReadableRe> = Lazy::new(|| {
    either([
        Raw("0") + chars("1-9"),
        chars("12") + chars("1-9"),
        Raw("3") + chars("01"),
    ])
});

/// Month numeral, `01`-`12`
pub const MONTH: Lazy<ReadableRe> =
    Lazy::new(|| either([Raw("0") + chars("1-9"), Raw("1") + chars("0-2")]));

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

assert!(DATE_Y_M_D.compile().unwrap().is_match("2022/04/18"));
```

## Acknowledges

This library was highly inspired by the python [`Humre`](https://github.com/asweigart/humre) package.
