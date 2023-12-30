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
    type Output: Ord;

    fn diff(&self, other: &Self) -> Self::Output;
}
