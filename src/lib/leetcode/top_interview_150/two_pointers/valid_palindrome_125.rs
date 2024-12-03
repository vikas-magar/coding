// https://leetcode.com/problems/valid-palindrome/description/?envType=study-plan-v2&envId=top-interview-150

#[warn(dead_code)]
struct Solution;

impl Solution {
    fn is_palindrome(s: String) -> bool {
        let fs = s
            .chars()
            .filter(|x| x.is_alphanumeric())
            .map(|x| x.to_lowercase().to_string());
        let bw = s
            .chars()
            .filter(|x| x.is_alphanumeric())
            .rev()
            .map(|c| c.to_lowercase().to_string());

        fs.partial_cmp(bw) == Some(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
    }
}
