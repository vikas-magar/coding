use crate::Solution;

impl Solution {
    pub fn length_of_last_word_one(s: String) -> i32 {
        let mut count = 0;
        match Some(s) {
            Some(s) => {
                for ch in s.trim().chars().rev() {
                    if ch != ' ' {
                        count += 1;
                    }
                    if ch == ' ' {
                        break;
                    }
                }
            }
            None => {
                count;
            }
        }
        count as i32
    }

    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        assert_eq!(
            Solution::length_of_last_word_one("Hello World".to_string()),
            5
        );

        assert_eq!(
            Solution::length_of_last_word("Welcome to India".to_string()),
            5
        );
    }
}
