use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut store = magazine.chars().fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e).or_insert(0) += 1;
            acc
        });

        for ele in ransom_note.chars() {
            if !store.contains_key(&ele) {
                return false;
            }
            store.entry(ele).and_modify(|x| *x -= 1);

            match store.get_key_value(&ele) {
                Some(val) => {
                    if *val.1 < 0 {
                        return false;
                    }
                }
                None => return true,
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
        let w1 = "aa".to_string();
        let w2 = "aab".to_string();
        assert!(Solution::can_construct(w1, w2));
    }
}
