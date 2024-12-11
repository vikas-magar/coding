use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let ln = nums.len();
        let mut store: HashMap<i32, usize> = HashMap::with_capacity(ln);
        for (idx, num) in nums.into_iter().enumerate() {
            let m = target - num;
            match store.get(&m) {
                Some(&m) => {
                    return vec![m as i32, idx as i32];
                }
                None => {
                    store.insert(num, idx);
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        assert_eq!(Solution::two_sum(vec![1, 2, 6, 4], 3).len(), 2);
    }
}
