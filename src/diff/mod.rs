mod i128;
mod i16;
mod i32;
mod i64;
mod i8;
mod isize;
mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod usize;

pub trait Diff {
    /// The output of a diff calculation.
    type Output: Ord;

    /// Return the difference between self and other,
    /// as a type that implements Ord for sorting.
    /// ```
    /// use close_enough::Diff;
    /// 
    /// let a: u32 = 29;
    /// let b: u32 = 12;
    /// 
    /// assert_eq!(a.diff(&b), 17);
    /// assert_eq!(b.diff(&a), 17);
    /// ```
    fn diff(&self, other: &Self) -> Self::Output;
}
