mod hash;
mod slice;

use crate::diff::Diff;

pub trait Closest<T: Diff> {
    /// The output of closest().
    type Value;

    /// Return the closest match to a given search key.
    /// ```
    /// use close_enough::Closest;
    /// 
    /// let values: [usize; 5] = [0, 1, 3, 5, 9];
    /// assert_eq!(values.closest(&2), Some(&1));
    /// ```
    fn closest(&self, to: &T) -> Option<&Self::Value>;
}
