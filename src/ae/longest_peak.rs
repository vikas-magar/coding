use std::{collections::HashMap, i32, usize};

fn peak(a: &[i32]) -> i32 {
    let mut max = 0;
    let mut store: HashMap<usize, &i32> = HashMap::new();
    let start = 0;
    let end = a.len() - 1;
    if a.len() < 2 {
        return 0;
    }
    for (key, item) in a.iter().enumerate() {
        if key != start && key != end {
            if a[key - 1] < *item && *item > a[key + 1] {
                store.insert(key, item);
            }
        }
    }
    // down
    for (k, v) in store.iter() {
        let mut kk;
        let mut cnt = 1;
        let mut left;
        let mut right;
        //down
        kk = *k - 1;
        left = a[kk];
        right = **v;
        while kk >= start && left < right {
            cnt += 1;
            if kk == 0 {
                break;
            }
            kk = kk - 1;
            right = left;
            left = a[kk];
        }
        // up
        kk = *k + 1;
        left = a[kk];
        right = **v;
        println!("{:?}", a);
        while left < right && kk <= end {
            println!("left : {}", left);
            println!("right: {}", right);
            println!("kk: {}", kk);
            cnt += 1;
            kk = kk + 1;
            if kk > end {
                break;
            }
            right = left;
            left = a[kk];
        }
        if cnt > max {
            max = cnt;
        }
    }
    return max;
}
#[test]
fn validate() {
    let a: [i32; 13] = [1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
    assert_eq!(6, peak(&a));
}

#[test]
fn validate2() {
    let a: [i32; 7] = [2, 1, 4, 7, 3, 2, 5];
    assert_eq!(5, peak(&a));
}

#[test]
fn validate3() {
    let a: [i32; 11] = [0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0];
    assert_eq!(11, peak(&a));
}

pub fn run() {
    let a: [i32; 13] = [1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
    peak(&a);
}
