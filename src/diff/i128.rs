use super::Diff;

impl Diff for i128 {
    type Output = u128;

    fn diff(&self, other: &Self) -> u128 {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: i128 = 29;
        let b: i128 = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: i128 = i128::MAX;
        let b: i128 = i128::MIN;

        assert_eq!(a.diff(&b), u128::MAX);
        assert_eq!(b.diff(&a), u128::MAX);
    }

    #[test]
    fn sort() {
        let mut values: [i128; 5] = [0, 1, 3, 5, 9];
        values.sort_by(|a, b| a.diff(&2).cmp(&b.diff(&2)));

        assert_eq!(values, [1, 3, 0, 5, 9]);
    }
}
