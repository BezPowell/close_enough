pub mod hash;
pub mod slice;

use crate::diff::Diff;

pub trait Closest<T: Diff> {
    type Value;

    fn closest(&self, to: &T) -> Option<&Self::Value>;
}
