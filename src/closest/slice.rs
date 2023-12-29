use crate::diff::Diff;

use super::Closest;

impl<T: Diff> Closest<T> for [T] {
    type Value = T;

    fn closest(&self, to: &T) -> Option<&Self::Value> {
        let mut sorted: Vec<&T> = self.iter().collect();
        sorted.sort_by(|a, b| a.diff(to).cmp(&b.diff(to)));

        sorted.first().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::closest::Closest;

    #[test]
    fn closest_unsigned() {
        let values: [usize; 5] = [0, 1, 3, 5, 9];

        assert_eq!(values.closest(&2), Some(&1));
    }

    #[test]
    fn closest_signed() {
        let values: [isize; 5] = [-3, -1, 1, 4, 5];

        assert_eq!(values.closest(&0), Some(&-1));
    }
}
