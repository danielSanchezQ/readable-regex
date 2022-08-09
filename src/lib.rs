pub const DIGIT: &str = r"\d";
pub const WORD: &str = r"\w";
pub const WHITESPACE: &str = r"\s";
pub const NON_DIGIT: &str = r"\D";
pub const NON_WORD: &str = r"\W";
pub const NON_WHITESPACE: &str = r"\S";
pub const BOUNDARY: &str = r"\b";

pub const ASCII_LETTER: &str = "[A-Za-z]";
pub const ASCII_NON_LETTER: &str = "[^A-Za-z]";
pub const ASCII_UPPERCASE: &str = "[A-Z]";
pub const ASCII_NON_UPPERCASE: &str = "[^A-Z]";
pub const ASCII_LOWERCASE: &str = "[a-z]";
pub const ASCII_NON_LOWERCASE: &str = "[^a-z]";
pub const ASCII_ALPHANUMERIC: &str = "[A-Za-z0-9]";
pub const ASCII_NON_ALPHANUMERIC: &str = "[^A-Za-z0-9]";
pub const ASCII_NUMERIC: &str = "[0-9]";
pub const ASCII_NON_NUMERIC: &str = "[^0-9]";

pub const HEXADECIMAL: &str = "[0-9A-Fa-f]";
pub const NON_HEXADECIMAL: &str = "[^0-9A-Fa-f]";

pub const ANYTHING: &str = ".*?";
pub const EVERYTHING: &str = ".*";
pub const SOMETHING_GREEDY: &str = ".+";
pub const SOMETHING: &str = ".+?";
pub const ANY_CHAR: &str = ".";

// labels for escaped regex metacharacters: . ^ $ * + ? { } [ ] \ | ( )
// This full list copied from https://docs.python.org/3/howto/regex.html
pub const PERIOD: &str = r"\.";
pub const CARET: &str = r"\^";
pub const DOLLAR_SIGN: &str = r"\$";
pub const ASTERISK: &str = r"\*";
pub const PLUS_SIGN: &str = r"\+";
pub const MINUS_SIGN: &str = r"\-";
pub const QUESTION_MARK: &str = r"\?";
pub const OPEN_BRACE: &str = r"\{";
pub const CLOSE_BRACE: &str = r"\}";
pub const OPEN_BRACKET: &str = r"\[";
pub const CLOSE_BRACKET: &str = r"\]";
pub const BACKSLASH: &str = r"\\";
pub const PIPE: &str = r"\|";
pub const OPEN_PARENTHESIS: &str = r"\(";
pub const CLOSE_PARENTHESIS: &str = r"\)";

pub const NEWLINE: &str = r"\n"; // TODO - double check this: do I want r"\n" or "\n" here?
pub const TAB: &str = r"\t";
pub const QUOTE: &str = r"\'";
pub const DOUBLE_QUOTE: &str = r#"\""#;

pub const BACK_1: &str = r"\1";
pub const BACK_2: &str = r"\2";
pub const BACK_3: &str = r"\3";
pub const BACK_4: &str = r"\4";
pub const BACK_5: &str = r"\5";
pub const BACK_6: &str = r"\6";
pub const BACK_7: &str = r"\7";
pub const BACK_8: &str = r"\8";
pub const BACK_9: &str = r"\9";
