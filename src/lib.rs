//! ## Regex for human beings.
//!
//! Regex is useful. But, sincerely, since code it is more read than written regex could
//! be more understandable in a verbose mode.
//!
//! `readable-regex` crate is a set of tools to build those regexes in a verbose way. Which aims
//! to improve readability of code.
//!
//! ### Available APIs
//!
//!
//! ### Examples
//! How to build a simple date match (as implemented in the `datetime` module under `presets` feature):
//! ```
//! use once_cell::sync::Lazy;
//! use readable_regex::*;
//! use readable_regex::ReadableRe::*;
//!
//! /// Month day, `01`-`31`
//! pub const DAY: Lazy<ReadableRe> = Lazy::new(|| {
//!     either([
//!         Raw("0") + chars("1-9"),
//!         chars("12") + Digit,
//!         Raw("3") + chars("01"),
//!     ])
//! });
//!
//! /// Month numeral, `01`-`12`
//! pub const MONTH: Lazy<ReadableRe> =
//!     Lazy::new(|| either([Raw("0") + chars("1-9"), Raw("1") + chars("0-2")]));
//!
//! /// Years from `1000` to `2999`
//! pub const YEAR: Lazy<ReadableRe> = Lazy::new(|| chars("12") + exactly(3, Digit));
//!
//! /// Date Format `YYYY-MM-dd`
//! pub const DATE_Y_M_D: Lazy<ReadableRe> = Lazy::new(|| {
//!     group(
//!         group(YEAR.clone())
//!             + chars(r"-/.\\")
//!             + group(MONTH.clone())
//!             + chars(r"-/.\\")
//!             + group(DAY.clone()),
//!     )
//! });
//!
//! assert!(DATE_Y_M_D.compile().unwrap().is_match("2022/04/18"));
//! ```
//!
//! ### Features
//! * `re` => Use [`regex`] crate backend.
//! * `re-fancy` => Use [`fancy_regex`] crate backend and expands this crate functionality.

mod constants;
#[cfg(feature = "presets")]
pub mod presets;
pub mod readable;
pub mod solvers;

pub use readable::ReadableRe;
use std::ops::RangeBounds;

pub const fn digit<'a>() -> ReadableRe<'a> {
    ReadableRe::Digit
}

pub const fn word<'a>() -> ReadableRe<'a> {
    ReadableRe::Word
}

pub const fn whitespace<'a>() -> ReadableRe<'a> {
    ReadableRe::Whitespace
}

pub const fn non_digit<'a>() -> ReadableRe<'a> {
    ReadableRe::NonDigit
}

pub const fn non_word<'a>() -> ReadableRe<'a> {
    ReadableRe::NonWord
}

pub const fn non_whitespace<'a>() -> ReadableRe<'a> {
    ReadableRe::NonWhitespace
}

pub const fn boundary<'a>() -> ReadableRe<'a> {
    ReadableRe::Boundary
}

pub const fn ascii_letter<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiLetter
}

pub const fn ascii_non_letter<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonLetter
}

pub const fn ascii_uppercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiUppercase
}

pub const fn ascii_non_uppercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonUppercase
}

pub const fn ascii_lowercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiLowercase
}

pub const fn ascii_non_lowercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonLowercase
}

pub const fn ascii_alphanumeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiAlphanumeric
}

pub const fn ascii_non_alphanumeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonAlphanumeric
}

pub const fn ascii_numeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNumeric
}

pub const fn ascii_non_numeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonNumeric
}

pub const fn hexadecimal<'a>() -> ReadableRe<'a> {
    ReadableRe::Hexadecimal
}

pub const fn non_hexadecimal<'a>() -> ReadableRe<'a> {
    ReadableRe::NonHexadecimal
}

pub const fn anything<'a>() -> ReadableRe<'a> {
    ReadableRe::Anything
}

pub const fn everything<'a>() -> ReadableRe<'a> {
    ReadableRe::Everything
}

pub const fn something_greedy<'a>() -> ReadableRe<'a> {
    ReadableRe::SomethingGreedy
}

pub const fn something<'a>() -> ReadableRe<'a> {
    ReadableRe::Something
}

pub const fn any_char<'a>() -> ReadableRe<'a> {
    ReadableRe::AnyChar
}

pub const fn period<'a>() -> ReadableRe<'a> {
    ReadableRe::Period
}

pub const fn caret<'a>() -> ReadableRe<'a> {
    ReadableRe::Caret
}

pub const fn dollar<'a>() -> ReadableRe<'a> {
    ReadableRe::Dollar
}

pub const fn asterisk<'a>() -> ReadableRe<'a> {
    ReadableRe::Asterisk
}

pub const fn plus_sign<'a>() -> ReadableRe<'a> {
    ReadableRe::PlusSign
}

pub const fn minus_sign<'a>() -> ReadableRe<'a> {
    ReadableRe::MinusSign
}

pub const fn question_mark<'a>() -> ReadableRe<'a> {
    ReadableRe::QuestionMark
}

pub const fn open_brace<'a>() -> ReadableRe<'a> {
    ReadableRe::OpenBrace
}

pub const fn close_brace<'a>() -> ReadableRe<'a> {
    ReadableRe::CloseBrace
}

pub const fn open_bracket<'a>() -> ReadableRe<'a> {
    ReadableRe::OpenBracket
}

pub const fn close_bracket<'a>() -> ReadableRe<'a> {
    ReadableRe::CloseBracket
}

pub const fn open_parenthesis<'a>() -> ReadableRe<'a> {
    ReadableRe::OpenParenthesis
}

pub const fn close_parenthesis<'a>() -> ReadableRe<'a> {
    ReadableRe::CloseParenthesis
}

pub const fn back_slash<'a>() -> ReadableRe<'a> {
    ReadableRe::BackSlash
}

pub const fn pipe<'a>() -> ReadableRe<'a> {
    ReadableRe::Pipe
}

pub const fn new_line<'a>() -> ReadableRe<'a> {
    ReadableRe::Newline
}

pub const fn tab<'a>() -> ReadableRe<'a> {
    ReadableRe::Tab
}

pub const fn quote<'a>() -> ReadableRe<'a> {
    ReadableRe::Quote
}

pub const fn double_quote<'a>() -> ReadableRe<'a> {
    ReadableRe::DoubleQuote
}

#[cfg(feature = "re-fancy")]
pub const fn back1<'a>() -> ReadableRe<'a> {
    ReadableRe::Back1
}

#[cfg(feature = "re-fancy")]
pub const fn back2<'a>() -> ReadableRe<'a> {
    ReadableRe::Back2
}

#[cfg(feature = "re-fancy")]
pub const fn back3<'a>() -> ReadableRe<'a> {
    ReadableRe::Back3
}

#[cfg(feature = "re-fancy")]
pub const fn back4<'a>() -> ReadableRe<'a> {
    ReadableRe::Back4
}

#[cfg(feature = "re-fancy")]
pub const fn back5<'a>() -> ReadableRe<'a> {
    ReadableRe::Back5
}

#[cfg(feature = "re-fancy")]
pub const fn back6<'a>() -> ReadableRe<'a> {
    ReadableRe::Back6
}

#[cfg(feature = "re-fancy")]
pub const fn back7<'a>() -> ReadableRe<'a> {
    ReadableRe::Back7
}

#[cfg(feature = "re-fancy")]
pub const fn back8<'a>() -> ReadableRe<'a> {
    ReadableRe::Back8
}

#[cfg(feature = "re-fancy")]
pub const fn back9<'a>() -> ReadableRe<'a> {
    ReadableRe::Back9
}

pub const fn raw_regex(s: &str) -> ReadableRe {
    ReadableRe::Raw(s)
}

pub const fn string_regex<'a>(s: String) -> ReadableRe<'a> {
    ReadableRe::String(s)
}

pub fn concat<'a>(iter: impl IntoIterator<Item = ReadableRe<'a>>) -> ReadableRe<'a> {
    ReadableRe::Concat(solvers::Concat::new(iter))
}

#[cfg(feature = "re-fancy")]
pub const fn back_reference<'a>(n: usize) -> ReadableRe<'a> {
    ReadableRe::BackReference(solvers::BackReference(n))
}

pub fn escape_str(s: &str) -> ReadableRe {
    ReadableRe::Escape(solvers::Escape::new_str(s))
}

pub fn escape(re: ReadableRe) -> ReadableRe {
    ReadableRe::Escape(solvers::Escape::new(re))
}

pub fn group(re: ReadableRe) -> ReadableRe {
    ReadableRe::Group(solvers::Group::new(re))
}

#[cfg(feature = "re-fancy")]
pub fn positive_look_ahead(re: ReadableRe) -> ReadableRe {
    ReadableRe::PositiveLookAhead(solvers::PositiveLookAhead::new(re))
}

#[cfg(feature = "re-fancy")]
pub fn negative_look_ahead(re: ReadableRe) -> ReadableRe {
    ReadableRe::NegativeLookAhead(solvers::NegativeLookAhead::new(re))
}

#[cfg(feature = "re-fancy")]
pub fn positive_look_behind(re: ReadableRe) -> ReadableRe {
    ReadableRe::PositiveLookBehind(solvers::PositiveLookBehind::new(re))
}

#[cfg(feature = "re-fancy")]
pub fn negative_look_behind(re: ReadableRe) -> ReadableRe {
    ReadableRe::NegativeLookBehind(solvers::NegativeLookBehind::new(re))
}

pub fn named_group<'a>(name: &'a str, re: ReadableRe<'a>) -> ReadableRe<'a> {
    ReadableRe::NamedGroup(solvers::NamedGroup::new(name, re))
}

pub fn non_capture_group(re: ReadableRe) -> ReadableRe {
    ReadableRe::NonCaptureGroup(solvers::NonCaptureGroup::new(re))
}

pub fn optional(re: ReadableRe) -> ReadableRe {
    ReadableRe::Optional(solvers::Optional::new(re))
}

pub fn either<'a>(iter: impl IntoIterator<Item = ReadableRe<'a>>) -> ReadableRe<'a> {
    ReadableRe::Either(solvers::Either::new(iter))
}

pub fn exactly(n: usize, re: ReadableRe) -> ReadableRe {
    ReadableRe::Exactly(solvers::Exactly::new(n, re))
}

pub fn ranged<R>(range: R, re: ReadableRe) -> ReadableRe
where
    R: RangeBounds<usize> + 'static,
{
    ReadableRe::Ranged(solvers::Ranged::new(range, re))
}

pub fn at_least(n: usize, re: ReadableRe) -> ReadableRe {
    ReadableRe::Ranged(solvers::Ranged::new(n.., re))
}

pub fn at_most(n: usize, re: ReadableRe) -> ReadableRe {
    ReadableRe::Ranged(solvers::Ranged::new(..n, re))
}

pub fn zero_or_more(re: ReadableRe) -> ReadableRe {
    ReadableRe::ZeroOrMore(solvers::ZeroOrMore::new(re))
}

pub fn zero_or_more_lazy(re: ReadableRe) -> ReadableRe {
    ReadableRe::ZeroOrMoreLazy(solvers::ZeroOrMoreLazy::new(re))
}

pub fn one_or_more(re: ReadableRe) -> ReadableRe {
    ReadableRe::OneOrMore(solvers::OneOrMore::new(re))
}

pub fn one_or_more_lazy(re: ReadableRe) -> ReadableRe {
    ReadableRe::OneOrMoreLazy(solvers::OneOrMoreLazy::new(re))
}

pub fn starts_with(re: ReadableRe) -> ReadableRe {
    ReadableRe::StartsWith(solvers::StartsWith::new(re))
}

pub fn ends_with(re: ReadableRe) -> ReadableRe {
    ReadableRe::EndsWith(solvers::EndsWith::new(re))
}

pub fn starts_and_ends_with(re: ReadableRe) -> ReadableRe {
    ReadableRe::StartsAndEndsWith(solvers::StartsAndEndsWith::new(re))
}

pub fn chars(re: &str) -> ReadableRe {
    ReadableRe::Chars(solvers::Chars::new(re))
}

pub fn not_chars<'a>(iter: impl IntoIterator<Item = char>) -> ReadableRe<'a> {
    ReadableRe::NotChars(solvers::NotChars::new(iter))
}

#[cfg(feature = "re-fancy")]
pub fn atomic_group(re: ReadableRe) -> ReadableRe {
    ReadableRe::AtomicGroup(solvers::AtomicGroup::new(re))
}
