use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let mut store = HashMap::with_capacity(s.len());
        for index in 0..s.len() {
            let val = store.entry(s[index]).or_insert(t[index]);
            if *val != t[index] {
                return false;
            }
        }
        store.values().collect::<HashSet<_>>().len() == store.keys().collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_function() {}
}
