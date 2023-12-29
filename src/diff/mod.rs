mod isize;
mod usize;

pub trait Diff {
    type Output: Ord;

    fn diff(&self, other: &Self) -> Self::Output;
}
