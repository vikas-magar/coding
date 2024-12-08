use core::num;
use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut store = HashMap::new();
        for &i in &nums {
            let counter = store.entry(i).or_insert(0);
            *counter += 1;
        }

        let (max_key, max_count) = store.iter().max_by_key(|&(_, count)| count).unwrap();

        *max_key as i32
    }

    pub fn major_ele_by_boyer_moore(nums: Vec<i32>) -> i32 {
        let mut candidate = None;
        let mut count = 0;
        for &i in &nums {
            if count == 0 {
                candidate = Some(i);
            }
            if candidate == Some(i) {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut nums = vec![0, 1, 2, 2, 3, 2, 2, 2];
        assert_eq!(Solution::majority_element(nums), 2);
        let mut nums = vec![1, 1, 2, 2, 3, 2, 2, 2];
        assert_eq!(Solution::major_ele_by_boyer_moore(nums), 2);
    }
}