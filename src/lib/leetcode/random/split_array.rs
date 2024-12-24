use crate::Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = nums.iter().max().cloned().unwrap();
        let mut right = nums.iter().sum();
        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::feasible(&nums, mid, k) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }

    pub fn feasible(nums: &Vec<i32>, mid: i32, k: i32) -> bool {
        let mut count = 1;
        let mut total = 0;

        for num in nums {
            total += num;
            if total > mid {
                total = *num;
                count += 1
            }
            if count > k {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_function() {
        let input = vec![7, 2, 5, 10, 8];
        assert_eq!(Solution::split_array(input, 2), 18);
    }
}
