pub fn sd() {
    let mut a = vec![-1, 5, 10, 20, 28, 3];
    let mut b = vec![26, 134, 135, 15, 17];
    println!("{:?}", cal_diff(&mut a, &mut b));
}

/* Time: O(nlog(n)) | Space: O(1)*/
fn cal_diff(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    a.sort();
    b.sort();
    let mut min_diff = i32::MAX;
    let mut curr_diff;
    let mut num1;
    let mut num2;
    let mut s_pair = vec![0, 0];
    let mut ptr1 = 0;
    let mut ptr2 = 0;

    while ptr1 < a.len() && ptr2 < b.len() {
        num1 = a[ptr1];
        num2 = b[ptr2];
        curr_diff = i32::abs(num1 - num2);

        if num1 < num2 {
            ptr1 += 1;
        } else if num2 < num1 {
            ptr2 += 1;
        } else {
            return vec![num1, num2];
        }
        if min_diff > curr_diff {
            min_diff = curr_diff;
            s_pair[0] = num1;
            s_pair[1] = num2;
        }
    }
    return s_pair;
}
