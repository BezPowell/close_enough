use super::Diff;

impl Diff for isize {
    type Output = usize;

    fn diff(&self, other: &Self) -> usize {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: isize = 29;
        let b: isize = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: isize = isize::MAX;
        let b: isize = isize::MIN;

        assert_eq!(a.diff(&b), usize::MAX);
        assert_eq!(b.diff(&a), usize::MAX);
    }

    #[test]
    fn diff_mixed() {
        let a: isize = 67;
        let b: isize = -123;

        assert_eq!(a.diff(&b), 190);
        assert_eq!(b.diff(&a), 190);
    }

    #[test]
    fn sort() {
        let mut values: [isize; 5] = [-3, -1, 1, 4, 5];
        values.sort_by(|a, b| a.diff(&0).cmp(&b.diff(&0)));

        assert_eq!(values, [-1, 1, -3, 4, 5]);
    }
}
