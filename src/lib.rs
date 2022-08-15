pub mod builders;
mod constants;

use std::fmt::{Display, Formatter};

use regex::Error;

#[cfg(feature = "re")]
use regex::Regex;

#[cfg(all(feature = "re-fancy", not(feature = "re")))]
use fancy_regex::Regex;

#[cfg(any(feature = "re", feature = "re-fancy"))]
pub trait ReadableRegex: Display {
    fn compile(&self) -> Result<Regex, Error> {
        Regex::new(&format!("{self}"))
    }
}

pub enum ReadableRe<'a> {
    Digit,
    Word,
    Whitespace,
    NonDigit,
    NonWord,
    NonWhitespace,
    Boundary,

    AsciiLetter,
    AsciiNonLetter,
    AsciiUppercase,
    AsciiNonUppercase,
    AsciiLowercase,
    AsciiNonLowercase,
    AsciiAlphanumeric,
    AsciiNonAlphanumeric,
    AsciiNumeric,
    AsciiNonNumeric,

    Hexadecimal,
    NonHexadecimal,

    Anything,
    Everything,
    SomethingGreedy,
    Something,
    AnyChar,

    Period,
    Caret,
    Dollar,
    Asterisk,
    PlusSign,
    MinusSign,
    QuestionMark,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    BackSlash,
    Pipe,

    Newline,
    Tab,
    Quote,
    DoubleQuote,

    Back1,
    Back2,
    Back3,
    Back4,
    Back5,
    Back6,
    Back7,
    Back8,
    Back9,

    Raw(&'a str),
    String(String),
    Concat(builders::Concat<'a>),

    #[cfg(feature = "re-fancy")]
    BackReference(builders::BackReference),

    Scape(builders::Scape<'a>),
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
            ReadableRe::Back1 => &constants::BACK_1 as &dyn Display,
            ReadableRe::Back2 => &constants::BACK_2 as &dyn Display,
            ReadableRe::Back3 => &constants::BACK_3 as &dyn Display,
            ReadableRe::Back4 => &constants::BACK_4 as &dyn Display,
            ReadableRe::Back5 => &constants::BACK_5 as &dyn Display,
            ReadableRe::Back6 => &constants::BACK_6 as &dyn Display,
            ReadableRe::Back7 => &constants::BACK_7 as &dyn Display,
            ReadableRe::Back8 => &constants::BACK_8 as &dyn Display,
            ReadableRe::Back9 => &constants::BACK_9 as &dyn Display,
            ReadableRe::Raw(raw) => raw as &dyn Display,
            ReadableRe::String(s) => s as &dyn Display,
            ReadableRe::Concat(concat) => concat as &dyn Display,
            #[cfg(feature = "re-fancy")]
            ReadableRe::BackReference(back_reference) => back_reference as &dyn Display,
            ReadableRe::Scape(scape) => scape as &dyn Display,
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
        };
        write!(f, "{}", to_write)
    }
}
