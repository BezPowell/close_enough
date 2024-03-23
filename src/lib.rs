//! Sometimes, in programming as in life, it's more useful
//! to return the closest match to a given search key than
//! it is to return no result at all. This crate provides a
//! (small) selection of traits and default implementations to
//! facilitate this.
//! 
//! Providing complex diffing algorithms is not a goal of this
//! crate, but simple utilities (for example finding the closest
//! integer in a set of values) are.
//! 
//! ## Simple example with usize:
//! ```
//! use close_enough::Closest;
//! 
//! let values: [usize; 5] = [0, 1, 3, 5, 9];
//! assert_eq!(values.closest(&2), Some(&1));
//! ```
//! 
//! ## More complex example with custom type:
//! ```
//! use close_enough::Closest;
//! use close_enough::Diff;
//! 
//! #[derive(Debug, Eq, PartialEq)]
//! struct LengthString(String);
//! 
//! impl Diff for LengthString {
//!     type Output = usize;
//! 
//!     fn diff(&self, other: &Self) -> usize {
//!         self.0.len().diff(&other.0.len())
//!     }
//! }
//! 
//! let values: [LengthString; 3] = [
//!     LengthString("a".to_string()),
//!     LengthString("bb".to_string()),
//!     LengthString("dddd".to_string()),
//! ];
//! assert_eq!(
//!     values.closest(&LengthString("ccc".to_string())),
//!     Some(&LengthString("bb".to_string()))
//! );
//! ```

mod closest;
mod diff;

pub use closest::Closest;
pub use diff::Diff;
