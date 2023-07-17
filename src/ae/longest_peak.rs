use std::{collections::HashMap, i32, usize};

fn peak(a: &[i32]) -> i32 {
    println!("{:?}", a);
    let mut max = 0;
    let mut store: HashMap<usize, &i32> = HashMap::new();
    let start = 0;
    let end = a.len() - 1;
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
        let mut cnt = 0;
        //down
        kk = *k - 1;
        while kk >= start && a[kk] < **v {
            cnt += 1;
            if kk == 0 {
                break;
            }
            kk = kk - 1;
        }
        // up
        kk = *k + 1;
        while a[kk] > **v && kk <= end {
            cnt += 1;
            kk = kk + 1;
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

pub fn run() {
    let a: [i32; 13] = [1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
    peak(&a);
}
