use crate::Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut nums = nums.clone();
        for i in queries {
            nums[i[0] as usize] -= 1;
            nums[i[1] as usize] -= 1;
        }
        nums.iter().any(|x| *x > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let nums = vec![2];
        let queries = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::is_zero_array(nums, queries), false);
    }
}
