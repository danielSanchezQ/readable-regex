//! ### Regex for human beings.
//!
//! Regex is useful. But, sincerely, since code it is more read than written regex could
//! be more understandable in a verbose mode.
//!
//! `readable-regex` crate is a set of tools to build those regexes in a verbose way. Which aims
//! to improve readability of code.

pub mod builders;
mod constants;

use std::fmt::{Display, Formatter};

use regex::Error;

#[cfg(feature = "re")]
use regex::Regex;

#[cfg(all(feature = "re-fancy", not(feature = "re")))]
use fancy_regex::Regex;

#[cfg(any(feature = "re", feature = "re-fancy"))]
/// Extension trait, compile any `Display` object into a regex
/// Depending on the feature selected [`re`] or [`re-fancy`] one or the other regex backend is used.
pub trait ReadableRegex: Display {
    fn compile(&self) -> Result<Regex, Error> {
        Regex::new(&format!("{self}"))
    }
}

/// Enum wrapper around regex expressions, it is a recursive version of regexes
pub enum ReadableRe<'a> {
    /// digit match, `"\d"`
    Digit,
    /// word match, `"\w"`
    Word,
    /// whitespace match, `"\s"`
    Whitespace,
    /// non digit match, `"\D"`
    NonDigit,
    /// non word match, `"\W"`
    NonWord,
    /// non whitespace match, `"\S"`
    NonWhitespace,
    /// boundary match, `"\b"`
    Boundary,

    /// ascii letters match, `"[A-Za-z]"`
    AsciiLetter,
    /// ascii non letters match, `"[^A-Za-z]"`
    AsciiNonLetter,
    /// ascii uppercase letters match, `"[A-Z]"`
    AsciiUppercase,
    /// ascii non uppercase letters match, `"[^A-Z]"`
    AsciiNonUppercase,
    /// ascii lowercase letters match, `"[a-z]"`
    AsciiLowercase,
    /// ascii non lowercase letters match, `"[^a-z]"`
    AsciiNonLowercase,
    /// ascii alphanumerics chars match, `"[A-Za-z0-9]"`
    AsciiAlphanumeric,
    /// ascii non alphanumerics chars match, `"[^A-Za-z0-9]"`
    AsciiNonAlphanumeric,
    /// ascii numeric match, `"[0-9]"`
    AsciiNumeric,
    /// ascii non numeric match, `"[^0-9]"`
    AsciiNonNumeric,

    /// hexadecimal match, `"[0-9A-Fa-f]"`
    Hexadecimal,
    /// non hexadecimal match, `"[^0-9A-Fa-f]"`
    NonHexadecimal,

    /// anything match, `".*?"`
    Anything,
    /// everything match, `".*"`
    Everything,
    /// something match, greedy, `".+"`
    SomethingGreedy,
    /// something match, `".+?"`
    Something,
    /// any char match, `"."`
    AnyChar,

    /// escaped period, `"\."`
    Period,
    /// escaped caret, `"\^"`
    Caret,
    /// escaped dollar, `"\$"`
    Dollar,
    /// escaped asterisk, `"\*"`
    Asterisk,
    /// escaped plus sign, `"\+"`
    PlusSign,
    /// escaped minus sign, `"\-"`
    MinusSign,
    /// escaped question mark, `"\?"`
    QuestionMark,
    /// escaped open brace, `"\{"`
    OpenBrace,
    /// escaped close brace, `"\}"`
    CloseBrace,
    /// escaped open bracket, `"\["`
    OpenBracket,
    /// escaped close bracket, `"\]"`
    CloseBracket,
    /// escaped open parenthesis, `"\("`
    OpenParenthesis,
    /// escaped close bracket, `"\)"`
    CloseParenthesis,
    /// escaped back slash, `"\\"`
    BackSlash,
    /// escaped pipe, `"\|"`
    Pipe,

    /// escaped new line, `"\n"`
    Newline,
    /// escaped tab, `"\t"`
    Tab,
    /// escaped quote, `"\'"`
    Quote,
    /// escaped double quote, `"\""`
    DoubleQuote,

    #[cfg(feature = "re-fancy")]
    /// back reference `"\1"`
    ///
    /// Available with feature `"re-fancy"`
    Back1,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\2"`
    ///
    /// Available with feature `"re-fancy"`
    Back2,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\3"`
    ///
    /// Available with feature `"re-fancy"`
    Back3,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\4"`
    ///
    /// Available with feature `"re-fancy"`
    Back4,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\5"`
    /// Available with feature `"re-fancy"`
    Back5,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\6"`
    ///
    /// Available with feature `"re-fancy"`
    Back6,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\7"`
    ///
    /// Available with feature `"re-fancy"`
    Back7,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\8"
    ///
    /// Available with feature `"re-fancy"`
    Back8,
    #[cfg(feature = "re-fancy")]
    /// back reference `"\9"`
    ///
    /// Available with feature `"re-fancy"`
    Back9,

    /// raw regex exp, `"exp"`
    Raw(&'a str),
    /// raw regex exp (owned), `"exp"`
    String(String),
    /// concatenation of regex exp, `"exp1exp2exp3"`, check [`builders::Concat`]
    Concat(builders::Concat<'a>),

    #[cfg(feature = "re-fancy")]
    /// `x` backreference, `"\x"`, check [`builders::BackReference`]
    ///
    /// Available with feature `"re-fancy"`
    BackReference(builders::BackReference),

    /// Special characters escape wrapper, check [`builders::Escape`]
    Escape(builders::Escape<'a>),
    /// Regex group, `"(expr)"`, check [`builders::Group`]
    Group(builders::Group<'a>),

    #[cfg(feature = "re-fancy")]
    /// look ahead, `"(?=expr)"`, check [`builders::PositiveLookAhead`]
    ///
    /// Available with feature `"re-fancy"`
    PositiveLookAhead(builders::PositiveLookAhead<'a>),
    #[cfg(feature = "re-fancy")]
    /// negative look ahead, `"(?!expr)"`, check [`builders::NegativeLookAhead`]
    ///
    /// Available with feature `"re-fancy"`
    NegativeLookAhead(builders::NegativeLookAhead<'a>),
    #[cfg(feature = "re-fancy")]
    /// look behind, `"(?<=expr)"`, check [`builders::PositiveLookBehind`]
    ///
    /// Available with feature `"re-fancy"`
    PositiveLookBehind(builders::PositiveLookBehind<'a>),
    #[cfg(feature = "re-fancy")]
    /// negative look behind, `"(?!expr)"`, check [`builders::NegativeLookBehind`]
    ///
    /// Available with feature `"re-fancy"`
    NegativeLookBehind(builders::NegativeLookBehind<'a>),
    /// named group match, `"(?P<name>expr)"`, check [`builders::NamedGroup`]
    NamedGroup(builders::NamedGroup<'a>),
    /// non-capture group, `"(?:expr)"`
    NonCaptureGroup(builders::NonCaptureGroup<'a>),
    /// optional match, `"expr?"`, check [`builders::Optional`]
    Optional(builders::Optional<'a>),
    /// either match, `"expr1|expr2|..."`, check [`builders::Either`]
    Either(builders::Either<'a>),
    /// exact number (n) of occurrences match, `"expr{n}"`, check [`builders::Exactly`]
    Exactly(builders::Exactly<'a>),
    /// variance number (min, max) of occurrences match, `"expr{min, max}"`, check [`builders::Ranged`]
    Ranged(builders::Ranged<'a>),
    /// zero or more occurrences match, `"expr*"`, check [`builders::ZeroOrMore`]
    ZeroOrMore(builders::ZeroOrMore<'a>),
    /// zero or more occurrences, lazy match, `"expr*?"`, , check [`builders::ZeroOrMoreLAzy`]
    ZeroOrMoreLazy(builders::ZeroOrMoreLazy<'a>),
    /// one or more occurrences match, `"expr+"`, check [`builders::OneOrMore`]
    OneOrMore(builders::OneOrMore<'a>),
    /// one or more occurrences, lazy match, `"expr+?"`, , check [`builders::OneOrMoreLazy`]
    OneOrMoreLazy(builders::OneOrMoreLazy<'a>),
    /// match start, `"^expr"`, check [`builders::StartsWith`]
    StartsWith(builders::StartsWith<'a>),
    /// match end, `"expr$"`, check [`builders::EndsWith`]
    EndsWith(builders::EndsWith<'a>),
    /// match start and end, `"^expr$"`, check [`builders::StartsAndEndsWith`]
    StartsAndEndsWith(builders::StartsAndEndsWith<'a>),
    /// match characters, `"[chars]"`, check [`builders::Chars`]
    Chars(builders::Chars),
    /// exclude match characters, `"[^chars]"`, check [`builders::NotChars`]
    NotChars(builders::NotChars),
    #[cfg(feature = "re-fancy")]
    /// match atomicly, `"(?>expr)"`, check [`builders::AtomicGroup`]
    /// Available with feature `"re-fancy"`
    AtomicGroup(builders::AtomicGroup<'a>),
}

impl Display for ReadableRe<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let to_write: &dyn Display = match self {
            ReadableRe::Digit => &constants::DIGIT,
            ReadableRe::Word => &constants::WORD,
            ReadableRe::Whitespace => &constants::WHITESPACE,
            ReadableRe::NonDigit => &constants::NON_DIGIT,
            ReadableRe::NonWord => &constants::NON_WORD,
            ReadableRe::NonWhitespace => &constants::NON_WHITESPACE,
            ReadableRe::Boundary => &constants::BOUNDARY,
            ReadableRe::AsciiLetter => &constants::ASCII_LETTER,
            ReadableRe::AsciiNonLetter => &constants::ASCII_NON_LETTER,
            ReadableRe::AsciiUppercase => &constants::ASCII_UPPERCASE,
            ReadableRe::AsciiNonUppercase => &constants::ASCII_NON_UPPERCASE,
            ReadableRe::AsciiLowercase => &constants::ASCII_LOWERCASE,
            ReadableRe::AsciiNonLowercase => &constants::ASCII_NON_LOWERCASE,
            ReadableRe::AsciiAlphanumeric => &constants::ASCII_ALPHANUMERIC,
            ReadableRe::AsciiNonAlphanumeric => &constants::ASCII_NON_ALPHANUMERIC,
            ReadableRe::AsciiNumeric => &constants::ASCII_NUMERIC,
            ReadableRe::AsciiNonNumeric => &constants::ASCII_NON_NUMERIC,
            ReadableRe::Hexadecimal => &constants::HEXADECIMAL,
            ReadableRe::NonHexadecimal => &constants::NON_HEXADECIMAL,
            ReadableRe::Anything => &constants::ANYTHING,
            ReadableRe::Everything => &constants::EVERYTHING,
            ReadableRe::SomethingGreedy => &constants::SOMETHING_GREEDY,
            ReadableRe::Something => &constants::SOMETHING,
            ReadableRe::AnyChar => &constants::ANY_CHAR,
            ReadableRe::Period => &constants::PERIOD,
            ReadableRe::Caret => &constants::CARET,
            ReadableRe::Dollar => &constants::DOLLAR_SIGN,
            ReadableRe::Asterisk => &constants::ASTERISK,
            ReadableRe::PlusSign => &constants::PLUS_SIGN,
            ReadableRe::MinusSign => &constants::MINUS_SIGN,
            ReadableRe::QuestionMark => &constants::QUESTION_MARK,
            ReadableRe::OpenBrace => &constants::OPEN_BRACE,
            ReadableRe::CloseBrace => &constants::CLOSE_BRACE,
            ReadableRe::OpenBracket => &constants::OPEN_BRACKET,
            ReadableRe::CloseBracket => &constants::CLOSE_BRACKET,
            ReadableRe::OpenParenthesis => &constants::OPEN_PARENTHESIS,
            ReadableRe::CloseParenthesis => &constants::CLOSE_PARENTHESIS,
            ReadableRe::BackSlash => &constants::BACKSLASH,
            ReadableRe::Pipe => &constants::PIPE,
            ReadableRe::Newline => &constants::NEWLINE,
            ReadableRe::Tab => &constants::TAB,
            ReadableRe::Quote => &constants::QUOTE,
            ReadableRe::DoubleQuote => &constants::DOUBLE_QUOTE,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back1 => &constants::BACK_1,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back2 => &constants::BACK_2,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back3 => &constants::BACK_3,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back4 => &constants::BACK_4,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back5 => &constants::BACK_5,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back6 => &constants::BACK_6,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back7 => &constants::BACK_7,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back8 => &constants::BACK_8,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back9 => &constants::BACK_9,
            ReadableRe::Raw(raw) => raw,
            ReadableRe::String(s) => s,
            ReadableRe::Concat(concat) => concat,
            #[cfg(feature = "re-fancy")]
            ReadableRe::BackReference(back_reference) => back_reference,
            ReadableRe::Escape(scape) => scape,
            ReadableRe::Group(group) => group,
            #[cfg(feature = "re-fancy")]
            ReadableRe::PositiveLookAhead(positive_look_ahead) => positive_look_ahead,
            #[cfg(feature = "re-fancy")]
            ReadableRe::NegativeLookAhead(negative_look_ahead) => negative_look_ahead,
            #[cfg(feature = "re-fancy")]
            ReadableRe::PositiveLookBehind(positive_look_behind) => positive_look_behind,
            #[cfg(feature = "re-fancy")]
            ReadableRe::NegativeLookBehind(negative_look_behind) => negative_look_behind,
            ReadableRe::NamedGroup(named_group) => named_group,
            ReadableRe::NonCaptureGroup(non_capture_group) => non_capture_group,
            ReadableRe::Optional(optional) => optional,
            ReadableRe::Either(either) => either,
            ReadableRe::Exactly(exactly) => exactly,
            ReadableRe::Ranged(between) => between,
            ReadableRe::ZeroOrMore(zero_or_more) => zero_or_more,
            ReadableRe::ZeroOrMoreLazy(zero_or_more_lazy) => zero_or_more_lazy,
            ReadableRe::OneOrMore(one_or_more) => one_or_more,
            ReadableRe::OneOrMoreLazy(one_or_more_lazy) => one_or_more_lazy,
            ReadableRe::StartsWith(starts_with) => starts_with,
            ReadableRe::EndsWith(ends_with) => ends_with,
            ReadableRe::StartsAndEndsWith(starts_and_ends_with) => starts_and_ends_with,
            ReadableRe::Chars(chars) => chars,
            ReadableRe::NotChars(not_chars) => not_chars,
            #[cfg(feature = "re-fancy")]
            ReadableRe::AtomicGroup(atomic_group) => atomic_group,
        };
        write!(f, "{}", to_write)
    }
}
