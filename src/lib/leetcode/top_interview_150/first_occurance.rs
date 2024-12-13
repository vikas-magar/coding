use crate::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }
        let inp_len = haystack.len();
        let sub_len = needle.len();
        let window = inp_len - sub_len + 1;
        for c in 0..window {
            let s1 = &haystack[c..c + sub_len];
            if needle == s1 {
                return c as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let indiex = Solution::str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(indiex, 0);
    }
    #[test]
    fn test_empty() {
        let index = Solution::str_str("".to_string(), "".to_string());
        assert_eq!(index, 0)
    }
    #[test]
    fn test_mote_len() {
        let index = Solution::str_str("".to_string(), " ".to_string());
        assert_eq!(index, -1)
    }
    #[test]
    fn test_two() {
        let index = Solution::str_str("hello".to_string(), "ll".to_string());
        assert_eq!(index, 2)
    }
}
