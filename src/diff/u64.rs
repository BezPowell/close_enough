use super::Diff;

impl Diff for u64 {
    type Output = Self;

    fn diff(&self, other: &Self) -> u64 {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: u64 = 29;
        let b: u64 = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: u64 = u64::MAX;
        let b: u64 = u64::MIN;

        assert_eq!(a.diff(&b), u64::MAX);
        assert_eq!(b.diff(&a), u64::MAX);
    }

    #[test]
    fn sort() {
        let mut values: [u64; 5] = [0, 1, 3, 5, 9];
        values.sort_by(|a, b| a.diff(&2).cmp(&b.diff(&2)));

        assert_eq!(values, [1, 3, 0, 5, 9]);
    }
}
