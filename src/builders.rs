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

impl<'a> Concat<'a> {
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
    }
}

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

#[cfg(feature = "re-fancy")]
/// Returns a string in the regex syntax for a back reference, such as \1, \2, etc.
/// ## Example
/// ```
/// use readable_regex::builders::BackReference;
/// let back_3 = BackReference(3);
/// assert_eq!(back_3.to_string(), "\\3");
/// ```
pub struct BackReference(pub usize);

#[cfg(feature = "re-fancy")]
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
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
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
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
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
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
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

#[cfg(feature = "re-fancy")]
/// Regex syntax for a positive lookbehind assertion of the input regexes.
/// A lookbehind matches text but does not consume it in the original parsed text
/// ## Example
///
/// In the following example, 'cat' is matched but only if 'kitty' is before 'cat'.
/// Note that the match only includes 'cat' and not 'kittycat'.
///
/// ```
/// use readable_regex::builders::{Concat, PositiveLookBehind};
/// use readable_regex::{ReadableRe, ReadableRegex};
/// use std::fmt::Display;
/// let query = Concat::from_iter([
///     ReadableRe::PositiveLookBehind(PositiveLookBehind::from_iter([ReadableRe::Raw("kitty")])),
///     ReadableRe::Raw("cat")
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "(?<=kitty)cat"
/// );
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("cat").unwrap());
/// ```
pub struct PositiveLookBehind<'a>(Concat<'a>);

#[cfg(feature = "re-fancy")]
impl<'a> PositiveLookBehind<'a> {
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> FromIterator<ReadableRe<'a>> for PositiveLookBehind<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(Concat::from_iter(iter))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for PositiveLookBehind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?<={})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
/// Negative lookbehind assertion of the regex input.
/// A lookbehind matches text but does not consume it in the original parsed text
///
/// ## Example
///
/// In the following example, 'cat' is matched but only if 'kitty' is not
/// before 'cat'. Note that the match only includes 'cat' and not 'kittycat'.
///
/// ```
/// use readable_regex::builders::{Concat, NegativeLookBehind};
/// use readable_regex::{ReadableRe, ReadableRegex};
/// use std::fmt::Display;
/// let query = Concat::from_iter([
///     ReadableRe::NegativeLookBehind(NegativeLookBehind::from_iter([ReadableRe::Raw("kitty")])),
///     ReadableRe::Raw("cat")
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "(?<!kitty)cat"
/// );
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("black cat").unwrap());
/// ```
pub struct NegativeLookBehind<'a>(Concat<'a>);

#[cfg(feature = "re-fancy")]
impl<'a> NegativeLookBehind<'a> {
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> FromIterator<ReadableRe<'a>> for NegativeLookBehind<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(Concat::from_iter(iter))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for NegativeLookBehind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?<!{})", self.0)
    }
}

/// Regex syntax for a named group of the regex strings in tuple_of_regex_strs.
/// Named groups can be referred to by their name rather than their group number.
///
/// ## Examples
///
/// ```
/// use readable_regex::builders::NamedGroup;
/// use readable_regex::ReadableRe::Raw;
/// use std::fmt::Display;
/// assert_eq!(
///     &NamedGroup::new("group_name", [Raw(r"pattern_to_look_for")]).to_string(),
///     "(?P<group_name>pattern_to_look_for)"
/// );
/// assert_eq!(
///     &NamedGroup::new("pobox", [Raw(r"PO BOX \d{3:5}")]).to_string(),
///     "(?P<pobox>PO BOX \\d{3:5})"
/// );
/// ```
pub struct NamedGroup<'a> {
    name: &'a str,
    regexes: Concat<'a>,
}

impl<'a> NamedGroup<'a> {
    pub fn new(name: &'a str, iter: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self {
            name,
            regexes: Concat::from_iter(iter),
        }
    }
}

impl<'a> Display for NamedGroup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?P<{}>{})", self.name, self.regexes)
    }
}

/// Regex syntax for a non-capturing group of the regex input
///
/// Non-capturing groups are not included in the groups field of a Pattern object.
/// They are useful for when you want to group parts of a regex string together without
/// affecting the numbered groups.
///
/// ## Example
///
/// ```
/// use readable_regex::builders::{Concat, NonCaptureGroup};
/// use readable_regex::{ReadableRe, ReadableRegex};
/// use std::fmt::Display;
/// let query = ReadableRe::NonCaptureGroup(
///     NonCaptureGroup::from_iter([ReadableRe::Raw("pattern_to_look_for")])
/// );
/// assert_eq!(
///     query.to_string(),
///     "(?:pattern_to_look_for)"
/// );
/// ```
pub struct NonCaptureGroup<'a>(Concat<'a>);

impl<'a> NonCaptureGroup<'a> {
    pub fn new(v: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(v)
    }
}

impl<'a> FromIterator<ReadableRe<'a>> for NonCaptureGroup<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        Self(Concat::from_iter(iter))
    }
}

impl<'a> Display for NonCaptureGroup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?:{})", self.0)
    }
}
