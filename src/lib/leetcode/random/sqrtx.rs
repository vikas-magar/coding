use std::cmp;

use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // first find k such that k * k  > x
        let mut left = 0;
        let mut right = cmp::min(x, 46340);

        while left < right {
            let mid = left + (right - left) / 2;
            if (mid * mid) > x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
