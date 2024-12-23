use crate::Solution;

impl Solution {
    pub fn first_bad_version(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::is_bad_version(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    pub fn is_bad_version(version: i32) -> bool {
        if version >= 4 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        assert_eq!(Solution::first_bad_version(5), 4);
    }
}
