use crate::Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = std::collections::HashSet::new();
        let mut t = n.clone();
        while t != 1 && !seen.contains(&t) {
            seen.insert(t);
            t = process_num(t)
        }
        return t == 1;
    }
}

fn process_num(n: i32) -> i32 {
    let mut nn = n.clone();
    nn.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as i32 * x as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_function() {
        assert!(Solution::is_happy(19));
        assert!(Solution::is_happy(1));
    }
}
