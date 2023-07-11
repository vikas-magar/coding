pub fn run() {
    println!("AE | Move Target Element to End of the List");
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
