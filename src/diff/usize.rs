use super::Diff;

impl Diff for usize {
    type Output = Self;

    fn diff(&self, other: &Self) -> usize {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: usize = 29;
        let b: usize = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: usize = usize::MAX;
        let b: usize = usize::MIN;

        assert_eq!(a.diff(&b), usize::MAX);
        assert_eq!(b.diff(&a), usize::MAX);
    }

    #[test]
    fn sort() {
        let mut values: [usize; 5] = [0, 1, 3, 5, 9];
        values.sort_by(|a, b| a.diff(&2).cmp(&b.diff(&2)));

        assert_eq!(values, [1, 3, 0, 5, 9]);
    }
}
