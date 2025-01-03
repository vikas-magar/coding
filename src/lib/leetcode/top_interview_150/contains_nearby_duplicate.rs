use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut store = HashMap::with_capacity(nums.len());
        for (i, val) in nums.iter().enumerate() {
            if let Some(j) = store.insert(val, i) {
                if (i as i32) - (j as i32) <= k {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_function() {
        let inp = vec![1, 2, 3, 1, 2, 3];
        assert_eq!(Solution::contains_nearby_duplicate(inp, 2), false);
        let inp = vec![1, 0, 1, 1];
        assert_eq!(Solution::contains_nearby_duplicate(inp, 1), true);
    }
}
