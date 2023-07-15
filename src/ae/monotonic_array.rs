use std::vec;

/* Monotonic array time: O(n) | space: O(1) */
fn monotonic_array(v: Vec<i32>) -> bool {
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
/* Monotonic Array, Time: O(n) | Space: O(1)*/
fn is_monotonic(v: &Vec<i32>) -> bool {
    use std::cmp::Ordering;
    let mut d = 0;
    for (a, b) in v.iter().zip(v[1..].iter()) {
        d |= match a.cmp(b) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => 2,
        };
        if d == 3 {
            return false;
        }
    }
    return true;
}

pub fn run() {
    let v = vec![-1, -5, -10, -1100, -1100, -1101, -1102, -9001];
    let expected = true;
    assert_eq!(expected, is_monotonic(&v));
}

#[test]
fn validate() {
    let v = vec![-1, -5, -10, -1100, -1100, -1101, -1102, -9001];
    let expected = true;
    assert_eq!(expected, is_monotonic(&v));
}
