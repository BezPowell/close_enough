use super::Diff;

impl Diff for i8 {
    type Output = u8;

    fn diff(&self, other: &Self) -> u8 {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: i8 = 29;
        let b: i8 = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: i8 = i8::MAX;
        let b: i8 = i8::MIN;

        assert_eq!(a.diff(&b), u8::MAX);
        assert_eq!(b.diff(&a), u8::MAX);
    }

    #[test]
    fn sort() {
        let mut values: [i8; 5] = [0, 1, 3, 5, 9];
        values.sort_by(|a, b| a.diff(&2).cmp(&b.diff(&2)));

        assert_eq!(values, [1, 3, 0, 5, 9]);
    }
}
