//! ## Regex for human beings.
//!
//! Regex is useful. But, sincerely, since code it is more read than written regex could
//! be more understandable in a verbose mode.
//!
//! `readable-regex` crate is a set of tools to build those regexes in a verbose way. Which aims
//! to improve readability of code.
//!
//! ### Examples
//!
//!
//! ### Features
//! * `re` => Use [`regex`] crate backend.
//! * `re-fancy` => Use [`fancy_regex`] crate backend and expands this crate functionality.

mod constants;
pub mod readable;
pub mod solvers;

pub use readable::ReadableRe;
use std::ops::RangeBounds;

pub fn digit<'a>() -> ReadableRe<'a> {
    ReadableRe::Digit
}

pub fn word<'a>() -> ReadableRe<'a> {
    ReadableRe::Word
}

pub fn whitespace<'a>() -> ReadableRe<'a> {
    ReadableRe::Whitespace
}

pub fn non_digit<'a>() -> ReadableRe<'a> {
    ReadableRe::NonDigit
}

pub fn non_word<'a>() -> ReadableRe<'a> {
    ReadableRe::NonWord
}

pub fn non_whitespace<'a>() -> ReadableRe<'a> {
    ReadableRe::NonWhitespace
}

pub fn boundary<'a>() -> ReadableRe<'a> {
    ReadableRe::Boundary
}

pub fn ascii_letter<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiLetter
}

pub fn ascii_non_letter<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonLetter
}

pub fn ascii_uppercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiUppercase
}

pub fn ascii_non_uppercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonUppercase
}

pub fn ascii_lowercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiLowercase
}

pub fn ascii_non_lowercase<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonLowercase
}

pub fn ascii_alphanumeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiAlphanumeric
}

pub fn ascii_non_alphanumeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonAlphanumeric
}

pub fn ascii_numeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNumeric
}

pub fn ascii_non_numeric<'a>() -> ReadableRe<'a> {
    ReadableRe::AsciiNonNumeric
}

pub fn hexadecimal<'a>() -> ReadableRe<'a> {
    ReadableRe::Hexadecimal
}

pub fn non_hexadecimal<'a>() -> ReadableRe<'a> {
    ReadableRe::NonHexadecimal
}

pub fn anything<'a>() -> ReadableRe<'a> {
    ReadableRe::Anything
}

pub fn everything<'a>() -> ReadableRe<'a> {
    ReadableRe::Everything
}

pub fn something_greedy<'a>() -> ReadableRe<'a> {
    ReadableRe::SomethingGreedy
}

pub fn something<'a>() -> ReadableRe<'a> {
    ReadableRe::Something
}

pub fn any_char<'a>() -> ReadableRe<'a> {
    ReadableRe::AnyChar
}

pub fn period<'a>() -> ReadableRe<'a> {
    ReadableRe::Period
}

pub fn caret<'a>() -> ReadableRe<'a> {
    ReadableRe::Caret
}

pub fn dollar<'a>() -> ReadableRe<'a> {
    ReadableRe::Dollar
}

pub fn asterisk<'a>() -> ReadableRe<'a> {
    ReadableRe::Asterisk
}

pub fn plus_sign<'a>() -> ReadableRe<'a> {
    ReadableRe::PlusSign
}

pub fn minus_sign<'a>() -> ReadableRe<'a> {
    ReadableRe::MinusSign
}

pub fn question_mark<'a>() -> ReadableRe<'a> {
    ReadableRe::QuestionMark
}

pub fn open_brace<'a>() -> ReadableRe<'a> {
    ReadableRe::OpenBrace
}

pub fn close_brace<'a>() -> ReadableRe<'a> {
    ReadableRe::CloseBrace
}

pub fn open_bracket<'a>() -> ReadableRe<'a> {
    ReadableRe::OpenBracket
}

pub fn close_bracket<'a>() -> ReadableRe<'a> {
    ReadableRe::CloseBracket
}

pub fn open_parenthesis<'a>() -> ReadableRe<'a> {
    ReadableRe::OpenParenthesis
}

pub fn close_parenthesis<'a>() -> ReadableRe<'a> {
    ReadableRe::CloseParenthesis
}

pub fn back_slash<'a>() -> ReadableRe<'a> {
    ReadableRe::BackSlash
}

pub fn pipe<'a>() -> ReadableRe<'a> {
    ReadableRe::Pipe
}

pub fn new_line<'a>() -> ReadableRe<'a> {
    ReadableRe::Newline
}

pub fn tab<'a>() -> ReadableRe<'a> {
    ReadableRe::Tab
}

pub fn quote<'a>() -> ReadableRe<'a> {
    ReadableRe::Quote
}

pub fn double_quote<'a>() -> ReadableRe<'a> {
    ReadableRe::DoubleQuote
}

#[cfg(feature = "re-fancy")]
pub fn back1<'a>() -> ReadableRe<'a> {
    ReadableRe::Back1
}

#[cfg(feature = "re-fancy")]
pub fn back2<'a>() -> ReadableRe<'a> {
    ReadableRe::Back2
}

#[cfg(feature = "re-fancy")]
pub fn back3<'a>() -> ReadableRe<'a> {
    ReadableRe::Back3
}

#[cfg(feature = "re-fancy")]
pub fn back4<'a>() -> ReadableRe<'a> {
    ReadableRe::Back4
}

#[cfg(feature = "re-fancy")]
pub fn back5<'a>() -> ReadableRe<'a> {
    ReadableRe::Back5
}

#[cfg(feature = "re-fancy")]
pub fn back6<'a>() -> ReadableRe<'a> {
    ReadableRe::Back6
}

#[cfg(feature = "re-fancy")]
pub fn back7<'a>() -> ReadableRe<'a> {
    ReadableRe::Back7
}

#[cfg(feature = "re-fancy")]
pub fn back8<'a>() -> ReadableRe<'a> {
    ReadableRe::Back8
}

#[cfg(feature = "re-fancy")]
pub fn back9<'a>() -> ReadableRe<'a> {
    ReadableRe::Back9
}

pub fn raw_regex(s: &str) -> ReadableRe {
    ReadableRe::Raw(s)
}

pub fn string_regex<'a>(s: String) -> ReadableRe<'a> {
    ReadableRe::String(s)
}

pub fn concat<'a>(iter: impl IntoIterator<Item = ReadableRe<'a>>) -> ReadableRe<'a> {
    ReadableRe::Concat(solvers::Concat::new(iter))
}

#[cfg(feature = "re-fancy")]
pub fn back_reference<'a>(n: usize) -> ReadableRe<'a> {
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

pub fn chars<'a>(iter: impl IntoIterator<Item = char>) -> ReadableRe<'a> {
    ReadableRe::Chars(solvers::Chars::new(iter))
}

pub fn not_chars<'a>(iter: impl IntoIterator<Item = char>) -> ReadableRe<'a> {
    ReadableRe::NotChars(solvers::NotChars::new(iter))
}

#[cfg(feature = "re-fancy")]
pub fn atomic_group(re: ReadableRe) -> ReadableRe {
    ReadableRe::AtomicGroup(solvers::AtomicGroup::new(re))
}
