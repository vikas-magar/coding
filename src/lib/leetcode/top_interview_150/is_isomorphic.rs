use std::collections::HashMap;

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
        map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_function() {}
}
