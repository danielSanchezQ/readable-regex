//! ## Regex for human beings.
//!
//! Regex is useful. But, sincerely, since code it is more read than written regex could
//! be more understandable in a verbose mode.
//!
//! `readable-regex` crate is a set of tools to build those regexes in a verbose way. Which aims
//! to improve readability of code.
//!
//! ### Examples
//!
//!
//! ### Features
//! * `re` => Use [`regex`] crate backend.
//! * `re-fancy` => Use [`fancy_regex`] crate backend and expands this crate functionality.

mod constants;
pub mod readable;
pub mod solvers;

pub use readable::ReadableRe;
