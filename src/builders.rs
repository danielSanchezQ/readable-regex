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
/// let scaped = Scape("!#$%&");
/// assert_eq!(scaped.to_string(), "!\\#\\$%\\&");
/// ```
pub struct Scape<'a>(pub &'a str);

impl Display for Scape<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", regex::escape(self.0))
    }
}

/// Regex syntax for a regex group surrounded by parentheses of the regex input str
/// ### Example
/// ```
/// use readable_regex::builders::Group;
/// assert_eq!(Group(vec!["cat"]).to_string(), "(cat)");
/// assert_eq!(Group(vec!["cat", "dog", "moose"]).to_string(), "(catdogmoose)");
/// assert_eq!(Group(vec!["cat", &Group(vec!["dog"]).to_string(), &Group(vec!["moose"]).to_string()]).to_string(), "(cat(dog)(moose))");
/// ```
pub struct Group<'a>(pub Vec<&'a str>);

impl Display for Group<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for s in &self.0 {
            write!(f, "{}", s)?;
        }
        write!(f, ")")
    }
}
