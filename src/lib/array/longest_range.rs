use std::collections::HashMap;

// Time: O(n) | Space: O(n)
pub fn run(nums: Vec<i32>) -> i32 {
    let mut store: HashMap<&i32, bool> = HashMap::new();

    for ele in nums.iter() {
        store.insert(ele, true);
    }
    let mut max = 0;
    for ele in nums.iter() {
        let mut tmp = ele.clone();
        let mut cnt = 0;
        // Up
        while store.contains_key(&tmp) {
            cnt += 1;
            tmp += 1;
        }

        //Down
        tmp = ele - 1;
        while store.contains_key(&tmp) && tmp >= 0 {
            cnt += 1;
            tmp -= 1;
        }
        if max < cnt {
            max = cnt;
        }
    }
    max
}
