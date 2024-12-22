use crate::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut current = s_chars.next();

        for t_c in t.chars() {
            if let Some(c) = current {
                if c == t_c {
                    current = s_chars.next();
                }
            }
        }

        current.is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ))
    }
}
