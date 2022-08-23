use crate::ReadableRe;
use std::fmt::{Display, Formatter};
use std::ops::{Bound, RangeBounds};

macro_rules! impl_builder_from_iter {
    ($struct_name:ident) => {
        impl<'a> FromIterator<ReadableRe<'a>> for $struct_name<'a> {
            fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
                Self(Box::new(ReadableRe::Concat(Concat::new(
                    iter.into_iter().collect::<Vec<_>>(),
                ))))
            }
        }
    };
}

/// Just a [`ReadableRe`] concatenation wrapper
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::Concat;
/// use readable_regex::ReadableRe::Raw;
/// assert_eq!(&Concat::from_iter([Raw("foo"), Raw("bar")]).to_string(), "foobar");
/// ```
#[derive(Clone)]
pub struct Concat<'a>(pub(crate) Vec<ReadableRe<'a>>);

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
/// use readable_regex::solvers::BackReference;
/// let back_3 = BackReference(3);
/// assert_eq!(back_3.to_string(), "\\3");
/// ```
#[derive(Clone)]
pub struct BackReference(pub usize);

#[cfg(feature = "re-fancy")]
impl Display for BackReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r"\{}", self.0)
    }
}

/// A wrapper for [`regex::escape`]. Escape special characters in the input str
/// ## Example
/// ```
/// use readable_regex::solvers::Escape;
/// use readable_regex::ReadableRe;
/// let scaped = Escape::new_str("!#$%&");
/// assert_eq!(scaped.to_string(), "!\\#\\$%\\&");
/// ```
#[derive(Clone)]
pub struct Escape<'a>(Box<ReadableRe<'a>>);

impl Display for Escape<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", regex::escape(&self.0.to_string()))
    }
}

impl<'a> Escape<'a> {
    pub fn new_str(s: &'a str) -> Self {
        Self(Box::new(ReadableRe::Raw(s)))
    }

    pub fn new(readable_regex: ReadableRe<'a>) -> Self {
        Self(Box::new(readable_regex))
    }
}

impl_builder_from_iter!(Escape);

/// Regex syntax for a regex group surrounded by parentheses of the regex input str
/// ### Example
/// ```
/// use readable_regex::solvers::{Concat, Group};
/// use readable_regex::ReadableRe::{self, Raw};
/// assert_eq!(Group::new(Raw("cat")).to_string(), "(cat)");
/// assert_eq!(Group::new(ReadableRe::Concat(Concat::new([Raw("cat"), Raw("dog"), Raw("moose")]))).to_string(), "(catdogmoose)");
/// assert_eq!(
///     Group::new(ReadableRe::Concat(Concat::new([
///         Raw("cat"),
///         ReadableRe::Group(Group::new(Raw("dog"))),
///         ReadableRe::Group(Group::new(Raw("moose"))),
///     ]))).to_string(),
///     "(cat(dog)(moose))",
/// );
/// ```
#[derive(Clone)]
pub struct Group<'a>(Box<ReadableRe<'a>>);

impl<'a> Group<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for Group<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl_builder_from_iter!(Group);

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
/// use readable_regex::solvers::{Concat, PositiveLookAhead};
/// use readable_regex::ReadableRe;
/// use std::fmt::Display;
/// let query = Concat::new([
///     ReadableRe::Raw("kitty"),
///     ReadableRe::PositiveLookAhead(PositiveLookAhead::new(ReadableRe::Raw("cat"))),
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "kitty(?=cat)"
/// );
///
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kitty").unwrap());
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// ```
#[derive(Clone)]
pub struct PositiveLookAhead<'a>(Box<ReadableRe<'a>>);

#[cfg(feature = "re-fancy")]
impl<'a> PositiveLookAhead<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for PositiveLookAhead<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?={})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
impl_builder_from_iter!(PositiveLookAhead);

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
/// use readable_regex::solvers::{Concat, NegativeLookAhead};
/// use readable_regex::ReadableRe;
/// use std::fmt::Display;
/// let query = Concat::new([
///     ReadableRe::Raw("kitty"),
///     ReadableRe::NegativeLookAhead(NegativeLookAhead::new(ReadableRe::Raw("cat"))),
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "kitty(?!cat)"
/// );
///
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kitty").unwrap());
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// ```
#[derive(Clone)]
pub struct NegativeLookAhead<'a>(Box<ReadableRe<'a>>);

#[cfg(feature = "re-fancy")]
impl<'a> NegativeLookAhead<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for NegativeLookAhead<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?!{})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
impl_builder_from_iter!(NegativeLookAhead);

#[cfg(feature = "re-fancy")]
/// Regex syntax for a positive lookbehind assertion of the input regexes.
/// A lookbehind matches text but does not consume it in the original parsed text
/// ## Example
///
/// In the following example, 'cat' is matched but only if 'kitty' is before 'cat'.
/// Note that the match only includes 'cat' and not 'kittycat'.
///
/// ```
/// use readable_regex::solvers::{Concat, PositiveLookBehind};
/// use readable_regex::ReadableRe;
/// use std::fmt::Display;
/// let query = Concat::new([
///     ReadableRe::PositiveLookBehind(PositiveLookBehind::new(ReadableRe::Raw("kitty"))),
///     ReadableRe::Raw("cat")
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "(?<=kitty)cat"
/// );
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("cat").unwrap());
/// ```
#[derive(Clone)]
pub struct PositiveLookBehind<'a>(Box<ReadableRe<'a>>);

#[cfg(feature = "re-fancy")]
impl<'a> PositiveLookBehind<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for PositiveLookBehind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?<={})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
impl_builder_from_iter!(PositiveLookBehind);

#[cfg(feature = "re-fancy")]
/// Negative lookbehind assertion of the input regex.
/// A lookbehind matches text but does not consume it in the original parsed text
///
/// ## Example
///
/// In the following example, 'cat' is matched but only if 'kitty' is not
/// before 'cat'. Note that the match only includes 'cat' and not 'kittycat'.
///
/// ```
/// use readable_regex::solvers::{Concat, NegativeLookBehind};
/// use readable_regex::ReadableRe;
/// use std::fmt::Display;
/// let query = Concat::from_iter([
///     ReadableRe::NegativeLookBehind(NegativeLookBehind::new(ReadableRe::Raw("kitty"))),
///     ReadableRe::Raw("cat")
///  ]);
/// assert_eq!(
///     query.to_string(),
///     "(?<!kitty)cat"
/// );
/// assert!(!fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("kittycat").unwrap());
/// assert!(fancy_regex::Regex::new(&query.to_string()).unwrap().is_match("black cat").unwrap());
/// ```
#[derive(Clone)]
pub struct NegativeLookBehind<'a>(Box<ReadableRe<'a>>);

#[cfg(feature = "re-fancy")]
impl<'a> NegativeLookBehind<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for NegativeLookBehind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?<!{})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
impl_builder_from_iter!(NegativeLookBehind);

/// Regex syntax for a named group of the input regex.
/// Named groups can be referred to by their name rather than their group number.
///
/// ## Examples
///
/// ```
/// use readable_regex::solvers::NamedGroup;
/// use readable_regex::ReadableRe::Raw;
/// use std::fmt::Display;
/// assert_eq!(
///     &NamedGroup::new("group_name", Raw(r"pattern_to_look_for")).to_string(),
///     "(?P<group_name>pattern_to_look_for)"
/// );
/// assert_eq!(
///     &NamedGroup::new("pobox", Raw(r"PO BOX \d{3:5}")).to_string(),
///     "(?P<pobox>PO BOX \\d{3:5})"
/// );
/// ```
#[derive(Clone)]
pub struct NamedGroup<'a> {
    name: &'a str,
    regexes: Box<ReadableRe<'a>>,
}

impl<'a> NamedGroup<'a> {
    pub fn new(name: &'a str, re: ReadableRe<'a>) -> Self {
        Self {
            name,
            regexes: Box::new(re),
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
/// use readable_regex::solvers::{Concat, NonCaptureGroup};
/// use readable_regex::ReadableRe;
/// use std::fmt::Display;
/// let query = ReadableRe::NonCaptureGroup(
///     NonCaptureGroup::new(ReadableRe::Raw("pattern_to_look_for"))
/// );
/// assert_eq!(
///     query.to_string(),
///     "(?:pattern_to_look_for)"
/// );
/// ```
#[derive(Clone)]
pub struct NonCaptureGroup<'a>(Box<ReadableRe<'a>>);

impl<'a> NonCaptureGroup<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for NonCaptureGroup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?:{})", self.0)
    }
}

impl_builder_from_iter!(NonCaptureGroup);

/// Regex syntax for an optional part of the pattern of the regex strings in tuple_of_regex_strs
///
/// ## Example
/// ```
/// use readable_regex::solvers::Optional;
/// use readable_regex::ReadableRe::{self, Raw};
/// let query = ReadableRe::Optional(Optional::new(Raw("foo")));
/// assert_eq!(
///     query.to_string(),
///     "foo?"
/// );
/// ```
#[derive(Clone)]
pub struct Optional<'a>(Box<ReadableRe<'a>>);

impl<'a> Optional<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for Optional<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}?", self.0)
    }
}

impl_builder_from_iter!(Optional);

/// Regex syntax for the alternation or "or" operator of the patterns in iterator input,
/// and the alternation is placed in a group
///
/// ## Examples
///
/// ```
/// use readable_regex::solvers::Either;
/// use readable_regex::ReadableRe::Raw;
/// assert_eq!(Either::new([Raw("a"), Raw("b"), Raw("c")]).to_string(), "a|b|c")
/// ```
#[derive(Clone)]
pub struct Either<'a>(Concat<'a>);

impl<'a> Either<'a> {
    pub fn new(iter: impl IntoIterator<Item = ReadableRe<'a>>) -> Self {
        Self::from_iter(iter)
    }
}

impl<'a> Display for Either<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromIterator<ReadableRe<'a>> for Either<'a> {
    fn from_iter<T: IntoIterator<Item = ReadableRe<'a>>>(iter: T) -> Self {
        let iter = iter
            .into_iter()
            .flat_map(|re| [re, ReadableRe::Raw("|")].into_iter());
        // TODO: substitute with `interleave` when available
        let mut concat = Concat::from_iter(iter);
        concat.0.pop();
        Self(concat)
    }
}

/// Regex syntax for matching an exact number of occurrences of the input regex
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::Exactly;
/// use readable_regex::ReadableRe::Raw;
/// let query = Exactly::new(3, Raw("A"));
/// assert_eq!(query.to_string(), "A{3}")
/// ```
#[derive(Clone)]
pub struct Exactly<'a> {
    quantity: usize,
    re: Box<ReadableRe<'a>>,
}

impl<'a> Exactly<'a> {
    pub fn new(quantity: usize, re: ReadableRe<'a>) -> Self {
        Self {
            quantity,
            re: Box::new(re),
        }
    }
}

impl<'a> Display for Exactly<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{}}}", self.re, self.quantity)
    }
}

/// Regex syntax for matching an between the minimum and maximum number of occurrences of the input regex
/// It accepts unbounded ranges. It also do not discriminate between open/closed ranges, and both
/// works as including both ends.
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::Ranged;
/// use readable_regex::ReadableRe::Raw;
/// let query = Ranged::new(3..5, Raw("abc"));
/// assert_eq!(query.to_string(), "abc{3,5}");
/// let query = Ranged::new(..5, Raw("abc"));
/// assert_eq!(query.to_string(), "abc{,5}");
/// let query = Ranged::new(3.., Raw("abc"));
/// assert_eq!(query.to_string(), "abc{3,}");
/// let query = Ranged::new(.., Raw("abc"));
/// assert_eq!(query.to_string(), "abc{,}");
/// ```
pub struct Ranged<'a> {
    range: (Bound<usize>, Bound<usize>),
    re: Box<ReadableRe<'a>>,
}

impl<'a> Clone for Ranged<'a> {
    fn clone(&self) -> Self {
        Self {
            range: self.range,
            re: self.re.clone(),
        }
    }
}

impl<'a> Ranged<'a> {
    pub fn new<R>(range: R, re: ReadableRe<'a>) -> Self
    where
        R: RangeBounds<usize> + 'static,
    {
        Self {
            range: (range.start_bound().cloned(), range.end_bound().cloned()),
            re: Box::new(re),
        }
    }
}

impl<'a> Display for Ranged<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min, max) = match self.range {
            (
                Bound::Included(min) | Bound::Excluded(min),
                Bound::Included(max) | Bound::Excluded(max),
            ) => (min.to_string(), max.to_string()),
            (Bound::Included(min) | Bound::Excluded(min), Bound::Unbounded) => {
                (min.to_string(), "".to_string())
            }
            (Bound::Unbounded, Bound::Included(max) | Bound::Excluded(max)) => {
                ("".to_string(), max.to_string())
            }
            (Bound::Unbounded, Bound::Unbounded) => ("".to_string(), "".to_string()),
        };
        write!(f, "{}{{{},{}}}", self.re, min, max)
    }
}

/// Regex syntax for matching zero or more occurrences of the input regex.
/// This does a greedy match, which tries to make the largest match possible.
///
/// ## Example
///
/// ```
/// use readable_regex::ReadableRe::Raw;
/// use readable_regex::solvers::{ZeroOrMore};
/// let query = ZeroOrMore::new(Raw("abc"));
/// assert_eq!(query.to_string(), "abc*")
/// ```
#[derive(Clone)]
pub struct ZeroOrMore<'a>(Box<ReadableRe<'a>>);

impl<'a> ZeroOrMore<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for ZeroOrMore<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*", self.0)
    }
}

impl_builder_from_iter!(ZeroOrMore);

/// Regex syntax for matching zero or more occurrences of the input regex.
/// This does a lazy match, which tries to make the smallest match possible.
///
/// ## Example
///
/// ```
/// use readable_regex::ReadableRe::Raw;
/// use readable_regex::solvers::{ZeroOrMoreLazy};
/// let query = ZeroOrMoreLazy::new(Raw("abc"));
/// assert_eq!(query.to_string(), "abc*?")
/// ```
#[derive(Clone)]
pub struct ZeroOrMoreLazy<'a>(Box<ReadableRe<'a>>);

impl<'a> ZeroOrMoreLazy<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for ZeroOrMoreLazy<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*?", self.0)
    }
}

impl_builder_from_iter!(ZeroOrMoreLazy);

/// Regex syntax for matching one or more occurrences of the input regex.
/// This does a greedy match, which tries to make the largest match possible.
///
/// ## Example
///
/// ```
/// use readable_regex::ReadableRe::Raw;
/// use readable_regex::solvers::{OneOrMore};
/// let query = OneOrMore::new(Raw("abc"));
/// assert_eq!(query.to_string(), "abc+")
/// ```
#[derive(Clone)]
pub struct OneOrMore<'a>(Box<ReadableRe<'a>>);

impl<'a> OneOrMore<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for OneOrMore<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+", self.0)
    }
}

impl_builder_from_iter!(OneOrMore);

/// Regex syntax for matching one or more occurrences of the input regex.
/// This does a lazy match, which tries to make the smallest match possible.
///
/// ## Example
///
/// ```
/// use readable_regex::ReadableRe::Raw;
/// use readable_regex::solvers::{OneOrMoreLazy};
/// let query = OneOrMoreLazy::new(Raw("abc"));
/// assert_eq!(query.to_string(), "abc+?")
/// ```
#[derive(Clone)]
pub struct OneOrMoreLazy<'a>(Box<ReadableRe<'a>>);

impl<'a> OneOrMoreLazy<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for OneOrMoreLazy<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+?", self.0)
    }
}

impl_builder_from_iter!(OneOrMoreLazy);

/// Regex syntax for matching the pattern of the input regex at the start of the searched text.
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::StartsWith;
/// use readable_regex::ReadableRe::Raw;
/// let query = StartsWith::new(Raw("abc"));
/// assert_eq!(query.to_string(), "^abc");
/// ```
#[derive(Clone)]
pub struct StartsWith<'a>(Box<ReadableRe<'a>>);

impl<'a> StartsWith<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for StartsWith<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "^{}", self.0)
    }
}

impl_builder_from_iter!(StartsWith);

/// Regex syntax for matching the pattern of the input regex at the end of the searched text.
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::EndsWith;
/// use readable_regex::ReadableRe::Raw;
/// let query = EndsWith::new(Raw("abc"));
/// assert_eq!(query.to_string(), "abc$");
/// ```
#[derive(Clone)]
pub struct EndsWith<'a>(Box<ReadableRe<'a>>);

impl<'a> EndsWith<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

impl<'a> Display for EndsWith<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}$", self.0)
    }
}

impl_builder_from_iter!(EndsWith);

/// Regex syntax for matching the pattern of the input regex at the start and end of the searched text.
/// (That is, the pattern must match the complete searched text.)
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::StartsAndEndsWith;
/// use readable_regex::ReadableRe::Raw;
/// let query =  StartsAndEndsWith::new(Raw("abc"));
/// assert_eq!(query.to_string(), "^abc$");
/// ```
#[derive(Clone)]
pub struct StartsAndEndsWith<'a>(Box<ReadableRe<'a>>);

impl<'a> StartsAndEndsWith<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(ReadableRe::StartsWith(StartsWith::new(
            ReadableRe::EndsWith(EndsWith::new(re)),
        ))))
    }
}

impl<'a> Display for StartsAndEndsWith<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_builder_from_iter!(StartsAndEndsWith);

/// Regex syntax for a character class including the characters in the input [`IntoIterator`]
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::Chars;
/// let query = Chars::new("abc".chars());
/// assert_eq!(query.to_string(), "[abc]");
/// ```
#[derive(Clone)]
pub struct Chars(String);

impl Chars {
    pub fn new(iter: impl IntoIterator<Item = char>) -> Self {
        Self::from_iter(iter)
    }
}

impl Display for Chars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl FromIterator<char> for Chars {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Regex syntax for a character class excluding the characters in the input [`IntoIterator`]
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::NotChars;
/// let query = NotChars::new("abc".chars());
/// assert_eq!(query.to_string(), "[^abc]");
/// ```
#[derive(Clone)]
pub struct NotChars(String);

impl NotChars {
    pub fn new(iter: impl IntoIterator<Item = char>) -> Self {
        Self::from_iter(iter)
    }
}

impl Display for NotChars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[^{}]", self.0)
    }
}

impl FromIterator<char> for NotChars {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(feature = "re-fancy")]
/// Regex syntax for an atomic group of the input regex
///
/// ## Example
///
/// ```
/// use readable_regex::solvers::AtomicGroup;
/// use readable_regex::ReadableRe::Raw;
/// let query = AtomicGroup::new(Raw("foo"));
/// assert_eq!(query.to_string(), "(?>foo)")
/// ```
#[derive(Clone)]
pub struct AtomicGroup<'a>(Box<ReadableRe<'a>>);

#[cfg(feature = "re-fancy")]
impl<'a> AtomicGroup<'a> {
    pub fn new(re: ReadableRe<'a>) -> Self {
        Self(Box::new(re))
    }
}

#[cfg(feature = "re-fancy")]
impl<'a> Display for AtomicGroup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?>{})", self.0)
    }
}

#[cfg(feature = "re-fancy")]
impl_builder_from_iter!(AtomicGroup);
