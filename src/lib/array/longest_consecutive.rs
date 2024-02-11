use std::collections::HashSet;
pub fn run(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<_> = nums.into_iter().collect();
    let mut ans = 0;
    for &ele in &num_set {
        if !num_set.contains(&(ele - 1)) {
            let count = (ele..).take_while(|x| num_set.contains(x)).count();
            ans = ans.max(count);
        }
    }
    ans as i32
}
