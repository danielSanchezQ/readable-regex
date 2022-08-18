use std::fmt::{Display, Formatter};

use regex::Error;

use crate::{constants, solvers};

#[cfg(feature = "re")]
use regex::Regex;

#[cfg(all(feature = "re-fancy", not(feature = "re")))]
use fancy_regex::Regex;

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
    /// concatenation of regex exp, `"exp1exp2exp3"`, check [`solvers::Concat`]
    Concat(solvers::Concat<'a>),

    #[cfg(feature = "re-fancy")]
    /// `x` backreference, `"\x"`, check [`solvers::BackReference`]
    ///
    /// Available with feature `"re-fancy"`
    BackReference(solvers::BackReference),

    /// Special characters escape wrapper, check [`solvers::Escape`]
    Escape(solvers::Escape<'a>),
    /// Regex group, `"(expr)"`, check [`solvers::Group`]
    Group(solvers::Group<'a>),

    #[cfg(feature = "re-fancy")]
    /// look ahead, `"(?=expr)"`, check [`solvers::PositiveLookAhead`]
    ///
    /// Available with feature `"re-fancy"`
    PositiveLookAhead(solvers::PositiveLookAhead<'a>),
    #[cfg(feature = "re-fancy")]
    /// negative look ahead, `"(?!expr)"`, check [`solvers::NegativeLookAhead`]
    ///
    /// Available with feature `"re-fancy"`
    NegativeLookAhead(solvers::NegativeLookAhead<'a>),
    #[cfg(feature = "re-fancy")]
    /// look behind, `"(?<=expr)"`, check [`solvers::PositiveLookBehind`]
    ///
    /// Available with feature `"re-fancy"`
    PositiveLookBehind(solvers::PositiveLookBehind<'a>),
    #[cfg(feature = "re-fancy")]
    /// negative look behind, `"(?!expr)"`, check [`solvers::NegativeLookBehind`]
    ///
    /// Available with feature `"re-fancy"`
    NegativeLookBehind(solvers::NegativeLookBehind<'a>),
    /// named group match, `"(?P<name>expr)"`, check [`solvers::NamedGroup`]
    NamedGroup(solvers::NamedGroup<'a>),
    /// non-capture group, `"(?:expr)"`
    NonCaptureGroup(solvers::NonCaptureGroup<'a>),
    /// optional match, `"expr?"`, check [`solvers::Optional`]
    Optional(solvers::Optional<'a>),
    /// either match, `"expr1|expr2|..."`, check [`solvers::Either`]
    Either(solvers::Either<'a>),
    /// exact number (n) of occurrences match, `"expr{n}"`, check [`solvers::Exactly`]
    Exactly(solvers::Exactly<'a>),
    /// variance number (min, max) of occurrences match, `"expr{min, max}"`, check [`solvers::Ranged`]
    Ranged(solvers::Ranged<'a>),
    /// zero or more occurrences match, `"expr*"`, check [`solvers::ZeroOrMore`]
    ZeroOrMore(solvers::ZeroOrMore<'a>),
    /// zero or more occurrences, lazy match, `"expr*?"`, , check [`solvers::ZeroOrMoreLazy`]
    ZeroOrMoreLazy(solvers::ZeroOrMoreLazy<'a>),
    /// one or more occurrences match, `"expr+"`, check [`solvers::OneOrMore`]
    OneOrMore(solvers::OneOrMore<'a>),
    /// one or more occurrences, lazy match, `"expr+?"`, , check [`solvers::OneOrMoreLazy`]
    OneOrMoreLazy(solvers::OneOrMoreLazy<'a>),
    /// match start, `"^expr"`, check [`solvers::StartsWith`]
    StartsWith(solvers::StartsWith<'a>),
    /// match end, `"expr$"`, check [`solvers::EndsWith`]
    EndsWith(solvers::EndsWith<'a>),
    /// match start and end, `"^expr$"`, check [`solvers::StartsAndEndsWith`]
    StartsAndEndsWith(solvers::StartsAndEndsWith<'a>),
    /// match characters, `"[chars]"`, check [`solvers::Chars`]
    Chars(solvers::Chars),
    /// exclude match characters, `"[^chars]"`, check [`solvers::NotChars`]
    NotChars(solvers::NotChars),
    #[cfg(feature = "re-fancy")]
    /// match atomicly, `"(?>expr)"`, check [`solvers::AtomicGroup`]
    /// Available with feature `"re-fancy"`
    AtomicGroup(solvers::AtomicGroup<'a>),
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

impl<'a> ReadableRe<'a> {
    pub fn compile(&self) -> Result<Regex, Error> {
        Regex::new(&format!("{self}"))
    }
}
