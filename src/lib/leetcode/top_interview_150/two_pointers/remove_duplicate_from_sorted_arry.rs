/* https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150 */

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => 0,
            false => {
                let mut prev = 0;
                for i in 1..nums.len() {
                    if nums[prev] != nums[i] {
                        prev += 1;
                        nums[prev] = nums[i];
                    }
                }
                (prev + 1) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_() {
        let mut lst = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(
            Solution::remove_duplicates(&mut lst) as usize,
            expected.len()
        );
    }
}
