use super::Diff;

impl Diff for u16 {
    type Output = Self;

    fn diff(&self, other: &Self) -> u16 {
        self.abs_diff(*other)
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::Diff;

    #[test]
    fn diff() {
        let a: u16 = 29;
        let b: u16 = 12;

        assert_eq!(a.diff(&b), 17);
        assert_eq!(b.diff(&a), 17);
    }

    #[test]
    fn diff_max() {
        let a: u16 = u16::MAX;
        let b: u16 = u16::MIN;

        assert_eq!(a.diff(&b), u16::MAX);
        assert_eq!(b.diff(&a), u16::MAX);
    }

    #[test]
    fn sort() {
        let mut values: [u16; 5] = [0, 1, 3, 5, 9];
        values.sort_by(|a, b| a.diff(&2).cmp(&b.diff(&2)));

        assert_eq!(values, [1, 3, 0, 5, 9]);
    }
}
