use std::{char, collections::HashMap};

use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut store: HashMap<char, usize> = HashMap::with_capacity(s.len());
        for ch in s.chars() {
            store.entry(ch).and_modify(|x| *x += 1).or_insert(1);
        }
        for c in t.chars() {
            if store.contains_key(&c) && store.get(&c).unwrap() > &0 {
                store.entry(c).and_modify(|x| *x -= 1);
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_function() {
        let t = "anagram".to_string();
        let g = "nagaram".to_string();
        assert_eq!(Solution::is_anagram(g, t), true);
    }
}
