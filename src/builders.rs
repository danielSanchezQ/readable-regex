use crate::ReadableRe;
use std::fmt::{Display, Formatter};

/// Returns a string in the regex syntax for a back reference, such as \1, \2, etc.
/// ## Example
/// ```
/// use readable_regex::builders::BackReference;
/// let back_3 = BackReference(3);
/// assert_eq!(back_3.to_string(), "\\3");
/// ```
pub struct BackReference(pub usize);

impl Display for BackReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r"\{}", self.0)
    }
}

/// A wrapper for re.escape(). Escape special characters in the input str
/// ## Example
/// ```
/// use readable_regex::builders::Scape;
/// use readable_regex::ReadableRe;
/// let scaped = Scape::from_str("!#$%&");
/// assert_eq!(scaped.to_string(), "!\\#\\$%\\&");
/// ```
pub struct Scape<'a>(Box<ReadableRe<'a>>);

impl Display for Scape<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", regex::escape(&self.0.to_string()))
    }
}

impl<'a> Scape<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self(Box::new(ReadableRe::Raw(s)))
    }

    pub fn new(readable_regex: ReadableRe<'a>) -> Self {
        Self(Box::new(readable_regex))
    }
}

/// Regex syntax for a regex group surrounded by parentheses of the regex input str
/// ### Example
/// ```
/// use readable_regex::builders::Group;
/// use readable_regex::ReadableRe::{Raw, String};
/// assert_eq!(Group::new(vec![Raw("cat")]).to_string(), "(cat)");
/// assert_eq!(Group::new(vec![Raw("cat"), Raw("dog"), Raw("moose")]).to_string(), "(catdogmoose)");
/// assert_eq!(
///     Group::new(
///         vec![
///             Raw("cat"),
///             String(Group::new(vec![Raw("dog")]).to_string()),
///             String(Group::new(vec![Raw("moose")]).to_string())
///         ]
///     ).to_string(),
///     "(cat(dog)(moose))"
/// );
/// ```
pub struct Group<'a>(Box<Vec<ReadableRe<'a>>>);

impl<'a> Group<'a> {
    pub fn new(v: Vec<ReadableRe<'a>>) -> Self {
        Self(Box::new(v.into()))
    }
}

impl Display for Group<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for s in self.0.as_ref() {
            write!(f, "{}", s)?;
        }
        write!(f, ")")
    }
}
