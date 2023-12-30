use super::Diff;

impl Diff for i64 {
    type Output = u64;

    fn diff(&self, other: &Self) -> u64 {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: i64 = 29;
        let b: i64 = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: i64 = i64::MAX;
        let b: i64 = i64::MIN;

        assert_eq!(a.diff(&b), u64::MAX);
        assert_eq!(b.diff(&a), u64::MAX);
    }

    #[test]
    fn sort() {
        let mut values: [i64; 5] = [0, 1, 3, 5, 9];
        values.sort_by(|a, b| a.diff(&2).cmp(&b.diff(&2)));

        assert_eq!(values, [1, 3, 0, 5, 9]);
    }
}
