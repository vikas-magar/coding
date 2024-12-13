// https://leetcode.com/problems/valid-palindrome/description/?envType=study-plan-v2&envId=top-interview-150

use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let st: Vec<_> = s
            .chars()
            .filter_map(|c| c.is_alphanumeric().then(|| c.to_ascii_lowercase()))
            .collect();
        st.iter().eq(st.iter().rev())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_one() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }
    #[test]
    fn test_two() {
        println!("welcome to indai");
    }
}
