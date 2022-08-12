use crate::ReadableRe;
use std::fmt::{Display, Formatter};

/// Just a [`ReadableRe`] concatenation wrapper
///
/// ## Example
///
/// ```
/// use readable_regex::builders::Concat;
/// use readable_regex::ReadableRe::Raw;
/// assert_eq!(&Concat::from_iter([Raw("foo"), Raw("bar")]).to_string(), "foobar");
/// ```
pub struct Concat<'a>(Vec<ReadableRe<'a>>);

impl<'a> FromIterator<ReadableRe<'a>> for Concat<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> Display for Concat<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for re in &self.0 {
            write!(f, "{}", re)?;
        }
        Ok(())
    }
}

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
pub struct Group<'a>(Concat<'a>);

impl<'a> Group<'a> {
    pub fn new(v: Vec<ReadableRe<'a>>) -> Self {
        Self(Concat(v))
    }
}

impl<'a> FromIterator<ReadableRe<'a>> for Group<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(Concat::from_iter(iter))
    }
}

impl<'a> Display for Group<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
/// Regex syntax for a positive lookahead assertion of the regex strings
/// A lookahead matches text but does not consume it in the original parsed text.
///
/// ## Example
///
/// In the following example, 'kitty' is matched but only if 'cat' follows
/// 'kitty'. Note that the match only includes 'kitty' and not 'kittycat'.
///
/// ```
/// use readable_regex::builders::{Concat, PositiveLookAhead};
/// use readable_regex::{ReadableRe, ReadableRegex};
/// use std::fmt::Display;
/// let query = Concat::from_iter([
///     ReadableRe::Raw("kitty"),
///     ReadableRe::PositiveLookAhead(PositiveLookAhead::from_iter([ReadableRe::Raw("cat")])),
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "kitty(?=cat)"
/// );
///
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kitty").unwrap());
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// ```
pub struct PositiveLookAhead<'a>(Concat<'a>);

#[cfg(feature = "re-fancy")]
impl<'a> PositiveLookAhead<'a> {
    pub fn new(v: Vec<ReadableRe<'a>>) -> Self {
        Self(Concat(v))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> FromIterator<ReadableRe<'a>> for PositiveLookAhead<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(Concat::from_iter(iter))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for PositiveLookAhead<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?={})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
/// Regex syntax for a negative lookahead assertion of the regex strings
/// A lookahead matches text but does not consume it in the original parsed text.
///
/// ## Example
///
/// In the following example, 'kitty' is matched but only if the 'cat'
/// does not follow 'kitty'. Note that the match only includes 'kitty' and not 'kittycat'
///
/// ```
/// use readable_regex::builders::{Concat, NegativeLookAhead};
/// use readable_regex::{ReadableRe, ReadableRegex};
/// use std::fmt::Display;
/// let query = Concat::from_iter([
///     ReadableRe::Raw("kitty"),
///     ReadableRe::NegativeLookAhead(NegativeLookAhead::from_iter([ReadableRe::Raw("cat")])),
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "kitty(?!cat)"
/// );
///
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kitty").unwrap());
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// ```
pub struct NegativeLookAhead<'a>(Concat<'a>);

#[cfg(feature = "re-fancy")]
impl<'a> NegativeLookAhead<'a> {
    pub fn new(v: Vec<ReadableRe<'a>>) -> Self {
        Self(Concat(v))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> FromIterator<ReadableRe<'a>> for NegativeLookAhead<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(Concat::from_iter(iter))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for NegativeLookAhead<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?!{})", self.0)
    }
}
