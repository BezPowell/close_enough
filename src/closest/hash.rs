use super::Closest;
use crate::diff::Diff;
use std::{collections::HashMap, hash::Hash};

impl<T: Diff + Hash + Eq, V> Closest<T> for HashMap<T, V> {
    type Value = V;

    fn closest(&self, to: &T) -> Option<&Self::Value> {
        let mut keys: Vec<&T> = self.keys().into_iter().collect();
        keys.sort_by(|a, b| a.diff(to).cmp(&b.diff(to)));

        match keys.first() {
            Some(key) => self.get(key),
            None => None,
        }
    }
}

mod tests {
    use crate::closest::Closest;
    use std::collections::HashMap;

    #[test]
    fn closest() {
        let mut values: HashMap<usize, String> = HashMap::new();
        values.insert(1940, "Nineteen Forty".to_string());
        values.insert(1950, "Nineteen Fifty".to_string());
        values.insert(1961, "Nineteen Sixty One".to_string());
        values.insert(1970, "Nineteen Seventy".to_string());

        assert_eq!(values.closest(&1955), Some(&"Nineteen Fifty".to_string()));
    }
}
