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
    /// back reference `\1`
    Back1,
    #[cfg(feature = "re-fancy")]
    /// back reference `\2`
    Back2,
    #[cfg(feature = "re-fancy")]
    /// back reference `\3`
    Back3,
    #[cfg(feature = "re-fancy")]
    /// back reference `\4`
    Back4,
    #[cfg(feature = "re-fancy")]
    /// back reference `\5`
    Back5,
    #[cfg(feature = "re-fancy")]
    /// back reference `\6`
    Back6,
    #[cfg(feature = "re-fancy")]
    /// back reference `\7`
    Back7,
    #[cfg(feature = "re-fancy")]
    /// back reference `\8
    Back8,
    #[cfg(feature = "re-fancy")]
    /// back reference `\9`
    Back9,

    Raw(&'a str),
    String(String),
    Concat(builders::Concat<'a>),

    #[cfg(feature = "re-fancy")]
    BackReference(builders::BackReference),

    Escape(builders::Escape<'a>),
    Group(builders::Group<'a>),

    #[cfg(feature = "re-fancy")]
    PositiveLookAhead(builders::PositiveLookAhead<'a>),
    #[cfg(feature = "re-fancy")]
    NegativeLookAhead(builders::NegativeLookAhead<'a>),
    #[cfg(feature = "re-fancy")]
    PositiveLookBehind(builders::PositiveLookBehind<'a>),
    #[cfg(feature = "re-fancy")]
    NegativeLookBehind(builders::NegativeLookBehind<'a>),
    NamedGroup(builders::NamedGroup<'a>),
    NonCaptureGroup(builders::NonCaptureGroup<'a>),
    Optional(builders::Optional<'a>),
    Either(builders::Either<'a>),
    Between(builders::Ranged<'a>),
    ZeroOrMore(builders::ZeroOrMore<'a>),
    ZeroOrMoreLazy(builders::ZeroOrMoreLazy<'a>),
    OneOrMore(builders::OneOrMore<'a>),
    OneOrMoreLazy(builders::OneOrMoreLazy<'a>),
    StartsWith(builders::StartsWith<'a>),
    EndsWith(builders::EndsWith<'a>),
    StartsAndEndsWith(builders::StartsAndEndsWith<'a>),
    Chars(builders::Chars),
    NotChars(builders::NotChars),
    #[cfg(feature = "re-fancy")]
    AtomicGroup(builders::AtomicGroup<'a>),
}

impl Display for ReadableRe<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            ReadableRe::Digit => &constants::DIGIT as &dyn Display,
            ReadableRe::Word => &constants::WORD as &dyn Display,
            ReadableRe::Whitespace => &constants::WHITESPACE as &dyn Display,
            ReadableRe::NonDigit => &constants::NON_DIGIT as &dyn Display,
            ReadableRe::NonWord => &constants::NON_WORD as &dyn Display,
            ReadableRe::NonWhitespace => &constants::NON_WHITESPACE as &dyn Display,
            ReadableRe::Boundary => &constants::BOUNDARY as &dyn Display,
            ReadableRe::AsciiLetter => &constants::ASCII_LETTER as &dyn Display,
            ReadableRe::AsciiNonLetter => &constants::ASCII_NON_LETTER as &dyn Display,
            ReadableRe::AsciiUppercase => &constants::ASCII_UPPERCASE as &dyn Display,
            ReadableRe::AsciiNonUppercase => &constants::ASCII_NON_UPPERCASE as &dyn Display,
            ReadableRe::AsciiLowercase => &constants::ASCII_LOWERCASE as &dyn Display,
            ReadableRe::AsciiNonLowercase => &constants::ASCII_NON_LOWERCASE as &dyn Display,
            ReadableRe::AsciiAlphanumeric => &constants::ASCII_ALPHANUMERIC as &dyn Display,
            ReadableRe::AsciiNonAlphanumeric => &constants::ASCII_NON_ALPHANUMERIC as &dyn Display,
            ReadableRe::AsciiNumeric => &constants::ASCII_NUMERIC as &dyn Display,
            ReadableRe::AsciiNonNumeric => &constants::ASCII_NON_NUMERIC as &dyn Display,
            ReadableRe::Hexadecimal => &constants::HEXADECIMAL as &dyn Display,
            ReadableRe::NonHexadecimal => &constants::NON_HEXADECIMAL as &dyn Display,
            ReadableRe::Anything => &constants::ANYTHING as &dyn Display,
            ReadableRe::Everything => &constants::EVERYTHING as &dyn Display,
            ReadableRe::SomethingGreedy => &constants::SOMETHING_GREEDY as &dyn Display,
            ReadableRe::Something => &constants::SOMETHING as &dyn Display,
            ReadableRe::AnyChar => &constants::ANY_CHAR as &dyn Display,
            ReadableRe::Period => &constants::PERIOD as &dyn Display,
            ReadableRe::Caret => &constants::CARET as &dyn Display,
            ReadableRe::Dollar => &constants::DOLLAR_SIGN as &dyn Display,
            ReadableRe::Asterisk => &constants::ASTERISK as &dyn Display,
            ReadableRe::PlusSign => &constants::PLUS_SIGN as &dyn Display,
            ReadableRe::MinusSign => &constants::MINUS_SIGN as &dyn Display,
            ReadableRe::QuestionMark => &constants::QUESTION_MARK as &dyn Display,
            ReadableRe::OpenBrace => &constants::OPEN_BRACE as &dyn Display,
            ReadableRe::CloseBrace => &constants::CLOSE_BRACE as &dyn Display,
            ReadableRe::OpenBracket => &constants::OPEN_BRACKET as &dyn Display,
            ReadableRe::CloseBracket => &constants::CLOSE_BRACKET as &dyn Display,
            ReadableRe::OpenParenthesis => &constants::OPEN_PARENTHESIS as &dyn Display,
            ReadableRe::CloseParenthesis => &constants::CLOSE_PARENTHESIS as &dyn Display,
            ReadableRe::BackSlash => &constants::BACKSLASH as &dyn Display,
            ReadableRe::Pipe => &constants::PIPE as &dyn Display,
            ReadableRe::Newline => &constants::NEWLINE as &dyn Display,
            ReadableRe::Tab => &constants::TAB as &dyn Display,
            ReadableRe::Quote => &constants::QUOTE as &dyn Display,
            ReadableRe::DoubleQuote => &constants::DOUBLE_QUOTE as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back1 => &constants::BACK_1 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back2 => &constants::BACK_2 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back3 => &constants::BACK_3 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back4 => &constants::BACK_4 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back5 => &constants::BACK_5 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back6 => &constants::BACK_6 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back7 => &constants::BACK_7 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back8 => &constants::BACK_8 as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::Back9 => &constants::BACK_9 as &dyn Display,
            ReadableRe::Raw(raw) => raw as &dyn Display,
            ReadableRe::String(s) => s as &dyn Display,
            ReadableRe::Concat(concat) => concat as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::BackReference(back_reference) => back_reference as &dyn Display,
            ReadableRe::Escape(scape) => scape as &dyn Display,
            ReadableRe::Group(group) => group as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::PositiveLookAhead(positive_look_ahead) => {
                positive_look_ahead as &dyn Display
            }
            #[cfg(feature = "re-fancy")]
            ReadableRe::NegativeLookAhead(negative_look_ahead) => {
                negative_look_ahead as &dyn Display
            }
            #[cfg(feature = "re-fancy")]
            ReadableRe::PositiveLookBehind(positive_look_behind) => {
                positive_look_behind as &dyn Display
            }
            #[cfg(feature = "re-fancy")]
            ReadableRe::NegativeLookBehind(negative_look_behind) => {
                negative_look_behind as &dyn Display
            }
            ReadableRe::NamedGroup(named_group) => named_group as &dyn Display,
            ReadableRe::NonCaptureGroup(non_capture_group) => non_capture_group as &dyn Display,
            ReadableRe::Optional(optional) => optional as &dyn Display,
            ReadableRe::Either(either) => either as &dyn Display,
            ReadableRe::Between(between) => between as &dyn Display,
            ReadableRe::ZeroOrMore(zero_or_more) => zero_or_more as &dyn Display,
            ReadableRe::ZeroOrMoreLazy(zero_or_more_lazy) => zero_or_more_lazy as &dyn Display,
            ReadableRe::OneOrMore(one_or_more) => one_or_more as &dyn Display,
            ReadableRe::OneOrMoreLazy(one_or_more_lazy) => one_or_more_lazy as &dyn Display,
            ReadableRe::StartsWith(starts_with) => starts_with as &dyn Display,
            ReadableRe::EndsWith(ends_with) => ends_with as &dyn Display,
            ReadableRe::StartsAndEndsWith(starts_and_ends_with) => {
                starts_and_ends_with as &dyn Display
            }
            ReadableRe::Chars(chars) => chars as &dyn Display,
            ReadableRe::NotChars(not_chars) => not_chars as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::AtomicGroup(atomic_group) => atomic_group as &dyn Display,
        };
        write!(f, "{}", to_write)
    }
}
