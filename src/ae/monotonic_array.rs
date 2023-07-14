use std::vec;

/* Monotonic array time: O(n) | space: O(1) */
fn monotonic_array(v: &Vec<i32>) -> bool {
    let mut is_incremental: bool = false;

    let end = v.len();
    let mut start: usize = 0;
    for _item in v.iter() {
        if *_item < v[start + 1] && start < end {
            is_incremental = true
        }
        if !is_incremental {
            break;
        }
        start += 1;
    }
    return !is_incremental;
}

pub fn run() {
    let mut v = vec![-1, -5, -10, -1100, -1100, -1101, -1102, -9001];
    let expected = true;
    assert_eq!(expected, monotonic_array(&mut v));
}

#[test]
fn validate() {
    let mut v = vec![-1, -5, -10, -1100, -1100, -1101, -1102, -9001];
    let expected = true;
    assert_eq!(expected, monotonic_array(&mut v));
}
