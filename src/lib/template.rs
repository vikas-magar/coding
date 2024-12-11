use crate::Solution;

impl Solution {
    pub fn function(s: String) -> String {
        return s;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        assert_eq!(
            Solution::function("Hello World".to_string()),
            "Hello World".to_string()
        );
    }
}
