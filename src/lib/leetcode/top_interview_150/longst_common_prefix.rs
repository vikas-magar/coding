use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs.get(0).unwrap().clone();
        for st in strs.iter().skip(1) {
            loop {
                if st.starts_with(prefix.as_str()) {
                    break;
                } else {
                    prefix.pop();
                    if prefix.len() == 0 {
                        break;
                    }
                }
            }
        }
        return prefix;
    }

    pub fn longest_common_prefix_sol1(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, e| {
                acc.chars()
                    .zip(e.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_function() {
        let inp: Vec<String> = vec!["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            Solution::longest_common_prefix(inp.clone()),
            "fl".to_string()
        );
        assert_eq!(Solution::longest_common_prefix_sol1(inp), "fl".to_string());
    }
}
