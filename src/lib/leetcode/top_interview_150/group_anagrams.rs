use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut store: HashMap<String, Vec<String>> = HashMap::new();
        for ele in strs.iter() {
            let l = sort_string(&ele);
            store
                .entry(l)
                .and_modify(|e| e.push(ele.clone()))
                .or_insert(vec![ele.clone()]);
        }
        store.into_values().collect()
    }
}
fn sort_string(input: &str) -> String {
    let mut vector: Vec<char> = input.chars().collect();
    vector.sort();
    vector.into_iter().collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_function() {
        let _inp = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    }
}
