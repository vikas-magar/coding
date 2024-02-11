use std::vec;

pub fn run() {
    println!("LC | Move Target Element to End of the List");
    let mut v = vec![2, 1, 2, 2, 2, 3, 4, 2];
    let te: i32 = 2;
    move_element_end(&mut v, te);
    println!("{:?}", v)
}

/* time: O(n) | space: O(1) */
fn move_element_end(v: &mut Vec<i32>, t: i32) {
    let mut start = 0;
    let mut end = v.len() - 1;

    while start <= end {
        if v[start] == t {
            while v[end] == t && end > start {
                end -= 1;
            }
            let tt = v[start];
            v[start] = v[end];
            v[end] = tt;
        }
        start += 1;
    }
}

#[test]
fn validate() {
    let mut v = vec![2, 1, 2, 2, 2, 3, 4, 2];
    let te: i32 = 2;
    let opt = vec![4, 1, 3, 2, 2, 2, 2, 2];
    move_element_end(&mut v, te);
    let out = v.iter().zip(&opt).filter(|&(a, b)| a == b).count();
    assert_eq!(v.len(), out);
}
